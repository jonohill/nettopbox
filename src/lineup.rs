use base64::engine::general_purpose::URL_SAFE_NO_PAD as base64_url;
use base64::Engine;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::m3u::Playlist;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LineupStatus {
    scan_in_progress: u8,
    scan_possible: u8,
    source: String,
    source_list: Vec<String>,
}

impl Default for LineupStatus {
    fn default() -> Self {
        LineupStatus {
            scan_in_progress: 0,
            scan_possible: 1,
            source: String::from("Cable"),
            source_list: vec![String::from("Cable")],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LineupChannel {
    guide_number: String,
    guide_name: String,
    #[serde(rename = "URL")]
    url: String,
}

pub fn convert_playlist_to_lineup(playlist: &Playlist, base_url: &Url) -> Vec<LineupChannel> {
    playlist
        .streams
        .iter()
        .map(|stream| {
            let stream = stream.clone();

            let path = stream
                .id
                .unwrap_or_else(|| base64_url.encode(stream.url.as_bytes()));

            LineupChannel {
                guide_number: stream.channel_number.or(stream.epg_id).unwrap_or_default(),
                guide_name: stream.name.unwrap_or_default(),
                // TODO url encoding?
                url: base_url.join(&path).unwrap().to_string(),
            }
        })
        .collect()
}

pub fn get_url_for_stream_id(playlist: &Playlist, stream_id: &str) -> Option<String> {
    // If no id was available, the stream_id will be the base64 encoded stream url itself

    base64_url
        .decode(stream_id.as_bytes())
        .map(|bytes| String::from_utf8(bytes).ok())
        .ok()
        .flatten()
        .or_else(|| {
            playlist
                .streams
                .iter()
                .find(|stream| {
                    stream
                        .id
                        .as_ref()
                        .map(|id| id == stream_id)
                        .unwrap_or(false)
                })
                .map(|stream| stream.url.clone())
        })
}
