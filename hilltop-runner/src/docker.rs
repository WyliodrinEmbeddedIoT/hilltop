use tokio::net::UnixListener;

use crate::api::apis::image_api;
use crate::hardware::HardwareManager;
use crate::{api::apis::configuration::Configuration, config::RunnerConfig};

use std::path::{Path, PathBuf};

pub mod container;

const IMAGE_BUILD_ARCHIVE_NAME: &str = "image_build.tar";

pub async fn create_docker_config() -> anyhow::Result<Configuration> {
    let client = reqwest::Client::builder()
        .unix_socket("/var/run/docker.sock")
        .build()?;

    let mut config = Configuration::new();
    config.client = client;
    config.base_path = "http://localhost".to_string();

    // Validate API version matches
    let version_info = crate::api::apis::system_api::system_version(&config).await?;
    let api_version = version_info.api_version.as_deref().unwrap_or("unknown");

    if api_version != crate::api::API_VERSION {
        anyhow::bail!(
            "Unsupported Docker API version: {api_version}. Expected {}",
            crate::api::API_VERSION
        );
    }

    Ok(config)
}

pub struct DockerManager {
    /// Where the config and image build directory is located on host
    config_dir: PathBuf,
    /// Temporary directory for docker manager to transfer files and submit
    /// image build requests.
    temp_dir: tempfile::TempDir,
    /// Path (guaranteed inside temp_dir) to the tar archive to build images
    image_build_archive: Option<PathBuf>,
    /// Api engine configuration to submit commands
    api_engine_config: Configuration,
    /// Runner config
    runner_config: RunnerConfig,
    /// Hardware Manager
    hardware_manager: HardwareManager,
}

impl DockerManager {
    /// Create a new DockerManager with Docker configuration
    pub async fn new(config_dir: PathBuf, runner_config: RunnerConfig) -> anyhow::Result<Self> {
        let config = create_docker_config().await?;
        let mut temp_dir =
            tempfile::tempdir().map_err(|e| anyhow::anyhow!("Failed to build tempdir: {e:?}"))?;

        temp_dir.disable_cleanup(true);

        let hardware_manager = HardwareManager::new(&runner_config).await?;

        Ok(Self {
            config_dir,
            temp_dir,
            image_build_archive: None,
            api_engine_config: config,
            runner_config,
            hardware_manager,
        })
    }

    /// Get a reference to the Docker API configuration
    pub fn api_config(&self) -> &Configuration {
        &self.api_engine_config
    }

    /// Get a reference to the runner configuration
    pub fn runner_config(&self) -> &RunnerConfig {
        &self.runner_config
    }

    pub fn hardware_manager(&self) -> &HardwareManager {
        &self.hardware_manager
    }

    pub fn hardware_manager_mut(&mut self) -> &mut HardwareManager {
        &mut self.hardware_manager
    }

    pub fn image_build_archive(&mut self) -> anyhow::Result<&PathBuf> {
        if let Some(ref path) = self.image_build_archive {
            Ok(path)
        } else {
            // Archive config_dir
            let archive_path = self.temp_dir.path().join(IMAGE_BUILD_ARCHIVE_NAME);
            let file = std::fs::File::create(&archive_path)
                .map_err(|e| anyhow::anyhow!("Failed to create image build archive file: {e:?}"))?;

            let mut builder = tar::Builder::new(file);
            builder
                .append_dir_all(".", &self.config_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create image build archive: {e:?}"))?;
            self.image_build_archive = Some(archive_path);

            tracing::debug!(
                "Created image build archive at {}",
                self.image_build_archive.as_ref().unwrap().display()
            );

            Ok(&self.image_build_archive.as_ref().unwrap())
        }
    }

    pub fn temp_artifact_path(&self, name: &str) -> PathBuf {
        self.temp_dir.path().join(name)
    }

    /// Build a single image from Dockerfile
    pub async fn build_image(
        &mut self,
        image_name: &str,
        image_tag: &str,
        dockerfile_path: &Path,
    ) -> anyhow::Result<()> {
        tracing::info!(
            "Building image: {} -> {} from {}",
            image_name,
            image_tag,
            dockerfile_path.display()
        );

        let archive_path = self.image_build_archive()?.clone();

        let image_build_logs = image_api::image_build(
            &self.api_engine_config,
            Some(dockerfile_path.to_str().unwrap()),
            Some(image_tag),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(archive_path),
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to build image: {:?}", e))?;

        tracing::debug!("Image build logs:\n{}", image_build_logs);

        if image_build_logs.contains("\"error\":") {
            anyhow::bail!("Image build failed: {}", image_build_logs);
        }

        tracing::info!("Successfully built image: {}", image_tag);
        Ok(())
    }

    /// Create a tokio socket for a container to stream logs
    pub async fn create_socket(
        &self,
        socket_name: &str,
    ) -> anyhow::Result<(UnixListener, PathBuf)> {
        let socket_path = self.temp_dir.path().join(socket_name);
        if socket_path.exists() {
            anyhow::bail!("Socket path already exists: {}", socket_path.display());
        }

        let listener = UnixListener::bind(&socket_path)
            .map_err(|e| anyhow::anyhow!("Failed to create unix socket: {e:?}"))?;

        tracing::debug!("Created unix socket at {}", socket_path.display());

        Ok((listener, socket_path.canonicalize().unwrap()))
    }
}
