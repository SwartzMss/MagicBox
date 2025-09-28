# Providers

Provider 层抽象外部能力（如翻译引擎）或可替换实现（本地/云端）。

## 目标

- 通过 `TranslateProvider` trait 屏蔽具体厂商差异
- 支持：`deepseek`（优先）、后续可增加 `openai`、`deepl`、`local`
- 配置与依赖注入：通过环境变量切换 Provider

## Trait（伪代码）

```rust
pub enum Lang { Zh, En }

#[async_trait]
pub trait TranslateProvider: Send + Sync {
    async fn translate(
        &self,
        text: &str,
        source: Option<Lang>, // None => auto
        target: Lang,
    ) -> Result<String, ProviderError>;
    fn name(&self) -> &'static str { "deepseek" }
}
```

错误统一为 `ProviderError`，上层转换为 API 错误。

## DeepSeek

详见 `deepseek/README.md`，默认假设为 OpenAI 兼容 Chat Completions（可通过 `DEEPSEEK_BASE_URL` 覆写）。

## 构造与切换

从环境变量读取 `TRANSLATE_PROVIDER`，创建对应 Provider，并注入到 `AppState`。

