use axum::{routing::post, Router};

use crate::app::AppState;

mod base64;
mod hash;
mod json_format;
mod timestamp;
mod translate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/tools/json/format", post(json_format::handle))
        .route("/api/tools/hash", post(hash::handle))
        .route("/api/tools/base64", post(base64::handle))
        .route("/api/tools/timestamp", post(timestamp::handle))
        .route("/api/tools/translate", post(translate::handle))
}
