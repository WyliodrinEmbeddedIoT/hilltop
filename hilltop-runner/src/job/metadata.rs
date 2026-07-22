use socket_protocol::messages::new_job::JobDescription;

use crate::{config::RunnerConfig, job::error::JobMetadataError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JobMetadata {
    /// Name of image, as specified in the runner config's available_images
    pub image: String,
    /// Hardware configuration name
    pub hardware: String,
    pub stdout_artifact: bool,
    pub stderr_artifact: bool,
    #[serde(default)]
    pub artifacts: Vec<String>,
}

impl JobMetadata {
    /// Load job metadata from JSON string
    pub fn from_json(json: &str, runner_config: &RunnerConfig) -> Result<Self, JobMetadataError> {
        let job_metadata: JobMetadata =
            serde_json::from_str(json).map_err(|e| JobMetadataError::FailedToParse(e))?;

        job_metadata.validate(runner_config)?;

        Ok(job_metadata)
    }

    /// Load job metadata from JSON bytes
    pub fn from_bytes(
        bytes: &[u8],
        runner_config: &RunnerConfig,
    ) -> Result<Self, JobMetadataError> {
        let json = std::str::from_utf8(bytes).map_err(|e| JobMetadataError::NonUtf8Metadata(e))?;

        Self::from_json(json, runner_config)
    }

    pub fn from_socket_protocol(
        job_description: JobDescription,
        runner_config: &RunnerConfig,
    ) -> Result<Self, JobMetadataError> {
        let metdata = Self {
            image: job_description.image,
            hardware: job_description.hardware,
            stdout_artifact: job_description.stdout_artifact,
            stderr_artifact: job_description.stderr_artifact,
            artifacts: job_description.artifacts,
        };

        metdata.validate(runner_config)?;
        Ok(metdata)
    }

    pub fn validate(&self, runner_config: &RunnerConfig) -> Result<(), JobMetadataError> {
        if runner_config.get_image(&self.image).is_none() {
            return Err(JobMetadataError::InvalidImage {
                image: self.image.clone(),
            });
        }

        if runner_config.get_hardware(&self.hardware).is_none() {
            return Err(JobMetadataError::InvalidHardware {
                hardware: self.hardware.clone(),
            });
        }

        Ok(())
    }
}
