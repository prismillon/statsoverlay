use axum::{
    extract::{Query, State},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};
use tracing::{error, info};

#[derive(Debug, Deserialize)]
struct PlayerQuery {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UpstreamPlayerData {
    mmr: Option<i64>,
    rank: String,
    #[serde(default)]
    mmr_changes: Vec<MmrChange>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MmrChange {
    mmr_delta: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct PlayerData {
    mmr: i64,
    rank: String,
    rank_icon_url: Option<String>,
    diff: String,
    #[serde(rename = "mod")]
    mod_class: String,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

fn rank_to_url(rank: &str) -> Option<&'static str> {
    match rank.to_lowercase().as_str() {
        "iron" => Some("/static/image/iron.png"),
        "bronze" => Some("/static/image/bronze.png"),
        "silver" => Some("/static/image/silver.png"),
        "gold" => Some("/static/image/gold.png"),
        "platinum" => Some("/static/image/platinum.png"),
        "sapphire" => Some("/static/image/sapphire.png"),
        "ruby" => Some("/static/image/ruby.png"),
        "diamond" => Some("/static/image/diamond.png"),
        "master" => Some("/static/image/master.png"),
        "grand master" | "grandmaster" => Some("/static/image/grandmaster.png"),
        _ => None,
    }
}

#[derive(Clone)]
struct AppState {
    client: reqwest::Client,
    cache: Arc<tokio::sync::RwLock<HashMap<String, (PlayerData, std::time::Instant)>>>,
}

impl AppState {
    fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("StatsOverlay/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    async fn get_cached_or_fetch(&self, player_name: &str) -> Result<PlayerData, String> {
        let cache_key = player_name.to_lowercase();
        let cache_duration = Duration::from_secs(60);

        {
            let cache = self.cache.read().await;
            if let Some((data, timestamp)) = cache.get(&cache_key) {
                if timestamp.elapsed() < cache_duration {
                    info!("Cache hit for player: {}", player_name);
                    return Ok(data.clone());
                }
            }
        }

        info!("Fetching data for player: {}", player_name);
        let url = format!(
            "https://lounge.mkcentral.com/api/player/details?name={}",
            urlencoding::encode(player_name),
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(format!("API error {status}: {error_text}"));
        }

        let upstream: UpstreamPlayerData = response
            .json()
            .await
            .map_err(|e| format!("JSON parse error: {e}"))?;

        let mmr = upstream.mmr.unwrap_or(0);
        let rank_clean = upstream
            .rank
            .replace(|c: char| c.is_ascii_digit(), "")
            .trim()
            .to_string();

        let rank_icon_url = rank_to_url(&rank_clean).map(|s| s.to_string());

        let table_events: Vec<&MmrChange> = upstream.mmr_changes.iter().collect();
        let last_delta = table_events.first().and_then(|c| c.mmr_delta).unwrap_or(0);

        let diff = if last_delta > 0 {
            format!("+{last_delta}")
        } else {
            format!("{last_delta}")
        };

        let mod_class = if last_delta > 0 {
            "modifier green".to_string()
        } else if last_delta < 0 {
            "modifier red".to_string()
        } else {
            "disabled".to_string()
        };

        let player_data = PlayerData {
            mmr,
            rank: rank_clean,
            rank_icon_url,
            diff,
            mod_class,
        };

        {
            let mut cache = self.cache.write().await;
            cache.insert(cache_key, (player_data.clone(), std::time::Instant::now()));
            cache.retain(|_, (_, timestamp)| timestamp.elapsed() < cache_duration * 2);
        }

        Ok(player_data)
    }
}

fn is_valid_player_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == ' ' || c == '.' || c == '_' || c == '-')
}

async fn api_player_details(
    Query(query): Query<PlayerQuery>,
    State(state): State<AppState>,
) -> Result<Json<PlayerData>, Response> {
    let player_name = match query.name {
        Some(name) if !name.trim().is_empty() => name.trim().to_string(),
        _ => {
            let error = ErrorResponse {
                error: "Player name is required".to_string(),
            };
            return Err((StatusCode::BAD_REQUEST, Json(error)).into_response());
        }
    };

    if !is_valid_player_name(&player_name) {
        let error = ErrorResponse {
            error: "Player name can only contain letters, numbers, spaces, and hyphens".to_string(),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error)).into_response());
    }

    match state.get_cached_or_fetch(&player_name).await {
        Ok(data) => Ok(Json(data)),
        Err(error_msg) => {
            error!("Failed to fetch player data: {}", error_msg);
            let error = ErrorResponse { error: error_msg };
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response())
        }
    }
}

async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "statsoverlay=info,tower_http=info".to_string()),
        )
        .init();

    let state = AppState::new();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET])
        .allow_headers(Any);

    let spa_fallback = ServeFile::new("dist/index.html");

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/player/details", get(api_player_details))
        .nest_service("/static", ServeDir::new("static"))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .fallback_service(ServeDir::new("dist").not_found_service(spa_fallback))
        .layer(cors)
        .with_state(state);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let addr = format!("0.0.0.0:{port}");
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
