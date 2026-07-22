use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct JobError {
    pub command: &'static str,
    pub job_identifier: String,
    pub error_message: String,
}

impl JobError {
    pub fn new(job_identifier: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            command: "JOB_ERROR",
            job_identifier: job_identifier.into(),
            error_message: error_message.into(),
        }
    }
}

impl SocketProtocolMessage for JobError {
    type Response = JobErrorResponse;
}

pub type JobErrorResponse = GenericResponse;
