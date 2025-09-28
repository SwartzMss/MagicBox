use super::{Lang, ProviderError, TranslateProvider};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub struct DeepseekProvider {
    client: reqwest::Client,
    base_url: String,
    api_key: String,
    model: String,
}

impl DeepseekProvider {
    pub fn new(base_url: String, api_key: String, model: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("reqwest client");
        Self { client, base_url, api_key, model }
    }
}

#[derive(Serialize)]
struct ChatReq<'a> {
    model: &'a str,
    temperature: f32,
    top_p: f32,
    stream: bool,
    messages: Vec<Message<'a>>,
}

#[derive(Serialize)]
struct Message<'a> { role: &'a str, content: String }

#[derive(Deserialize)]
struct ChatResp {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice { message: ChoiceMessage }

#[derive(Deserialize)]
struct ChoiceMessage { content: String }

#[async_trait]
impl TranslateProvider for DeepseekProvider {
    async fn translate(
        &self,
        text: &str,
        source: Option<Lang>,
        target: Lang,
    ) -> Result<String, ProviderError> {
        let system = "You are a translation engine. Preserve formatting, code blocks and placeholders. Only output the translated text.";
        let source_str = source.map(|l| l.as_str()).unwrap_or("auto");
        let user = format!("Source: {}\nTarget: {}\nText:\n{}", source_str, target.as_str(), text);

        let body = ChatReq {
            model: &self.model,
            temperature: 0.2,
            top_p: 1.0,
            stream: false,
            messages: vec![
                Message { role: "system", content: system.to_string() },
                Message { role: "user", content: user },
            ],
        };

        let url = format!("{}/v1/chat/completions", self.base_url.trim_end_matches('/'));
        let resp = self.client
            .post(url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .map_err(|e| ProviderError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(ProviderError::Http(format!("status={} body={}", status, text)));
        }

        let out: ChatResp = resp.json().await.map_err(|e| ProviderError::BadResponse(e.to_string()))?;
        let content = out
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| ProviderError::BadResponse("empty choices".into()))?;
        Ok(content)
    }
    fn name(&self) -> &'static str { "deepseek" }
}

pub fn from_env() -> Option<DeepseekProvider> {
    let api_key = std::env::var("DEEPSEEK_API_KEY").ok()?;
    let base = std::env::var("DEEPSEEK_BASE_URL").unwrap_or_else(|_| "https://api.deepseek.com".to_string());
    let model = std::env::var("DEEPSEEK_MODEL").unwrap_or_else(|_| "deepseek-chat".to_string());
    Some(DeepseekProvider::new(base, api_key, model))
}

