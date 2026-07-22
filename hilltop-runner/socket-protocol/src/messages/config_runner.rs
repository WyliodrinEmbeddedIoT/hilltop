use serde::Serialize;

use crate::messages::SocketProtocolMessage;

use super::generic::GenericResponse;

#[derive(Debug, Serialize)]
pub struct ConfigRunner {
    pub command: &'static str,
    pub runner_slug: String,
    pub api_key: String,
}

impl ConfigRunner {
    pub fn new(runner_slug: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            command: "CONFIG_RUNNER",
            runner_slug: runner_slug.into(),
            api_key: api_key.into(),
        }
    }
}

impl SocketProtocolMessage for ConfigRunner {
    type Response = ConfigRunnerResponse;
}

pub type ConfigRunnerResponse = GenericResponse;
