use axum::{routing::post, Router};

use crate::app::AppState;

mod json_format;
mod md5;
mod translate;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/tools/json/format", post(json_format::handle))
        .route("/api/tools/hash/md5", post(md5::handle))
        .route("/api/tools/translate", post(translate::handle))
}
