use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

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
#[serde(try_from = "RawTransmission")]
pub struct Transmission {
    pub url: String,
    pub username: String,
    pub password: String,
}

impl TryFrom<RawTransmission> for Transmission {
    type Error = std::io::Error;

    fn try_from(value: RawTransmission) -> Result<Self, Self::Error> {
        let password = match value.password {
            TransmissionPassword::Raw { password } => password,
            TransmissionPassword::File { password_file } => {
                read_to_string(password_file)?.trim().to_string()
            }
        };
        Ok(Transmission {
            url: value.url,
            username: value.username,
            password,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct RawTransmission {
    pub url: String,
    pub username: String,
    #[serde(flatten)]
    pub password: TransmissionPassword,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TransmissionPassword {
    Raw { password: String },
    File { password_file: String },
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
    pub telegram: Option<TelegramNotification>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(try_from = "RawTelegramNotification")]
pub struct TelegramNotification {
    pub bot_token: String,
    pub chat_id: i64,
}

impl TryFrom<RawTelegramNotification> for TelegramNotification {
    type Error = std::io::Error;

    fn try_from(value: RawTelegramNotification) -> Result<Self, Self::Error> {
        let bot_token = match value.bot_token {
            TelegramToken::Raw { bot_token } => bot_token,
            TelegramToken::File { bot_token_file } => {
                read_to_string(bot_token_file)?.trim().to_string()
            }
        };
        Ok(TelegramNotification {
            bot_token,
            chat_id: value.chat_id,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawTelegramNotification {
    #[serde(flatten)]
    pub bot_token: TelegramToken,
    pub chat_id: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum TelegramToken {
    Raw { bot_token: String },
    File { bot_token_file: String },
}
