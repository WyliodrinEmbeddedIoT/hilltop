use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct JobStarted {
    pub command: &'static str,
    pub job_identifier: String,
}

impl JobStarted {
    pub fn new(job_identifier: impl Into<String>) -> Self {
        Self {
            command: "JOB_STARTED",
            job_identifier: job_identifier.into(),
        }
    }
}

impl SocketProtocolMessage for JobStarted {
    type Response = JobStartedResponse;
}

pub type JobStartedResponse = GenericResponse;
