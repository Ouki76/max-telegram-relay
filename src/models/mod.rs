pub mod auth;
pub mod handshake;
pub mod message;
pub mod profile;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request<T> {
    pub ver: i64,
    pub cmd: i64,
    pub seq: i64,
    pub opcode: i64,
    pub payload: T,
}
