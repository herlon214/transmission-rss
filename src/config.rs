use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub transmission: Transmission,
    pub rss_list: Vec<RssList>,
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
