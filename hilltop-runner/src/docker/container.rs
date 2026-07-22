use std::path::{Path, PathBuf};

use tokio::io::AsyncReadExt;

use crate::api::apis::configuration::Configuration;
use crate::api::apis::container_api;
use crate::api::models::DeviceMapping;
use crate::api::models::{ContainerCreateRequest, HostConfig};
use crate::api::models::{Mount, MountType};

#[derive(Debug)]
pub struct Container {
    id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserMount {
    pub host_path: PathBuf,
    pub container_path: PathBuf,
    pub read_only: bool,
}

#[derive(Debug)]
pub struct LogLine {
    pub stream_type: StreamType,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamType {
    Stdin,
    Stdout,
    Stderr,
}

#[derive(Debug)]
pub struct UserDevice {
    pub dev_host_path: PathBuf,
    pub container_path: PathBuf,
}

impl Container {
    /// Create a new container from an image tag
    pub async fn create(
        api_engine_config: &Configuration,
        image_tag: &str,
        container_name: &str,
        user_mounts: &[UserMount],
        user_devices: &[UserDevice],
    ) -> anyhow::Result<Self> {
        let mut mounts = Vec::new();
        for user_mount in user_mounts {
            let mut mount = Mount::new();
            mount.source = Some(user_mount.host_path.to_str().unwrap().to_string());
            mount.target = Some(user_mount.container_path.to_str().unwrap().to_string());
            mount.read_only = Some(user_mount.read_only);
            mount.r#type = Some(MountType::Bind);
            mounts.push(mount);
        }

        let mut devices = Vec::new();
        for user_device in user_devices {
            devices.push(DeviceMapping {
                path_on_host: Some(user_device.dev_host_path.to_str().unwrap().to_string()),
                path_in_container: Some(user_device.container_path.to_str().unwrap().to_string()),
                cgroup_permissions: Some("rwm".to_string()),
            });
        }

        let mut host_config = HostConfig::new();
        host_config.mounts = Some(mounts);
        host_config.devices = Some(devices);

        let mut container_req = ContainerCreateRequest::new();
        container_req.image = Some(image_tag.to_string());
        container_req.host_config = Some(Box::new(host_config));

        let response = container_api::container_create(
            api_engine_config,
            container_req,
            Some(container_name),
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to create container: {}", e))?;

        tracing::info!("Created container: {}", response.id);
        Ok(Self { id: response.id })
    }

    /// Start the container
    pub async fn start(&self, api_engine_config: &Configuration) -> anyhow::Result<()> {
        tracing::info!("Starting container {}...", self.id);

        container_api::container_start(api_engine_config, &self.id, None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to start container: {}", e))?;

        tracing::info!("Container started");
        Ok(())
    }

    /// Stop the container
    pub async fn stop(&self, api_engine_config: &Configuration) -> anyhow::Result<()> {
        tracing::info!("Stopping container {}...", self.id);

        container_api::container_stop(api_engine_config, &self.id, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to stop container: {}", e))?;

        tracing::info!("Container stopped");
        Ok(())
    }

    /// Copy data from host to container
    pub async fn copy_from_host(
        &self,
        api_engine_config: &Configuration,
        file_data: &[u8],
        container_file_name: &str,
        container_path: &Path,
    ) -> anyhow::Result<()> {
        tracing::debug!(
            "Copying data to container {} at {}...",
            self.id,
            container_path.display()
        );

        let mut tar_builder = tar::Builder::new(Vec::new());
        let mut header = tar::Header::new_gnu();
        header.set_size(file_data.len() as u64);
        header.set_mode(0o644);
        header.set_mtime(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        );
        tar_builder
            .append_data(&mut header, container_file_name, file_data)
            .map_err(|e| anyhow::anyhow!("Failed to create tar: {}", e))?;

        let tar_of_data = tar_builder
            .into_inner()
            .map_err(|e| anyhow::anyhow!("Failed to finalize tar: {}", e))?;

        container_api::put_container_archive(
            api_engine_config,
            &self.id,
            container_path.to_str().unwrap(),
            tar_of_data,
            None,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to push to file to container: {:?}", e))?;

        tracing::debug!("Successfully copied data to container");
        Ok(())
    }

    /// Copy data from container to host
    ///
    /// Returns true if the file was successfully copied, false if the file did
    /// not exist in the container.
    pub async fn copy_to_host(
        &self,
        api_engine_config: &Configuration,
        container_path: &Path,
        host_path: &Path,
    ) -> anyhow::Result<bool> {
        tracing::debug!(
            "Copying data from container {} at {} to host {}...",
            self.id,
            container_path.display(),
            host_path.display()
        );

        let archive_result = container_api::container_archive(
            api_engine_config,
            &self.id,
            container_path.to_str().unwrap(),
        )
        .await;

        let tarball = match archive_result {
            Ok(tarball) => tarball,
            Err(crate::api::apis::Error::ResponseError(response))
                if response.status == reqwest::StatusCode::NOT_FOUND =>
            {
                return Ok(false);
            }
            Err(e) => {
                return Err(anyhow::anyhow!(
                    "Failed to get archive from container: {:?}",
                    e
                ));
            }
        };

        let mut tar = tar::Archive::new(std::io::Cursor::new(tarball));

        // Confirm tar contains only one element - and that is the file we requested
        let mut entries = tar.entries()?;
        let entry = entries
            .next()
            .ok_or_else(|| anyhow::anyhow!("Archive from container is empty"))??;

        let path_in_container = entry
            .path()
            .map_err(|e| anyhow::anyhow!("Could not read path from archive entry: {:?}", e))?;

        if path_in_container != container_path.file_name().unwrap() {
            return Err(anyhow::anyhow!(
                "Archive from container does not contain the requested file '{}' but instead got '{}'",
                container_path.file_name().unwrap().display(),
                path_in_container.display()
            ));
        }

        // Extract the file to the host path
        let mut output_file = std::fs::File::create(host_path)
            .map_err(|e| anyhow::anyhow!("Failed to create output file on host: {:?}", e))?;
        let mut entry_reader = entry;
        std::io::copy(&mut entry_reader, &mut output_file)
            .map_err(|e| anyhow::anyhow!("Failed to copy file data to host: {:?}", e))?;

        tracing::info!("Successfully copied data from container");
        Ok(true)
    }

    /// Get container logs
    pub async fn get_logs(
        &self,
        api_engine_config: &Configuration,
    ) -> anyhow::Result<Vec<LogLine>> {
        tracing::debug!("Retrieving logs from container {}...", self.id);

        let logs = container_api::container_logs(
            api_engine_config,
            &self.id,
            None,
            Some(true),
            Some(true),
            None,
            None,
            None,
            None,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to get container logs: {:?}", e))?;

        let log_text = logs
            .text()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to read logs: {:?}", e))?;

        // Format:
        //
        // [STREAM_TYPE] 0x0 0x0 0x0 [SIZE(0..4)] [MESSAGE(0..SIZE)]
        //
        // - STREAM_TYPE is 1 byte: 0x0 for stdin, 0x1 for stdout, 0x2 for stderr
        // - SIZE is a 4 byte big-endian integer indicating the size of the message
        // - MESSAGE is the actual log message of length SIZE
        // See:
        // https://docs.docker.com/reference/api/engine/version/v1.54/#tag/Container/operation/ContainerAttach

        let reader = std::io::Cursor::new(log_text.as_bytes());
        let mut reader = tokio::io::BufReader::new(reader);
        let mut log_lines = Vec::new();

        loop {
            let mut header = [0u8; 8];

            let bytes_read = reader.read(&mut header).await?;
            if bytes_read != 8 {
                break;
            }

            let stream_type = match header[0] {
                0x0 => StreamType::Stdin,
                0x1 => StreamType::Stdout,
                0x2 => StreamType::Stderr,
                other => {
                    tracing::warn!("Received log line with unknown stream type: {}", other);
                    continue;
                }
            };

            let size = u32::from_be_bytes([header[4], header[5], header[6], header[7]]) as usize;

            let mut buffer = vec![0u8; size];
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read != size {
                tracing::warn!(
                    "Received malformed log line with declared size {} but actual size is {}: {}",
                    size,
                    bytes_read,
                    String::from_utf8_lossy(&buffer)
                );
                continue;
            }

            let message = String::from_utf8_lossy(&buffer).to_string();
            log_lines.push(LogLine {
                stream_type,
                message,
            });
        }

        Ok(log_lines)
    }

    /// Delete the container
    pub async fn delete(&self, api_engine_config: &Configuration) -> anyhow::Result<()> {
        tracing::info!("Deleting container {}...", self.id);

        container_api::container_delete(api_engine_config, &self.id, None, None, None)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to delete container: {:?}", e))?;

        tracing::info!("Container deleted");
        Ok(())
    }
}
