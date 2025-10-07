use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiErrorBody {
    pub code: &'static str,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("BadRequest: {0}")]
    BadRequest(String),
    #[error("Internal: {0}")]
    Internal(String),
    #[error("NotFound: {0}")]
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::BadRequest(m) => (StatusCode::BAD_REQUEST, "BadRequest", m),
            ApiError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal", m),
            ApiError::NotFound(m) => (StatusCode::NOT_FOUND, "NotFound", m),
        };

        let body = ApiErrorBody {
            code,
            message,
            details: None,
        };
        (status, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<Json<T>, ApiError>;
