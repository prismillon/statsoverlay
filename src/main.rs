use actix_web::{web, App, HttpResponse, HttpServer, Responder, get};
use serde::Deserialize;
use serde_json;
use std::collections::HashMap;
use actix_web::Result;

#[derive(Deserialize)]
struct MmrChanges {
    mmrDelta: i32,
    reason: String,
}

#[derive(Deserialize)]
struct PlayerDetails {
    mmr: i32,
    rank: String,
    mmrChanges: Option<Vec<MmrChanges>>,
}

const FONT: &[u8] = include_bytes!("../image/font.ttf");
const BACKGROUND: &[u8] = include_bytes!("../image/overlaybg.png");
const GRANDMASTER: &[u8] = include_bytes!("../image/grandmaster.png");
const MASTER: &[u8] = include_bytes!("../image/master.png");
const DIAMOND: &[u8] = include_bytes!("../image/diamond.png");
const RUBY: &[u8] = include_bytes!("../image/ruby.png");
const SAPPHIRE: &[u8] = include_bytes!("../image/sapphire.png");
const PLATINUM: &[u8] = include_bytes!("../image/platinum.png");
const GOLD: &[u8] = include_bytes!("../image/gold.png");
const SILVER: &[u8] = include_bytes!("../image/silver.png");
const BRONZE: &[u8] = include_bytes!("../image/bronze.png");
const IRON: &[u8] = include_bytes!("../image/iron.png");

const HTMLPAGE: &[u8] = include_bytes!("../mini.html");

async fn index(params: web::Query<HashMap<String, String>>) -> impl Responder {
    if params.get("name").is_none() {
        return HttpResponse::Ok().body("please add \"?name=<your_name_here>\" in the link")
    }
    HttpResponse::Ok().body(HTMLPAGE)
}

async fn stats_handler(params: web::Query<HashMap<String, String>>) -> impl Responder {
    let name = match params.get("name") {
        Some(x) => x,
        _ => return HttpResponse::BadRequest().finish()
    };
    let api_url = format!("https://www.mk8dx-lounge.com/api/player/details?name={}", name);
    match fetch_data(api_url).await {
        Ok(data) => {
            let mmr = &data.mmr;
            let rank = &data.rank;
            let mmr_delta = match &data.mmrChanges {
                Some(changes) if !changes.is_empty() && changes[0].reason == "Table" => &changes[0].mmrDelta,
                _ => &0,
            };

            let rank_image = get_rank_image(rank);

            let response_obj = serde_json::json!({
                "mmr": mmr,
                "rankImage": rank_image,
                "mmrDelta": mmr_delta,
            });

            HttpResponse::Ok().json(response_obj)
        }
        Err(_error) => {
            HttpResponse::Ok().json(serde_json::json!({
                "mmr": "Invalid Name",
                "rankImage": "",
                "mmrDelta": "",
            }))
        }
    }
}

async fn fetch_data(api_url: String) -> Result<PlayerDetails, reqwest::Error> {
    reqwest::get(&api_url).await?.json().await
}

fn get_rank_image(rank: &str) -> &str {
    match rank {
        "Grandmaster" => "./image/grandmaster.png",
        "Master" => "./image/master.png",
        "Diamond 1" | "Diamond 2" => "./image/diamond.png",
        "Ruby 1" | "Ruby 2" => "./image/ruby.png",
        "Sapphire 1" | "Sapphire 2" => "./image/sapphire.png",
        "Platinum 1" | "Platinum 2" => "./image/platinum.png",
        "Gold 1" | "Gold 2" => "./image/gold.png",
        "Silver 1" | "Silver 2" => "./image/silver.png",
        "Bronze 1" | "Bronze 2" => "./image/bronze.png",
        "Iron 1" | "Iron 2" => "./image/iron.png",
        _ => ""
    }
}

#[derive(Deserialize)]
struct Info {
    asset: String,
}

#[get("/image/{asset}")]
async fn assets_handler(info: web::Path<Info>) -> HttpResponse {
    match info.asset.as_str() {
        "font.ttf" => HttpResponse::Ok().content_type("font/ttf").body(FONT),
        "overlaybg.png" => HttpResponse::Ok().content_type("image/png").body(BACKGROUND),
        "grandmaster.png" => HttpResponse::Ok().content_type("image/png").body(GRANDMASTER),
        "master.png" => HttpResponse::Ok().content_type("image/png").body(MASTER),
        "diamond.png" => HttpResponse::Ok().content_type("image/png").body(DIAMOND),
        "ruby.png" => HttpResponse::Ok().content_type("image/png").body(RUBY),
        "sapphire.png" => HttpResponse::Ok().content_type("image/png").body(SAPPHIRE),
        "platinum.png" => HttpResponse::Ok().content_type("image/png").body(PLATINUM),
        "gold.png" => HttpResponse::Ok().content_type("image/png").body(GOLD),
        "silver.png" => HttpResponse::Ok().content_type("image/png").body(SILVER),
        "bronze.png" => HttpResponse::Ok().content_type("image/png").body(BRONZE),
        "iron.png" => HttpResponse::Ok().content_type("image/png").body(IRON),
        _ => HttpResponse::NotFound().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/stats").to(stats_handler))
            .service(web::resource("/").to(index))
            .service(assets_handler)
    })
    .bind("0.0.0.0:44994")?
    .run()
    .await
}
