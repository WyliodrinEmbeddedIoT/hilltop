#[derive(Debug, thiserror::Error)]
pub enum JobMetadataError {
    #[error("Failed to parse job metadata: {0}")]
    FailedToParse(#[source] serde_json::Error),
    #[error("Job metadata is not valid UTF-8: {0}")]
    NonUtf8Metadata(#[source] std::str::Utf8Error),
    #[error("Job references non-existent image '{image}'")]
    InvalidImage { image: String },
    #[error("Job references non-existent hardware configuration '{hardware}'")]
    InvalidHardware { hardware: String },
    #[error("Job env declares reserved key '{key}', which is set by the runner")]
    ReservedEnvKey { key: String },
}

/// Errors that occur during job execution
#[derive(Debug, thiserror::Error)]
pub enum JobExecutionError {
    #[error("Entrypoint script for job failed: {0}")]
    JobSetupError(String),
    #[error("Hardware error: {0}")]
    HardwareError(#[source] HardwareError),
    #[error("Internal error")]
    InternalError,
}

#[derive(Debug, thiserror::Error)]
pub enum HardwareError {
    #[error("Failed to find hardware configuration: {0}")]
    UnknownConfiguration(String),
    #[error("Failed to acquire devices, resources busy")]
    ResourcesBusy,
    #[error("Internal error")]
    InternalError,
}
