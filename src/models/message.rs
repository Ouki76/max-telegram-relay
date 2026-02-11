use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: i64,
    pub id: String,
    pub time: i64,
    pub text: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub attaches: Vec<()>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedMessage {
    pub chat_id: i64,
    pub unread: i64,
    pub message: Message,
    pub ttl: bool,
    pub mark: i64,
    pub prev_message_id: String,
}
