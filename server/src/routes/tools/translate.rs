use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::{
    app::AppState,
    error::{ApiError, ApiResult},
    providers::Lang,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TranslateReq {
    text: String,
    #[serde(default)]
    source_lang: Option<String>,
    #[serde(default)]
    target_lang: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TranslateResp {
    detected_lang: String,
    target_lang: String,
    translation: String,
    provider: &'static str,
    cached: bool,
}

pub async fn handle(
    State(state): State<AppState>,
    Json(req): Json<TranslateReq>,
) -> ApiResult<TranslateResp> {
    let text = req.text.trim();
    if text.is_empty() {
        return Err(ApiError::BadRequest("empty text".into()));
    }

    let detected = match req.source_lang.as_deref() {
        Some("zh") => Lang::Zh,
        Some("en") => Lang::En,
        _ => detect_lang(text),
    };

    let target = match req.target_lang.as_deref() {
        Some("zh") => Lang::Zh,
        Some("en") => Lang::En,
        _ => match detected {
            Lang::Zh => Lang::En,
            Lang::En => Lang::Zh,
        },
    };

    let cache_key = cache_key(
        state.provider.name(),
        detected.as_str(),
        target.as_str(),
        text,
    );

    let mut cached = false;
    let translation = if let Some(value) = state.cache.get(&cache_key).await {
        cached = true;
        value
    } else {
        match state.provider.translate(text, Some(detected), target).await {
            Ok(result) => {
                state.cache.insert(cache_key.clone(), result.clone()).await;
                result
            }
            Err(err) => {
                error!(error = ?err, "translate failed");
                return Err(ApiError::Internal("translation failed".into()));
            }
        }
    };

    Ok(Json(TranslateResp {
        detected_lang: detected.as_str().to_string(),
        target_lang: target.as_str().to_string(),
        translation,
        provider: state.provider.name(),
        cached,
    }))
}

fn detect_lang(text: &str) -> Lang {
    if text.chars().any(is_cjk) {
        Lang::Zh
    } else {
        Lang::En
    }
}

fn is_cjk(c: char) -> bool {
    matches!(
        c as u32,
        0x4E00..=0x9FFF
            | 0x3400..=0x4DBF
            | 0xF900..=0xFAFF
            | 0x2E80..=0x2EFF
            | 0x3000..=0x303F
            | 0x31C0..=0x31EF
            | 0x2F00..=0x2FDF
            | 0x2FF0..=0x2FFF
    )
}

fn cache_key(provider: &str, source: &str, target: &str, text: &str) -> String {
    use std::fmt::Write as _;

    let mut value = String::new();
    let _ = write!(&mut value, "{}|{}|{}|{}", provider, source, target, text);
    let digest = md5::compute(value.as_bytes());
    format!("{:x}", digest)
}
