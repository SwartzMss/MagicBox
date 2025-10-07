use axum::Json;
use base64::Engine;
use serde::{Deserialize, Serialize};

use crate::error::{ApiError, ApiResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Base64Req {
    text: String,
    #[serde(default = "default_action")]
    action: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Base64Resp {
    action: String,
    result: String,
}

pub async fn handle(Json(req): Json<Base64Req>) -> ApiResult<Base64Resp> {
    let action = Base64Action::from(req.action.as_str());

    match action {
        Base64Action::Encode => {
            let result = base64::engine::general_purpose::STANDARD.encode(req.text.as_bytes());
            Ok(Json(Base64Resp {
                action: action.as_str().to_string(),
                result,
            }))
        }
        Base64Action::Decode => {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(req.text.as_bytes())
                .map_err(|e| ApiError::BadRequest(format!("invalid base64: {}", e)))?;
            let result = String::from_utf8(decoded)
                .map_err(|_| ApiError::BadRequest("decoded bytes are not valid UTF-8".into()))?;
            Ok(Json(Base64Resp {
                action: action.as_str().to_string(),
                result,
            }))
        }
    }
}

fn default_action() -> String {
    "encode".to_string()
}

#[derive(Copy, Clone)]
enum Base64Action {
    Encode,
    Decode,
}

impl Base64Action {
    fn as_str(&self) -> &'static str {
        match self {
            Base64Action::Encode => "encode",
            Base64Action::Decode => "decode",
        }
    }
}

impl From<&str> for Base64Action {
    fn from(value: &str) -> Self {
        match value.to_ascii_lowercase().as_str() {
            "decode" => Base64Action::Decode,
            _ => Base64Action::Encode,
        }
    }
}
