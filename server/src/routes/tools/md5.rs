use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiResult;

#[derive(Deserialize)]
pub(super) struct Md5Req {
    text: String,
}

#[derive(Serialize)]
pub(super) struct Md5Resp {
    md5: String,
}

pub async fn handle(Json(req): Json<Md5Req>) -> ApiResult<Md5Resp> {
    let digest = md5::compute(req.text.as_bytes());
    Ok(Json(Md5Resp {
        md5: format!("{:x}", digest),
    }))
}
