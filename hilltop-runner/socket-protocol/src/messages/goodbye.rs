use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct Goodbye {
    pub command: &'static str,
}

impl Goodbye {
    pub fn new() -> Self {
        Self { command: "GOODBYE" }
    }
}

impl Default for Goodbye {
    fn default() -> Self {
        Self::new()
    }
}

impl SocketProtocolMessage for Goodbye {
    type Response = GoodbyeResponse;
}

pub type GoodbyeResponse = GenericResponse;
