use std::path::{Path, PathBuf};

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};
use uuid::Uuid;

use crate::docker::DockerManager;
use crate::docker::container::{Container, LogLine, UserDevice, UserMount};
use crate::job::error::JobExecutionError;
use crate::job::metadata::JobMetadata;

pub mod error;
pub mod metadata;

/// Current state of a job
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    /// Job just created, not yet started
    Created,
    /// Container is running
    Running,
    /// Container completed successfully
    Completed,
    /// Job cleaned up and resources released
    CleanedUp,
}

#[derive(Debug)]
pub struct Job {
    id: Uuid,
    pub broker_job_identifier: String,
    metadata: JobMetadata,
    /// Zip file bytes containing the job data (e.g. source code, input files, etc.)
    job_data: Vec<u8>,
    state: JobState,
    container: Option<Container>,
    container_socket: Option<JobSocket>,
}

#[derive(Debug)]
struct JobSocket {
    listener: UnixListener,
    stream: Option<UnixStream>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JobMessage {
    status: JobMessageStatus,
    message: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum JobMessageStatus {
    #[serde(rename = "container-started")]
    ContainerStarted,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "job-ready")]
    JobReady,
    #[serde(rename = "job-started")]
    JobStarted,
    #[serde(rename = "job-completed")]
    JobCompleted,
    #[serde(rename = "container-completed")]
    ContainerCompleted,
}

impl Job {
    /// Create a new job from metadata and data
    pub fn new(broker_job_identifier: String, metadata: JobMetadata, job_data: Vec<u8>) -> Self {
        Job {
            id: Uuid::new_v4(),
            broker_job_identifier,
            metadata,
            job_data,
            state: JobState::Created,
            container: None,
            container_socket: None,
        }
    }

    pub fn socket_name(&self) -> String {
        format!("hilltop_job_{}.sock", self.id)
    }

    pub fn can_start_job(&self, docker: &DockerManager) -> Result<bool, JobExecutionError> {
        if self.state != JobState::Created {
            return Ok(false);
        }

        if !docker
            .runner_config()
            .get_image(&self.metadata.image)
            .is_some()
        {
            return Ok(false);
        }

        if !docker
            .hardware_manager()
            .get_configuration(&self.metadata.hardware)
            .is_some()
        {
            return Ok(false);
        }

        if !docker
            .hardware_manager()
            .can_acquire_configuration(&self.metadata.hardware)
            .map_err(|err| {
                tracing::error!(
                    "Failed to check hardware configuration availability: {}",
                    err
                );
                JobExecutionError::HardwareError(err)
            })?
        {
            return Ok(false);
        }

        Ok(true)
    }

