mod config;
mod device;
mod ffmpeg;
mod lineup;
mod m3u;

use actix_web::{
    get,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use figment::{
    providers::{Env, Format, Serialized, Yaml},
    Figment,
};
use futures::TryStreamExt;
use lineup::LineupChannel;
use m3u::Playlist;
use url::Url;

use crate::{
    config::Config,
    lineup::{convert_playlist_to_lineup, get_url_for_stream_id},
};

#[get("/discover.json")]
async fn discover(config: Data<Config>) -> impl Responder {
    let device = device::Device::from(config.get_ref().clone());
    web::Json(device)
}

#[get("/lineup_status.json")]
async fn get_lineup_status() -> impl Responder {
    web::Json(lineup::LineupStatus::default())
}

#[get("/lineup.json")]
async fn get_lineup(config: Data<Config>, playlist: Data<Playlist>) -> impl Responder {
    let base_url = config.base_url.clone().join("stream/").unwrap();

    let playlist_ref = playlist.get_ref();
    let lineup: Vec<LineupChannel> = convert_playlist_to_lineup(playlist_ref, &base_url);
    web::Json(lineup)
}

#[get("/stream/{stream_id}")]
async fn get_stream(
    playlist: Data<Playlist>,
    stream_id: web::Path<String>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let url = get_url_for_stream_id(&playlist, &stream_id);

    if url.is_none() {
        return Ok(HttpResponse::NotFound().body("Stream not found"));
    }

    let url: Url = url.unwrap().parse()?;
    let stream = ffmpeg::stream_from_media_url(&url)?;

    let response = HttpResponse::Ok()
        .content_type("video/mpeg; codecs=\"avc1.4D401E\"")
        .streaming(stream.map_err(actix_web::error::ErrorInternalServerError));

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter_module("nettopbox", log::LevelFilter::Info)
        .init();

    let config_file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| String::from("config.yaml"));

    log::info!(
        "Loading config from {} and NTB_ environment variables",
        config_file_path
    );

    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Yaml::file(config_file_path))
        .merge(Env::prefixed("NTB_"))
        .extract()
        .expect("Failed to load config");

    log::debug!("Config: {:?}", config);

    if config.iptv_url.is_none() {
        log::error!("No IPTV URL was provided. Set NTB_IPTV_URL or add iptv_url to config.yaml");
        panic!("No IPTV URL was provided");
    }

    let iptv_url = config.iptv_url.clone().unwrap();

    log::info!("Downloading m3u from {}...", iptv_url);
    let playlist: Playlist = reqwest::get(iptv_url)
        .await
        .expect("Failed to download m3u")
        .text()
        .await
        .expect("Failed to read m3u")
        .parse::<Playlist>()
        .expect("Failed to parse m3u");

    log::info!("Loaded {} channels", playlist.streams.len());

    // TODO is there anything to be gleaned by probing the streams?

    if playlist.streams.is_empty() {
        log::warn!("No channels were found");
    }

    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(playlist.clone()))
            .service(discover)
            .service(get_lineup_status)
            .service(get_lineup)
            .service(get_stream)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
