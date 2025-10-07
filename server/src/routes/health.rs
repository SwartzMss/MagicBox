use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::app::AppState;

#[derive(Serialize)]
struct HealthResp {
    ok: bool,
}

async fn handler() -> Json<HealthResp> {
    Json(HealthResp { ok: true })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/api/health", get(handler))
}
