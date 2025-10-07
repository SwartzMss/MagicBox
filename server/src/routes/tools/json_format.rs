use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};

#[derive(Deserialize)]
pub(super) struct JsonFormatReq {
    json: String,
    #[serde(default = "default_indent")]
    indent: u8,
}

#[derive(Serialize)]
pub(super) struct JsonFormatResp {
    formatted: String,
}

fn default_indent() -> u8 {
    2
}

pub async fn handle(Json(req): Json<JsonFormatReq>) -> ApiResult<JsonFormatResp> {
    let value: serde_json::Value = serde_json::from_str(&req.json)
        .map_err(|e| ApiError::BadRequest(format!("invalid json: {}", e)))?;
    let formatted = if req.indent == 0 {
        serde_json::to_string(&value).unwrap_or_else(|_| req.json.clone())
    } else {
        let indent = " ".repeat(req.indent as usize);
        let formatter = serde_json::ser::PrettyFormatter::with_indent(indent.as_bytes());
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
        value
            .serialize(&mut ser)
            .map_err(|e| ApiError::Internal(e.to_string()))?;
        String::from_utf8(buf).unwrap_or_default()
    };
    Ok(Json(JsonFormatResp { formatted }))
}