    /// Create the container for this job
    pub async fn start_job(&mut self, docker: &mut DockerManager) -> Result<(), JobExecutionError> {
        if self.state != JobState::Created {
            tracing::error!("Cannot create container in state: {:?}", self.state);

            return Err(JobExecutionError::InternalError);
        }

        // Acquire hardware resources
        docker
            .hardware_manager_mut()
            .acquire_configuration(&self.metadata.hardware)
            .map_err(|e| {
                tracing::warn!("Failed to acquire hardware configuration: {}", e);
                JobExecutionError::HardwareError(e)
            })?;

        // Create socket

        let (container_socket_listener, container_socket_path) = docker
            .create_socket(&self.socket_name())
            .await
            .map_err(|e| {
                tracing::error!("Failed to create socket for job: {}", e);

                JobExecutionError::InternalError
            })?;

        self.container_socket = Some(JobSocket {
            listener: container_socket_listener,
            stream: None,
        });

        // Create container

        let container_name = format!("hilltop_job_{}", self.id);
        let image_tag = &docker
            .runner_config()
            .get_image(&self.metadata.image)
            .ok_or_else(|| {
                tracing::error!("Image not found in config: {}", self.metadata.image);

                JobExecutionError::InternalError
            })?
            .tag
            .clone();

        let mut user_mounts = Vec::new();
        user_mounts.push(UserMount {
            host_path: container_socket_path,
            container_path: Path::new("/var/run/hilltop.sock").to_path_buf(),
            read_only: false,
        });

        let hw_config = docker
            .hardware_manager_mut()
            .get_configuration(&self.metadata.hardware)
            .ok_or_else(|| {
                tracing::error!(
                    "Hardware configuration not found in config: {}",
                    self.metadata.hardware
                );
                JobExecutionError::InternalError
            })?
            .clone();

        let mut user_devices = Vec::new();
        let mut probe_selectors = Vec::new();
        let mut passthrough_device_metadata = Vec::new();
        for device_ref in hw_config.devices.iter() {
            let device_descriptor = docker
                .hardware_manager_mut()
                .get_device(&device_ref.device_name)
                .ok_or_else(|| {
                    tracing::error!(
                        "Hardware config '{}' references unknown device '{}'",
                        hw_config.config_name,
                        device_ref.device_name
                    );
                    JobExecutionError::InternalError
                })?;

            if device_ref.device_passthrough == Some(true) {
                if let Some(selector) = device_descriptor.probe_selector() {
                    probe_selectors.push(selector);
                }
                passthrough_device_metadata.push((
                    device_descriptor.device_name().to_string(),
                    device_descriptor.serial().to_string(),
                    device_descriptor.probe_rs_chip().map(str::to_string),
                    device_descriptor.tockloader_board().map(str::to_string),
                    device_descriptor
                        .tockloader_rs_board()
                        .map(str::to_string),
                    device_descriptor.board_dir().map(str::to_string),
                    device_descriptor.flash_target().map(str::to_string),
                    device_descriptor.openocd_board().map(str::to_string),
                ));
                let bus = device_descriptor.bus_path().to_path_buf();
                user_devices.push(UserDevice {
                    dev_host_path: bus.clone(),
                    container_path: bus,
                });
            } else {
                let container_path = device_ref
                    .container_path
                    .as_ref()
                    .expect("container_path validated at startup");
                user_devices.push(UserDevice {
                    dev_host_path: device_descriptor
                        .dev_path()
                        .expect("dev_path validated at startup")
                        .to_path_buf(),
                    container_path: PathBuf::from(container_path),
                });
            }
        }

        let mut container_environment = Vec::new();
        if probe_selectors.len() == 1 {
            container_environment.push(format!("HILLTOP_PROBE_SELECTOR={}", probe_selectors[0]));
            if let Some((
                device_name,
                serial,
                probe_rs_chip,
                tockloader_board,
                tockloader_rs_board,
                board_dir,
                flash_target,
                openocd_board,
            )) =
                passthrough_device_metadata.first()
            {
                container_environment.push(format!("HILLTOP_BOARD={device_name}"));
                container_environment.push(format!("HILLTOP_DEVICE_SERIAL={serial}"));
                if let Some(chip) = probe_rs_chip {
                    container_environment.push(format!("HILLTOP_PROBE_RS_CHIP={chip}"));
                }
                if let Some(board) = tockloader_board {
                    container_environment.push(format!("HILLTOP_TOCKLOADER_BOARD={board}"));
                }
                if let Some(board) = tockloader_rs_board {
                    container_environment.push(format!("HILLTOP_TOCKLOADER_RS_BOARD={board}"));
                }
                if let Some(board_dir) = board_dir {
                    container_environment.push(format!("HILLTOP_BOARD_DIR={board_dir}"));
                }
                if let Some(flash_target) = flash_target {
                    container_environment.push(format!("HILLTOP_FLASH_TARGET={flash_target}"));
                }
                if let Some(openocd_board) = openocd_board {
                    container_environment
                        .push(format!("HILLTOP_OPENOCD_BOARD={openocd_board}"));
                }
            }
        } else if probe_selectors.len() > 1 {
            tracing::warn!(
                "Hardware configuration '{}' contains multiple passthrough devices; not setting HILLTOP_PROBE_SELECTOR",
                hw_config.config_name
            );
        }

        if let Some(test_app) = &self.metadata.test_app {
            container_environment.push(format!("HILLTOP_TEST_APP={test_app}"));
        }
        if let Some(test_app_name) = &self.metadata.test_app_name {
            container_environment.push(format!("HILLTOP_TEST_APP_NAME={test_app_name}"));
        }

        tracing::debug!(
            "Request to create container with tag {}, container name {}, user mounts {:?}, and devices {:?}",
            image_tag,
            container_name,
            user_mounts,
            user_devices
        );

        let container = Container::create(
            docker.api_config(),
            image_tag,
            &container_name,
            &user_mounts,
            &user_devices,
            &container_environment,
        )
        .await
        .map_err(|e| {
            tracing::error!("Container creation failed: {}", e);

            JobExecutionError::InternalError
        })?;

        // Copy job data to container
        container
            .copy_from_host(
                docker.api_config(),
                &self.job_data,
                "job-data.zip",
                Path::new("/workspace/"),
            )
            .await
            .map_err(|e| {
                tracing::error!("Data copy to container failed: {}", e);

                JobExecutionError::InternalError
            })?;

        // Start container
        container.start(docker.api_config()).await.map_err(|e| {
            tracing::error!("Container start failed: {}", e);
            JobExecutionError::InternalError
        })?;

        // Start socket

        let (stream, _) = self
            .container_socket
            .as_mut()
            .unwrap()
            .listener
            .accept()
            .await
            .map_err(|e| {
                tracing::error!("Failed to accept connection on job socket: {}", e);

                JobExecutionError::InternalError
            })?;

        tracing::info!("Accepted connection on job socket");

        self.container_socket.as_mut().unwrap().stream = Some(stream);
        self.container = Some(container);
        self.state = JobState::Running;

        Ok(())
    }

