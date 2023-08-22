use serde::{Deserialize, Serialize};

use crate::config::Config;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Device {
    friendly_name: String,
    manufacturer: String,
    #[serde(rename = "ManufacturerURL")]
    manufacturer_url: String,
    model_number: String,
    firmware_name: String,
    tuner_count: u8,
    firmware_version: String,
    #[serde(rename = "DeviceID")]
    device_id: String,
    device_auth: String,
    #[serde(rename = "BaseURL")]
    base_url: String,
    #[serde(rename = "LineupURL")]
    lineup_url: String,
}

impl Default for Device {
    fn default() -> Self {
        // Not sure what Plex considers acceptable here
        Device {
            friendly_name: String::from("HDHomeRun"),
            manufacturer: String::from("Silicondust"),
            manufacturer_url: String::from("https://www.silicondust.com/"),
            model_number: String::from("HDTC-2US"),
            firmware_name: String::from("hdhomerun3_atsc"),
            tuner_count: 2,
            firmware_version: String::from("20150826"),
            device_id: String::from("bb0df8cf-0cbe-4467-9941-2c0826afb3ac"),
            device_auth: String::from("none"),
            base_url: String::from("http://localhost:8080"),
            lineup_url: String::from("http://localhost:8080/lineup.json"),
        }
    }
}

impl From<Config> for Device {
    fn from(config: Config) -> Self {
        let device_id = Uuid::new_v5(&Uuid::NAMESPACE_URL, config.base_url.as_str().as_bytes());

        Device {
            device_id: device_id.to_string(),
            base_url: config.base_url.to_string(),
            lineup_url: config.base_url.join("lineup.json").unwrap().to_string(),
            tuner_count: config.tuner_count as u8,
            ..Default::default()
        }
    }
}
