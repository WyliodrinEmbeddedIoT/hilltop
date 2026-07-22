use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct JobFinished {
    pub command: &'static str,
    pub job_identifier: String,
}

impl JobFinished {
    pub fn new(job_identifier: impl Into<String>) -> Self {
        Self {
            command: "JOB_FINISHED",
            job_identifier: job_identifier.into(),
        }
    }
}

impl SocketProtocolMessage for JobFinished {
    type Response = JobFinishedResponse;
}

pub type JobFinishedResponse = GenericResponse;
