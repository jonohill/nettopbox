use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub iptv_url: Option<Url>,
    pub base_url: Url,
    pub port: u16,
    pub tuner_count: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            iptv_url: None,
            base_url: "http://localhost:8080".parse().unwrap(),
            port: 8080,
            tuner_count: 10,
        }
    }
}
