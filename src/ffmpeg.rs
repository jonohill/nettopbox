use async_process::{Command, Stdio};
use async_std::io::BufReader;
use async_std::prelude::*;
use bytes::Bytes;
use futures::stream::StreamExt;
use std::error::Error;
use url::Url;

pub fn stream_from_media_url(
    url: &Url,
) -> Result<impl Stream<Item = Result<Bytes, Box<dyn Error + Send + Sync>>>, Box<dyn Error>> {
    let mut child = Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "warning",
            "-analyzeduration",
            "50M",
            "-probesize",
            "50M",
            "-i",
            url.as_str(),
            "-f",
            "mpegts",
            "-c",
            "copy",
            "-",
        ])
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("failed to execute ffmpeg: {}", e))?;

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    let stream = reader
        .bytes()
        .map(|b| b.map_err(|e| e.into()).map(|b| Bytes::from(vec![b])));

    Ok(stream)
}