    /// Wait for container to complete
    pub async fn wait_for_completion(&mut self) -> Result<(), JobExecutionError> {
        if self.state != JobState::Running {
            tracing::error!("Cannot wait for completion in state: {:?}", self.state);
            return Err(JobExecutionError::InternalError);
        }

        let stream = self
            .container_socket
            .as_mut()
            .unwrap()
            .stream
            .as_mut()
            .expect("Expected socket stream to exist in state Running");

        let mut reader = BufReader::new(stream);
        let mut line = Vec::new();

        loop {
            line.clear();

            let bytes_read = reader.read_until(b'\n', &mut line).await.map_err(|e| {
                tracing::error!("Error reading from socket: {}", e);
                JobExecutionError::InternalError
            })?;

            if bytes_read == 0 {
                tracing::info!("Container has closed the socket, assuming completion");
                break;
            }

            let msg = String::from_utf8_lossy(&line);
            let job_message = serde_json::from_str::<JobMessage>(&msg).map_err(|e| {
                tracing::error!(
                    "Failed to parse message from container: {}, error: {}",
                    msg,
                    e
                );
                JobExecutionError::InternalError
            })?;

            tracing::debug!("Received message from container: {:?}", job_message);

            if job_message.status == JobMessageStatus::Error {
                tracing::warn!("Received error message from container: {:?}", job_message);
                return Err(JobExecutionError::JobSetupError(
                    job_message.message.unwrap_or("unknown error".to_string()),
                ));
            }

            if job_message.status == JobMessageStatus::ContainerCompleted {
                tracing::info!("Received container completed message");
                break;
            }
        }

        self.state = JobState::Completed;
        Ok(())
    }

