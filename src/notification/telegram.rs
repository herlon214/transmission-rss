use serde::{Serialize};
use std::{error::Error as StdError};

use reqwest::StatusCode;

use super::notification::Error;

#[derive(Serialize)]
struct Message {
    chat_id: i64,
    text: String,
}

pub struct Telegram {
    chat_id: i64,
    bot_token: String,
}

impl Telegram {
    pub fn new(bot_token: String, chat_id: i64) -> Self {
        Self {
            chat_id,
            bot_token,
        }
    }

    pub async fn send(&self, message: String) -> Result<(), Box<dyn StdError>> {
        let tel_msg = Message{chat_id: self.chat_id.to_owned(), text: message.to_owned()};
        let data = serde_json::to_string(&tel_msg)?;
        
        let client = reqwest::Client::new();
        let res = client.post(format!("https://api.telegram.org/bot{}/sendMessage", self.bot_token))
            .body(data)
            .send().await?;
        
        
        if res.status() != StatusCode::OK {
            if let Ok(val) = res.text().await {
                return Err(Box::new(Error::new(val)));
            }
        }

        Ok(())

    }
}
