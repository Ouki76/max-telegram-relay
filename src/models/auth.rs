use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub interactive: bool,
    pub token: String,
    pub chats_count: i64,
    pub chats_sync: i64,
    pub contacts_sync: i64,
    pub presence_sync: i64,
    pub drafts_sync: i64,
}
