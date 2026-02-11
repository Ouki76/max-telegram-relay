use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub profile: ProfileData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub contact: Contact,
}
