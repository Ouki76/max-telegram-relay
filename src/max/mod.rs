use std::sync::{Arc, atomic::AtomicI64};

use tokio::sync::mpsc::{self, error::SendError};
use tokio_tungstenite::tungstenite::Message;

use crate::models;

type SendHandler = Result<(), SendError<Message>>;

pub struct WsClient {
    tx: mpsc::Sender<Message>,
    seq_counter: Arc<AtomicI64>,
}

impl WsClient {
    pub fn new(tx: mpsc::Sender<Message>) -> Self {
        Self {
            tx,
            seq_counter: Arc::new(AtomicI64::new(0)),
        }
    }

    pub async fn send_request<T>(&self, opcode: i64, payload: T) -> SendHandler
    where
        T: serde::Serialize,
    {
        let req = models::Request {
            ver: 11,
            cmd: 0,
            seq: self
                .seq_counter
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            opcode,
            payload,
        };

        let json = serde_json::to_string(&req).unwrap_or_default();

        self.tx.send(Message::text(json)).await
    }

    pub async fn send_handshake(&self) -> SendHandler {
        use models::handshake;

        let handshake_payload = handshake::Payload {
            user_agent: handshake::UserAgent::default(),
            device_id: uuid::Uuid::new_v4().to_string(),
        };

        self.send_request(6, handshake_payload).await
    }

    pub async fn send_auth(&self, token: String) -> SendHandler {
        use models::auth;

        let auth_payload = auth::Auth {
            token,
            interactive: true,
            chats_count: 40,
            chats_sync: 0,
            contacts_sync: 0,
            presence_sync: -1,
            drafts_sync: 0,
        };

        self.send_request(19, auth_payload).await
    }
}
