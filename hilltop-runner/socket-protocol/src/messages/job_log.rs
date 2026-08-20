use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct JobLog {
    pub command: &'static str,
    pub job_identifier: String,
    pub stream: String,
    pub log_data: String,
}

impl JobLog {
    pub fn new(
        job_identifier: impl Into<String>,
        stream: impl Into<String>,
        log_data: impl Into<String>,
    ) -> Self {
        Self {
            command: "JOB_LOG",
            job_identifier: job_identifier.into(),
            stream: stream.into(),
            log_data: log_data.into(),
        }
    }
}

impl SocketProtocolMessage for JobLog {
    type Response = JobLogResponse;
}

pub type JobLogResponse = GenericResponse;
