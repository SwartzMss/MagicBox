use std::sync::Arc;

use crate::providers::TranslateProvider;

#[derive(Clone)]
pub struct AppState {
    pub provider: Arc<dyn TranslateProvider>,
    pub cache: moka::future::Cache<String, String>,
}

impl AppState {
    pub fn new(
        provider: Arc<dyn TranslateProvider>,
        cache: moka::future::Cache<String, String>,
    ) -> Self {
        Self { provider, cache }
    }
}

pub fn build_provider() -> anyhow::Result<Arc<dyn TranslateProvider>> {
    let which = std::env::var("TRANSLATE_PROVIDER").unwrap_or_else(|_| "deepseek".to_string());
    match which.as_str() {
        "deepseek" => {
            let provider = crate::providers::deepseek::from_env()
                .ok_or_else(|| anyhow::anyhow!("missing DEEPSEEK_API_KEY"))?;
            Ok(Arc::new(provider))
        }
        other => Err(anyhow::anyhow!(format!("unsupported provider: {}", other))),
    }
}

pub fn build_cache() -> moka::future::Cache<String, String> {
    let ttl_secs: u64 = std::env::var("CACHE_TTL_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(300);
    let max_entries: u64 = std::env::var("CACHE_MAX_ENTRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10_000);
    moka::future::Cache::builder()
        .time_to_live(std::time::Duration::from_secs(ttl_secs))
        .max_capacity(max_entries)
        .build()
}
