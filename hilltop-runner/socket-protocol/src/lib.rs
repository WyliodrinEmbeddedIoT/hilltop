use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tungstenite::client::IntoClientRequest;

use crate::messages::{
    SocketProtocolMessage, TryParseMessage,
    config_runner::ConfigRunner,
    generic::GenericResponse,
    goodbye::Goodbye,
    hello_runner::HelloRunner,
    job_error::JobError,
    job_finished::JobFinished,
    job_started::JobStarted,
    new_job::{NewJob, NewJobAck, NewJobNack},
};

pub mod messages;

#[derive(Debug, thiserror::Error)]
pub enum SocketProtocolError {
    #[error("The broker returned a NACK: {0} - {1}")]
    Nack(i32, String),
    #[error("The broker thinks we sent a malformed request")]
    MalformedRequest,
    #[error("The broker does not allow this message in this state. Context: {0:?}")]
    ErrState(Option<String>),
    #[error("The broker returned an unexpected response")]
    UnexpectedResponse(String),
    #[error("The broker returned a malformed response")]
    MalformedResponse(Vec<u8>),
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("WebSocket error: {0}")]
    Socket(#[from] tungstenite::Error),
}

pub struct SocketProtocolClient {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl SocketProtocolClient {
    pub async fn connect(url: &str) -> tungstenite::Result<Self> {
        let request = url.into_client_request()?;
        let (stream, _response) = connect_async(request).await?;

        // dbg!(response);

        Ok(SocketProtocolClient { stream })
    }

    pub async fn send_message<M: SocketProtocolMessage>(
        &mut self,
        msg: &M,
    ) -> Result<(Option<M::Response>, Bytes), SocketProtocolError> {
        let msg = serde_json::to_string(&msg)?;

        self.stream
            .send(tungstenite::Message::Text(msg.into()))
            .await?;

        let response = loop {
            let msg = self.stream.next().await.ok_or(SocketProtocolError::ConnectionClosed)??;
            match msg {
                tungstenite::Message::Ping(_) | tungstenite::Message::Pong(_) => continue,
                tungstenite::Message::Close(_) => return Err(SocketProtocolError::ConnectionClosed),
                _ => break msg.into_data(),
            }
        };

        Ok((M::Response::try_parse(&response), response))
    }

    pub async fn hello(&mut self) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&HelloRunner::new("1.0.0")).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    pub async fn config(&mut self, config: ConfigRunner) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&config).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    pub async fn goodbye(&mut self) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&Goodbye::new()).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    pub async fn job_started(
        &mut self,
        job_started: JobStarted,
    ) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&job_started).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    pub async fn job_finished(
        &mut self,
        job_finished: JobFinished,
    ) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&job_finished).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    pub async fn job_error(&mut self, job_error: JobError) -> Result<(), SocketProtocolError> {
        let (resp, data) = self.send_message(&job_error).await?;
        GenericResponse::ok_or_err(resp, &data)
    }

    /// Wait for a new job. Make sure to ack_new_job or nack_new_job after!
    pub async fn wait_new_job(&mut self) -> Result<NewJob, SocketProtocolError> {
        loop {
            let msg = self.stream.next().await.ok_or(SocketProtocolError::ConnectionClosed)??;
            match msg {
                tungstenite::Message::Ping(_) | tungstenite::Message::Pong(_) => continue,
                tungstenite::Message::Close(_) => return Err(SocketProtocolError::ConnectionClosed),
                _ => {
                    let response = msg.into_data();
                    return NewJob::try_parse(&response).ok_or_else(|| {
                        if let Ok(text) = String::from_utf8(response.to_vec()) {
                            SocketProtocolError::UnexpectedResponse(text)
                        } else {
                            SocketProtocolError::MalformedResponse(response.to_vec())
                        }
                    });
                }
            }
        }
    }

    pub async fn ack_new_job(&mut self, ack: NewJobAck) -> Result<(), SocketProtocolError> {
        let msg = serde_json::to_string(&ack)?;
        self.stream
            .send(tungstenite::Message::Text(msg.into()))
            .await?;
        Ok(())
    }

    pub async fn nack_new_job(&mut self, nack: NewJobNack) -> Result<(), SocketProtocolError> {
        let msg = serde_json::to_string(&nack)?;
        self.stream
            .send(tungstenite::Message::Text(msg.into()))
            .await?;
        Ok(())
    }
}
