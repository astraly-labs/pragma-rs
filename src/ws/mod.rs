pub mod lightspeed;
pub mod starkex;

use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::Serialize;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const PING_INTERVAL: Duration = Duration::from_secs(25);

#[derive(Error, Debug)]
pub enum WsError {
    #[error("Connection error: {0}")]
    Connection(String),
    #[error("Send error: {0}")]
    Send(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

pub struct PragmaWsClient<T> {
    url: String,
    api_key: String,
    outgoing_sender: mpsc::UnboundedSender<T>,
    outgoing_receiver: Option<mpsc::UnboundedReceiver<T>>,
    incoming_sender: mpsc::UnboundedSender<T>,
    incoming_receiver: mpsc::UnboundedReceiver<T>,
    message_handler: Arc<dyn Fn(String) -> Option<T> + Send + Sync>,
}

impl<T: Send + 'static + Serialize> PragmaWsClient<T> {
    /// Creates a new WebSocket client with separate channels for sending and receiving.
    pub fn new<F>(url: String, api_key: String, message_handler: F) -> Self
    where
        F: Fn(String) -> Option<T> + Send + Sync + 'static,
    {
        // Channel for outgoing messages (user -> WebSocket)
        let (outgoing_sender, outgoing_receiver) = mpsc::unbounded_channel::<T>();
        // Channel for incoming messages (WebSocket -> user)
        let (incoming_sender, incoming_receiver) = mpsc::unbounded_channel::<T>();

        Self {
            url,
            api_key,
            outgoing_sender,
            outgoing_receiver: Some(outgoing_receiver),
            incoming_sender,
            incoming_receiver,
            message_handler: Arc::new(message_handler),
        }
    }

    /// Connects to the WebSocket and starts processing messages in separate tasks.
    pub async fn connect(&mut self) -> Result<(), WsError> {
        let url = self.url.clone();
        let message_handler = self.message_handler.clone();
        let api_key = self.api_key.clone();

        let mut request = match url.into_client_request() {
            Ok(r) => Ok(r),
            Err(e) => Err(WsError::Connection(format!("{e}"))),
        }?;

        request
            .headers_mut()
            .insert("x-api-key", api_key.parse().unwrap());

        let (ws_stream, _) = connect_async(request)
            .await
            .map_err(|e| WsError::Connection(e.to_string()))?;

        let (mut write, mut read) = ws_stream.split();

        let incoming_sender = self.incoming_sender.clone();

        let Some(mut outgoing_receiver) = self.outgoing_receiver.take() else {
            return Err(WsError::Send("Connect already called.".into()));
        };

        tokio::spawn(async move {
            let mut ping_interval = tokio::time::interval(PING_INTERVAL);
            loop {
                tokio::select! {
                    // Handle outgoing user messages
                    msg = outgoing_receiver.recv() => {
                        match msg {
                            Some(msg) => {
                                if let Ok(json) = serde_json::to_string(&msg) {
                                    if write.send(Message::Text(json.into())).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                    // Handle periodic pings
                    _ = ping_interval.tick() => {
                        if write.send(Message::Ping(Default::default())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        if let Some(parsed) = message_handler(text.to_string()) {
                            let _ = incoming_sender.send(parsed);
                        }
                    }
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    /// Sends a message to the WebSocket using the outgoing sender.
    pub async fn send(&self, msg: T) -> Result<(), WsError> {
        self.outgoing_sender
            .send(msg)
            .map_err(|e| WsError::Send(e.to_string()))?;
        Ok(())
    }

    /// Receives the next parsed message from the WebSocket.
    pub async fn recv(&mut self) -> Option<T> {
        self.incoming_receiver.recv().await
    }
}