    pub async fn get_logs(
        &self,
        docker: &DockerManager,
    ) -> Result<Vec<LogLine>, JobExecutionError> {
        if self.state != JobState::Completed {
            tracing::error!("Cannot get logs in state: {:?}", self.state);
            return Err(JobExecutionError::InternalError);
        }

        let container = self
            .container
            .as_ref()
            .expect("Expected container to exist in state Completed");

        container.get_logs(docker.api_config()).await.map_err(|e| {
            tracing::error!("Failed to get logs: {:?}", e);
            JobExecutionError::InternalError
        })
    }

    pub async fn get_artifacts(
        &self,
        docker: &DockerManager,
    ) -> Result<Vec<(String, PathBuf)>, JobExecutionError> {
        if self.state != JobState::Completed {
            tracing::error!("Cannot get artifacts in state: {:?}", self.state);
            return Err(JobExecutionError::InternalError);
        }

        // We have two special artifacts:
        // - /artifacts/stdout.log (if job config has stdout capture enabled)
        // - /artifacts/stderr.log (if job config has stderr capture enabled)
        //
        // Then, we have the remaining files in /workspace/ which are defined in
        // the job via relative path to it.

        let mut artifact_paths = Vec::new();

        let container = self
            .container
            .as_ref()
            .expect("Expected container to exist in state Completed");

        if self.metadata.stdout_artifact {
            let host_path = docker.temp_artifact_path(format!("{}_stdout.log", self.id).as_str());
            container
                .copy_to_host(
                    docker.api_config(),
                    Path::new("/artifacts/stdout.log"),
                    &host_path,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to copy stdout artifact: {:?}", e);
                    JobExecutionError::InternalError
                })?;
            artifact_paths.push(("stdout_artifact".to_string(), host_path));
        }

        if self.metadata.stderr_artifact {
            let host_path = docker.temp_artifact_path(format!("{}_stderr.log", self.id).as_str());
            container
                .copy_to_host(
                    docker.api_config(),
                    Path::new("/artifacts/stderr.log"),
                    &host_path,
                )
                .await
                .map_err(|e| {
                    tracing::error!("Failed to copy stderr artifact: {:?}", e);
                    JobExecutionError::InternalError
                })?;
            artifact_paths.push(("stderr_artifact".to_string(), host_path));
        }

        for artifact in &self.metadata.artifacts {
            let artifact_id = artifact.replace("/", "_");
            let host_path =
                docker.temp_artifact_path(format!("{}_{}", self.id, artifact_id).as_str());

            let container_path = Path::new("/workspace/").join(artifact);

            let added = container
                .copy_to_host(docker.api_config(), &container_path, &host_path)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to copy artifact: {:?}", e);
                    JobExecutionError::InternalError
                })?;

            if added {
                artifact_paths.push((artifact.clone(), host_path));
            } else {
                tracing::warn!(
                    "Artifact '{}' was defined in job metadata but was not found in container at '{}'",
                    artifact,
                    container_path.display()
                );
            }
        }

        Ok(artifact_paths)
    }

    pub async fn cleanup(&mut self, docker: &mut DockerManager) -> Result<(), JobExecutionError> {
        if self.state != JobState::Completed {
            tracing::error!("Cannot cleanup in state: {:?}", self.state);
            return Err(JobExecutionError::InternalError);
        }

        // Release hardware resources
        docker
            .hardware_manager_mut()
            .release_configuration(&self.metadata.hardware)
            .map_err(|e| {
                tracing::warn!("Failed to release hardware configuration: {}", e);
                JobExecutionError::HardwareError(e)
            })?;

        if let Some(container) = &self.container {
            container.stop(docker.api_config()).await.map_err(|e| {
                tracing::error!("Failed to stop container: {:?}", e);
                JobExecutionError::InternalError
            })?;

            container.delete(docker.api_config()).await.map_err(|e| {
                tracing::error!("Failed to delete container: {:?}", e);
                JobExecutionError::InternalError
            })?;
        }

        self.state = JobState::CleanedUp;

        Ok(())
    }
}
