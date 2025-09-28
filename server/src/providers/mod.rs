use async_trait::async_trait;

#[derive(Copy, Clone, Debug)]
pub enum Lang { Zh, En }

impl Lang {
    pub fn as_str(&self) -> &'static str { match self { Lang::Zh => "zh", Lang::En => "en" } }
}

#[derive(thiserror::Error, Debug)]
pub enum ProviderError {
    #[error("http: {0}")]
    Http(String),
    #[error("bad_response: {0}")]
    BadResponse(String),
}

#[async_trait]
pub trait TranslateProvider: Send + Sync {
    async fn translate(
        &self,
        text: &str,
        source: Option<Lang>,
        target: Lang,
    ) -> Result<String, ProviderError>;
    fn name(&self) -> &'static str { "deepseek" }
}

pub mod deepseek;

