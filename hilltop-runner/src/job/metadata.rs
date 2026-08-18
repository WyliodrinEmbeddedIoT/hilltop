use std::collections::HashMap;

use socket_protocol::messages::new_job::JobDescription;

use crate::{config::RunnerConfig, job::error::JobMetadataError};

/// Env var names the runner sets itself from physical device enumeration.
/// Clients cannot know these ahead of time (they depend on which runner and
/// which physical device instance the job lands on), so `env` is not allowed
/// to override them.
const RESERVED_ENV_KEYS: &[&str] = &[
    "HILLTOP_PROBE_SELECTOR",
    "HILLTOP_BOARD",
    "HILLTOP_DEVICE_SERIAL",
    "HILLTOP_TEST_APP",
    "HILLTOP_TEST_APP_NAME",
];

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
    /// Path to the test application inside the job archive.
    #[serde(default)]
    pub test_app: Option<String>,
    /// Expected application name in tockloader output.
    #[serde(default)]
    pub test_app_name: Option<String>,
    /// Arbitrary env vars the client wants set in the job container (board
    /// name, chip name, flash target, or anything else specific to whatever
    /// tool the client's entrypoint.sh runs). The runner does not interpret
    /// these keys or values at all.
    #[serde(default)]
    pub env: HashMap<String, String>,
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
            test_app: job_description.test_app,
            test_app_name: job_description.test_app_name,
            env: job_description.env,
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

        if let Some(reserved_key) = self
            .env
            .keys()
            .find(|k| RESERVED_ENV_KEYS.contains(&k.as_str()))
        {
            return Err(JobMetadataError::ReservedEnvKey {
                key: reserved_key.clone(),
            });
        }

        Ok(())
    }
}
