use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct HelloRunner {
    pub command: &'static str,
    pub socket_protocol_version: String,
}

impl HelloRunner {
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            command: "HELLO_RUNNER",
            socket_protocol_version: version.into(),
        }
    }
}

impl SocketProtocolMessage for HelloRunner {
    type Response = HelloRunnerResponse;
}

pub type HelloRunnerResponse = GenericResponse;
