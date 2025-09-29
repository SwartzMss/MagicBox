use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tracing::{error, info};
use std::sync::Arc;

mod providers;
use providers::{Lang, TranslateProvider};
use providers::deepseek;

#[derive(Clone)]
struct AppState {
    provider: Arc<dyn TranslateProvider>,
    cache: moka::future::Cache<String, String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(format!("{}", log_level))
        .with_target(false)
        .compact()
        .init();

    let provider = build_provider()?;
    let cache = build_cache();
    let state = AppState { provider, cache };
    let app = build_router(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(18080);

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();
    info!(%addr, "MagicBox server starting");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await?;
    Ok(())
}

fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::permissive();

    Router::new()
        .route("/api/health", get(health))
        .route("/api/tools/json/format", post(json_format))
        .route("/api/tools/hash/md5", post(hash_md5))
        .route("/api/tools/translate", post(translate))
        .with_state(state)
        .layer(cors)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
    info!("shutdown signal received");
    tokio::time::sleep(Duration::from_millis(100)).await;
}

// =============== Common error model ===============

#[derive(Debug, Serialize)]
struct ApiErrorBody {
    code: &'static str,
    message: String,
    details: Option<serde_json::Value>,
}

#[derive(thiserror::Error, Debug)]
enum ApiError {
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("Internal: {0}")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, msg) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, "BadRequest", m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal", m),
        };
        let body = ApiErrorBody {
            code,
            message: msg,
            details: None,
        };
        (status, Json(body)).into_response()
    }
}

type ApiResult<T> = Result<Json<T>, ApiError>;

// =============== Handlers ===============

#[derive(Serialize)]
struct HealthResp {
    ok: bool,
}

async fn health() -> Json<HealthResp> {
    Json(HealthResp { ok: true })
}

// ----- JSON format -----

#[derive(Deserialize)]
struct JsonFormatReq {
    json: String,
    #[serde(default = "default_indent")]
    indent: u8,
}

#[derive(Serialize)]
struct JsonFormatResp {
    formatted: String,
}

fn default_indent() -> u8 { 2 }

async fn json_format(Json(req): Json<JsonFormatReq>) -> ApiResult<JsonFormatResp> {
    let value: serde_json::Value = serde_json::from_str(&req.json)
        .map_err(|e| ApiError::BadRequest(format!("invalid json: {}", e)))?;
    let formatted = if req.indent == 0 {
        serde_json::to_string(&value).unwrap_or_else(|_| req.json.clone())
    } else {
        let indent = " ".repeat(req.indent as usize);
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        value.serialize(&mut ser).map_err(|e| ApiError::Internal(e.to_string()))?;
        String::from_utf8(buf).unwrap_or_default()
    };
    Ok(Json(JsonFormatResp { formatted }))
}

// ----- MD5 -----

#[derive(Deserialize)]
struct Md5Req { text: String }

#[derive(Serialize)]
struct Md5Resp { md5: String }

async fn hash_md5(Json(req): Json<Md5Req>) -> ApiResult<Md5Resp> {
    let digest = md5::compute(req.text.as_bytes());
    Ok(Json(Md5Resp { md5: format!("{:x}", digest) }))
}

// ----- Translate (with auto-detect inside) -----

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TranslateReq {
    text: String,
    #[serde(default)]
    source_lang: Option<String>, // "zh" | "en" | "auto"
    #[serde(default)]
    target_lang: Option<String>, // "zh" | "en"
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TranslateResp {
    detected_lang: String, // zh | en
    target_lang: String,   // zh | en
    translation: String,
    provider: &'static str,
    cached: bool,
}

async fn translate(State(state): State<AppState>, Json(req): Json<TranslateReq>) -> ApiResult<TranslateResp> {
    let text = req.text.trim();
    if text.is_empty() { return Err(ApiError::BadRequest("empty text".into())); }

    let detected = match req.source_lang.as_deref() {
        Some("zh") => Lang::Zh,
        Some("en") => Lang::En,
        _ => detect_lang(text),
    };

    let target = match req.target_lang.as_deref() {
        Some("zh") => Lang::Zh,
        Some("en") => Lang::En,
        _ => match detected { Lang::Zh => Lang::En, Lang::En => Lang::Zh },
    };

    // Cache key: md5(provider + source + target + text)
    let key = {
        use std::fmt::Write as _;
        let mut s = String::new();
        let _ = write!(
            &mut s,
            "{}|{}|{}|{}",
            state.provider.name(),
            detected.as_str(),
            target.as_str(),
            text
        );
        let digest = md5::compute(s.as_bytes());
        format!("{:x}", digest)
    };

    let mut cached = false;
    let translation = if let Some(v) = state.cache.get(&key).await {
        cached = true;
        v
    } else {
        match state.provider.translate(text, Some(detected), target).await {
            Ok(t) => {
                state.cache.insert(key.clone(), t.clone()).await;
                t
            }
            Err(e) => {
                error!(error = ?e, "translate failed");
                return Err(ApiError::Internal("translation failed".into()));
            }
        }
    };

    let resp = TranslateResp {
        detected_lang: detected.as_str().to_string(),
        target_lang: target.as_str().to_string(),
        translation,
        provider: state.provider.name(),
        cached,
    };
    Ok(Json(resp))
}

fn detect_lang(s: &str) -> Lang {
    // Heuristic: presence of CJK characters => zh, else en
    if s.chars().any(is_cjk) { Lang::Zh } else { Lang::En }
}

fn is_cjk(c: char) -> bool {
    matches!(
        c as u32,
        0x4E00..=0x9FFF   // CJK Unified Ideographs
        | 0x3400..=0x4DBF // CJK Unified Ideographs Extension A
        | 0xF900..=0xFAFF // CJK Compatibility Ideographs
        | 0x2E80..=0x2EFF // CJK Radicals Supplement
        | 0x3000..=0x303F // CJK Symbols and Punctuation
        | 0x31C0..=0x31EF // CJK Strokes
        | 0x2F00..=0x2FDF // Kangxi Radicals
        | 0x2FF0..=0x2FFF // Ideographic Description Characters
    )
}

fn build_provider() -> anyhow::Result<Arc<dyn TranslateProvider>> {
    let which = std::env::var("TRANSLATE_PROVIDER").unwrap_or_else(|_| "deepseek".to_string());
    match which.as_str() {
        "deepseek" => {
            let p = deepseek::from_env().ok_or_else(|| anyhow::anyhow!("missing DEEPSEEK_API_KEY"))?;
            Ok(Arc::new(p))
        }
        other => Err(anyhow::anyhow!(format!("unsupported provider: {}", other))),
    }
}

fn build_cache() -> moka::future::Cache<String, String> {
    let ttl_secs: u64 = std::env::var("CACHE_TTL_SECONDS").ok().and_then(|v| v.parse().ok()).unwrap_or(300);
    let max_entries: u64 = std::env::var("CACHE_MAX_ENTRIES").ok().and_then(|v| v.parse().ok()).unwrap_or(10_000);
    moka::future::Cache::builder()
        .time_to_live(std::time::Duration::from_secs(ttl_secs))
        .max_capacity(max_entries)
        .build()
}
