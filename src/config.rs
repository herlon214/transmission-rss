use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub persistence: Persistence,
    pub transmission: Transmission,
    pub rss_list: Vec<RssList>,
    pub notification: Notification,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Persistence {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transmission {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RssList {
    pub title: String,
    pub url: String,
    pub filters: Vec<String>,
    pub download_dir: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub telegram: Option<TelegramNotification>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TelegramNotification {
    pub bot_token: String,
    pub chat_id: i64,
}