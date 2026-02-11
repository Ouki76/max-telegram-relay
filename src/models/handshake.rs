use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub device_type: String,
    pub locale: String,
    pub device_locale: String,
    #[serde(rename = "osVersion")]
    pub device_version: String,
    pub device_name: String,
    pub header_user_agent: String,
    pub app_version: String,
    pub screen: String,
    pub timezone: String,
}

impl Default for UserAgent {
    fn default() -> Self {
        Self {
            device_type: "WEB".to_string(),
            locale: "ru".to_string(),
            device_locale: "ru".to_string(),
            device_version: "Windows".to_string(),
            device_name: "Chrome".to_string(),
            header_user_agent: concat!(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) ",
                "AppleWebKit/537.36 (KHTML, like Gecko) ",
                "Chrome/142.0.0.0 Safari/537.36"
            )
            .to_string(),
            app_version: "26.2.2".to_string(),
            screen: "1080x1920 1.0x".to_string(),
            timezone: chrono::Local::now().format("%z").to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub user_agent: UserAgent,
    pub device_id: String,
}
