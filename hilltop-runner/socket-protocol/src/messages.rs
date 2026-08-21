use serde::Serialize;

pub mod config_runner;
pub mod generic;
pub mod goodbye;
pub mod hello_runner;
pub mod job_error;
pub mod job_finished;
pub mod job_started;
pub mod new_job;

pub trait TryParseMessage {
    fn try_parse(data: &[u8]) -> Option<Self>
    where
        Self: Sized;
}

pub trait SocketProtocolMessage: Serialize {
    type Response: TryParseMessage;
}
