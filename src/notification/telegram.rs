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
    base_url: String,
    chat_id: i64,
    bot_token: String,
}

impl Telegram {
    pub fn new(bot_token: String, chat_id: i64, base_url: String) -> Self {
        Self {
            chat_id,
            bot_token,
            base_url,
        }
    }

    pub async fn send(&self, message: String) -> Result<(), Box<dyn StdError>> {
        let tel_msg = Message{chat_id: self.chat_id.to_owned(), text: message.to_owned()};

        let client = reqwest::Client::new();
        let res = client.post(format!("{}/bot{}/sendMessage", self.base_url, self.bot_token))
            .json(&tel_msg)
            .send().await?;
        
        if res.status() != StatusCode::OK {
            if let Ok(val) = res.text().await {
                dbg!(&val);

                return Err(Box::new(Error::new(val)));
            }
        }

        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use crate::notification::telegram::Telegram;
    use httpmock::prelude::*;

    #[test]
    fn test_send() {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(POST)
                .path("/bot123token123/sendMessage");
            then.status(200);
        });
        let notifier = Telegram::new("123token123".into(), 123, server.url(""));

        let result = tokio_test::block_on(notifier.send("test message".into()));

        mock.assert();
        assert!(result.is_ok());
    }
}