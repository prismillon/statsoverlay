use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use env_logger::Env;
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

async fn index(params: web::Query<HashMap<String, String>>) -> impl Responder {
    if params.get("name").is_none() {
        return HttpResponse::Ok().body("please add \"?name=<your_name_here>\" in the link")
    }
    let html_content = "<!DOCTYPE html><html><head><title>prismillon stats overlay</title><style>@import url(https://fonts.cdnfonts.com/css/ds-digital?styles=18001);body{font-family:DS-Digital,monospace;display:flex;align-items:center;justify-content:center;flex-direction:column}#stats{display:flex;align-items:center;margin-top:20px}#stats img{width:100px;height:auto}#stats p{font-size:80px;margin-left:20px;background-color:rgba(0,0,0,.5);color:#fff;padding:10px;border-radius: 15px;}#stats span{font-size:40px;margin-left:10px}#stats span.red{color:#ff5858}#stats span.green{color:#6eff58}</style></head><body><div id='stats'></div><script>function updateStats(){const name=new URLSearchParams(window.location.search).get('name'); fetch('/api/stats?name=' + name) .then(response=> response.json()) .then(data=>{const statsDiv=document.getElementById('stats'); if (data.mmrDelta !=''){const mmrDelta=data.mmrDelta; statsDiv.innerHTML=` <img src='${data.rankImage}' alt='Rank Image'> <p>${data.mmr}<span class=\"${mmrDelta >=0 ? 'green' : 'red'}\">${mmrDelta >=0 ? '+' : ''}${mmrDelta}</span></p>`;}else{statsDiv.innerHTML=` <img src='${data.rankImage}' alt='Rank Image'> <p>${data.mmr}</p>`;}}) .catch(error=>{console.error('Error:', error);});}updateStats(); setInterval(updateStats, 60000);</script></body></html>";
    HttpResponse::Ok().body(html_content)
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
        Err(error) => {
            eprintln!("Error: {}", error);
            HttpResponse::InternalServerError().finish()
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/stats").to(stats_handler))
            .service(web::resource("/").to(index))
            .service(Files::new("/image", "./image"))
            .wrap(Logger::default())
            .wrap(Logger::new("%a"))
    })
    .bind("0.0.0.0:44994")?
    .run()
    .await
}
