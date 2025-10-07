use std::path::PathBuf;

use axum::{response::IntoResponse, Router};
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

use crate::{app::AppState, error::ApiError};

mod health;
pub mod tools;

async fn api_not_found() -> axum::response::Response {
    ApiError::NotFound("route not found".into()).into_response()
}

pub fn router(state: AppState) -> Router {
    let cors = CorsLayer::permissive();

    let api = Router::new()
        .merge(health::router())
        .merge(tools::router())
        .fallback(api_not_found);

    let static_dir_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../web/public");
    let index_file = static_dir_path.join("index.html");

    Router::new()
        .merge(api)
        .nest_service(
            "/",
            ServeDir::new(static_dir_path).fallback(ServeFile::new(index_file)),
        )
        .with_state(state)
        .layer(cors)
}
