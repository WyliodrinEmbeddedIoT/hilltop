use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::TryParseMessage;

#[derive(Debug, Deserialize)]
pub struct NewJob {
    command: String,
    pub job_identifier: String,
    pub job_data_identifier: String,
    pub job_description: JobDescription,
}

#[derive(Debug, Deserialize)]
pub struct JobDescription {
    pub image: String,
    pub hardware: String,
    pub stdout_artifact: bool,
    pub stderr_artifact: bool,
    pub artifacts: Vec<String>,
    /// Path to the test application inside the job archive.
    #[serde(default)]
    pub test_app: Option<String>,
    /// Expected application name in tockloader output.
    #[serde(default)]
    pub test_app_name: Option<String>,
    /// Arbitrary client-declared env vars for the job container.
    #[serde(default)]
    pub env: HashMap<String, String>,
}

impl TryParseMessage for NewJob {
    fn try_parse(data: &[u8]) -> Option<Self> {
        let msg: NewJob = serde_json::from_slice(data).ok()?;
        if msg.command != "NEW_JOB" {
            return None;
        }
        Some(msg)
    }
}

#[derive(Debug, Serialize)]
pub struct NewJobAck {
    pub response: &'static str,
    pub data: NewJobAckData,
}

#[derive(Debug, Serialize)]
pub struct NewJobAckData {
    pub job_identifier: String,
}

impl NewJobAck {
    pub fn new(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_ACK",
            data: NewJobAckData {
                job_identifier: job_identifier.into(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NewJobNack {
    pub response: &'static str,
    pub error_code: i32,
    pub message: &'static str,
    pub data: NewJobNackData,
}

#[derive(Debug, Serialize)]
pub struct NewJobNackData {
    pub job_identifier: String,
}

impl NewJobNack {
    pub fn failed_to_parse(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_NACK",
            error_code: 1000,
            message: "Failed to parse job description.",
            data: NewJobNackData {
                job_identifier: job_identifier.into(),
            },
        }
    }

    pub fn invalid_hardware(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_NACK",
            error_code: 1001,
            message: "Invalid hardware.",
            data: NewJobNackData {
                job_identifier: job_identifier.into(),
            },
        }
    }

    pub fn invalid_image(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_NACK",
            error_code: 1002,
            message: "Invalid image.",
            data: NewJobNackData {
                job_identifier: job_identifier.into(),
            },
        }
    }

    pub fn reserved_env_key(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_NACK",
            error_code: 1004,
            message: "Job env declares a key reserved for the runner.",
            data: NewJobNackData {
                job_identifier: job_identifier.into(),
            },
        }
    }

    pub fn internal_error(job_identifier: impl Into<String>) -> Self {
        Self {
            response: "NEW_JOB_NACK",
            error_code: 1003,
            message: "Internal runner error.",
            data: NewJobNackData {
                job_identifier: job_identifier.into(),
            },
        }
    }
}
