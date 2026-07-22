use serde::Deserialize;

use crate::SocketProtocolError;

use super::TryParseMessage;

#[derive(Debug, Deserialize)]
pub struct Ack {}

#[derive(Debug, Deserialize)]
pub struct Nack {
    pub message: String,
    pub error_code: i32,
}

#[derive(Debug, Deserialize)]
pub struct MalformedRequest {}

#[derive(Debug, Deserialize)]
pub struct ErrState {
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "response")]
pub enum GenericResponse {
    #[serde(rename = "ACK")]
    Ack,
    #[serde(rename = "NACK")]
    Nack { message: String, error_code: i32 },
    #[serde(rename = "MALFORMED_REQUEST")]
    MalformedRequest,
    #[serde(rename = "ERR_STATE")]
    ErrState { message: Option<String> },
}

impl GenericResponse {
    pub fn try_parse_err(data: &[u8]) -> Result<(), SocketProtocolError> {
        Self::ok_or_err(Self::try_parse(data), data)
    }

    pub fn ok_or_err(maybe_response: Option<Self>, data: &[u8]) -> Result<(), SocketProtocolError> {
        let response = maybe_response.ok_or_else(|| {
            if let Ok(text) = String::from_utf8(data.to_vec()) {
                SocketProtocolError::UnexpectedResponse(text)
            } else {
                SocketProtocolError::MalformedResponse(data.to_vec())
            }
        })?;

        match response {
            GenericResponse::Ack => Ok(()),
            GenericResponse::Nack {
                message,
                error_code,
            } => Err(SocketProtocolError::Nack(error_code, message)),
            GenericResponse::MalformedRequest => Err(SocketProtocolError::MalformedRequest),
            GenericResponse::ErrState { message } => Err(SocketProtocolError::ErrState(message)),
        }
    }
}

impl TryParseMessage for GenericResponse {
    fn try_parse(data: &[u8]) -> Option<Self> {
        serde_json::from_slice(data).ok()
    }
}
