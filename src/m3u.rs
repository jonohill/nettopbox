use regex::Regex;

use itertools::{self, Itertools};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct Stream {
    pub url: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub channel_number: Option<String>,
    pub logo: Option<String>,
    pub group: Option<String>,
    pub epg_id: Option<String>,
    /// Anything found but not captured by the other fields
    pub metadata: HashMap<String, String>,
}

impl Stream {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Playlist {
    pub streams: Vec<Stream>,
}

impl FromStr for Playlist {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // matches e.g some-tag="a value"
        let re_tag_value = Regex::new(r#"(\S+)=\"([^"]+)\""#).unwrap();
        // matches end of line after comma, e.g. , some stuff
        let re_description = Regex::new(r#"\s*,\s*(.+)\s*$"#).unwrap();

        let streams = s
            .lines()
            .map(|line| line.trim().to_string())
            .coalesce(|prev, line| {
                if prev.starts_with("#EXT") && !line.starts_with("#EXT") {
                    Ok(format!("{}\n{}", prev, line))
                } else {
                    Err((prev, line))
                }
            })
            .filter_map(|stream_data| {
                let mut lines = stream_data.lines();

                lines.next().and_then(|line| {
                    if line.starts_with("#EXTINF") {
                        lines.next().map(|url| {
                            let mut stream = Stream::new(url);

                            let captures = re_tag_value.captures_iter(line).map(|c| c.extract());
                            for (_, [k, v]) in captures {
                                match k.to_lowercase().as_str() {
                                    "channel-id" | "tvg-id" => stream.id = Some(v.to_string()),
                                    "tvg-epgid" => stream.epg_id = Some(v.to_string()),
                                    "tvg-name" => stream.name = Some(v.to_string()),
                                    "tvg-chno" => stream.channel_number = Some(v.to_string()),
                                    "tvg-logo" => stream.logo = Some(v.to_string()),
                                    "tvg-group" | "group-title" => {
                                        stream.group = Some(v.to_string())
                                    }
                                    "tvg-url" => stream.url = v.to_string(),
                                    _ => {
                                        stream.metadata.insert(k.to_string(), v.to_string());
                                    }
                                }
                            }

                            if let Some(capture) = re_description.captures(line) {
                                stream.description = Some(capture[1].to_string());
                                if stream.name.is_none() {
                                    stream.name = stream.description.clone();
                                }
                            }

                            if let Some(description) = stream.description.clone() {
                                if stream.name.is_none() {
                                    stream.name = Some(description);
                                }
                            }

                            if let Some(id) = stream.id.clone() {
                                if stream.epg_id.is_none() {
                                    stream.epg_id = Some(id);
                                }
                            }

                            stream
                        })
                    } else {
                        None
                    }
                })
            })
            .collect();

        Ok(Self { streams })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let data = r#"
            #EXTM3U x-tvg-url="https://i.mjh.nz/nz/epg.xml.gz"

            #EXTINF:-1 channel-id="mjh-tvnz-1" tvg-id="mjh-tvnz-1" tvg-logo="https://i.mjh.nz/.images/tvnz-1.png" tvg-chno="1" group-title="Nz" , TVNZ 1
            https://i.mjh.nz/tvnz-1.m3u8

            #EXTINF:-1 channel-id="mjh-tvnz-2" tvg-id="mjh-tvnz-2" tvg-logo="https://i.mjh.nz/.images/tvnz-2.png" tvg-chno="2" group-title="Nz" , TVNZ 2
            https://i.mjh.nz/tvnz-2.m3u8

            #EXTINF:-1 channel-id="mjh-three" tvg-id="mjh-three" tvg-logo="https://i.mjh.nz/.images/three.png" tvg-chno="3" group-title="Nz" , Three
            https://i.mjh.nz/three.m3u8

            #EXTINF:-1 channel-id="mjh-bravo" tvg-id="mjh-bravo" tvg-logo="https://i.mjh.nz/.images/bravo.png" tvg-chno="4" group-title="Nz" , Bravo
            https://i.mjh.nz/bravo.m3u8

            #EXTINF:-1 channel-id="mjh-maori-tv" tvg-id="mjh-maori-tv" tvg-logo="https://i.mjh.nz/.images/maori-tv.png" tvg-chno="5" group-title="Nz" , Whakaata Māori
            https://i.mjh.nz/maori-tv.m3u8

            #EXTINF:-1 channel-id="mjh-tvnz-duke" tvg-id="mjh-tvnz-duke" tvg-logo="https://i.mjh.nz/.images/tvnz-duke.png" tvg-chno="6" group-title="Nz" , DUKE
            https://i.mjh.nz/tvnz-duke.m3u8

            #EXTINF:-1 channel-id="mjh-eden" tvg-id="mjh-eden" tvg-logo="https://i.mjh.nz/.images/eden.png" tvg-chno="8" group-title="Nz" , eden
            https://i.mjh.nz/eden.m3u8
        "#;

        let playlist: Playlist = data.parse().unwrap();

        assert_eq!(playlist.streams.len(), 7);
        assert_eq!(
            playlist.streams[0],
            Stream {
                url: "https://i.mjh.nz/tvnz-1.m3u8".to_string(),
                id: Some("mjh-tvnz-1".to_string()),
                name: Some("TVNZ 1".to_string()),
                description: Some("TVNZ 1".to_string()),
                channel_number: Some("1".to_string()),
                logo: Some("https://i.mjh.nz/.images/tvnz-1.png".to_string()),
                group: Some("Nz".to_string()),
                epg_id: Some("mjh-tvnz-1".to_string()),
                metadata: HashMap::new(),
            }
        );
    }
}
