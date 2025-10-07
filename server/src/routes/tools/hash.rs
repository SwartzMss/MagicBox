use std::str::FromStr;

use axum::Json;
use serde::{Deserialize, Serialize};
use sha2::{Digest as ShaDigest, Sha256};

use crate::error::{ApiError, ApiResult};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HashReq {
    text: String,
    #[serde(default = "default_algorithm")]
    algorithm: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HashResp {
    algorithm: String,
    digest: String,
}

pub async fn handle(Json(req): Json<HashReq>) -> ApiResult<HashResp> {
    let algorithm = req.algorithm.trim();
    let algo = HashAlgorithm::from_str(algorithm)
        .map_err(|_| ApiError::BadRequest(format!("unsupported algorithm: {}", algorithm)))?;

    let digest = compute_digest(algo, &req.text);

    Ok(Json(HashResp {
        algorithm: algo.as_str().to_string(),
        digest,
    }))
}

fn compute_digest(algo: HashAlgorithm, input: &str) -> String {
    match algo {
        HashAlgorithm::Md5 => format!("{:x}", md5::compute(input.as_bytes())),
        HashAlgorithm::Sha256 => {
            let digest = Sha256::digest(input.as_bytes());
            format!("{:x}", digest)
        }
    }
}

fn default_algorithm() -> String {
    "md5".to_string()
}

#[derive(Copy, Clone)]
enum HashAlgorithm {
    Md5,
    Sha256,
}

impl HashAlgorithm {
    fn as_str(&self) -> &'static str {
        match self {
            HashAlgorithm::Md5 => "md5",
            HashAlgorithm::Sha256 => "sha256",
        }
    }
}

impl FromStr for HashAlgorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "md5" => Ok(HashAlgorithm::Md5),
            "sha256" => Ok(HashAlgorithm::Sha256),
            _ => Err(()),
        }
    }
}
