use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
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
    let html_content = "<!DOCTYPE html><html><head><title>prismillon stats overlay</title><style>@font-face{font-family:testFont;src:url(image/font.ttf)}body{font-family:testFont;display:flex;align-items:center;justify-content:center;flex-direction:column;text-shadow:2px 2px 2px #000}#stats{display:flex;align-items:center;margin-top:20px}#stats img{width:60px;height:auto;margin-left:10px;margin-right:10px;position:relative;top:7px}#stats p{font-size:70px;background-image:url(image/overlaybg.png);background-repeat:no-repeat;background-size:100% 100%;color:#fff;padding:20px;padding-right:60px;padding-left:30px;border-radius:15px}#stats span{font-size:30px;margin-left:10px}#stats span.red{color:#ff5858}#stats span.green{color:#6eff58}#stats span.disabled{color:transparent;display:none}</style></head><body><div id=\"stats\"></div><script>localStorage.removeItem('mmr');
    function animateMmrChange(a,e,m,n,t,r,s){duration=3e3,null==a&&(a=e-n);let l=parseInt(a);e=parseInt(e);let i=setInterval(()=>{l<e?l+=1:l-=1,l===e?(clearInterval(i),m.innerHTML=` <p><img src='${s}' alt='Rank Image'>${e}<span class=\"${t}\">${r}${n}</span></p>`):m.innerHTML=` <p><img src='${s}' alt='Rank Image'>${l}<span class=\"${t}\">${r}${n}</span></p>`},duration/Math.abs(e-l))}function updateStats(){let a=new URLSearchParams(window.location.search).get(\"name\"),e=localStorage.getItem(\"mmr\");fetch(\"/api/stats?name=\"+a).then(a=>a.json()).then(a=>{let m=document.getElementById(\"stats\"),n=a.mmrDelta,t=n>0?\"green\":n<0?\"red\":\"disabled\";console.log(n,t,e,a.mmr),e!=a.mmr&&\"Invalid Name\"!=a.mmr?(animateMmrChange(e,a.mmr,m,n,t,n>0?\"+\":\"\",a.rankImage),localStorage.setItem(\"mmr\",a.mmr)):\"Invalid Name\"===a.mmr&&(m.innerHTML=\" <img src='' alt='Rank Image'> <p>Invalid name</p>\")}).catch(a=>{console.error(\"Error:\",a)})}updateStats(),setInterval(updateStats,6e4);</script></body></html>";
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/api/stats").to(stats_handler))
            .service(web::resource("/").to(index))
            .service(Files::new("/image", "./image"))
    })
    .bind("0.0.0.0:44994")?
    .run()
    .await
}
