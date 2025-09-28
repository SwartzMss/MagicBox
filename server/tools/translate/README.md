# Tool: Translate

提供中英互译，支持自动语言检测。默认 Provider 为 DeepSeek，可扩展。

## 请求与响应

- `POST /api/tools/translate`
- 请求：
```json
{ "text": "你好世界", "sourceLang": "auto", "targetLang": "en" }
```
- 响应：
```json
{
  "detectedLang": "zh",
  "targetLang": "en",
  "translation": "Hello, world",
  "provider": "deepseek",
  "cached": true
}
```

## 处理流程

1. 参数校验：空文本/过长文本（可限制长度）
2. 语言检测：若 `sourceLang=auto`，调用 detect 模块
3. 目标语言：若未给出，设为 `zh<->en` 的另一端
4. 缓存命中则直接返回
5. 调用 Provider（DeepSeek），带系统提示，`temperature=0.2`
6. 结果入缓存，返回响应

## 提示词（system prompt）

> You are a translation engine. Preserve formatting, code blocks and placeholders. Only output the translated text.

## 边界与兜底

- 代码块/占位符保留：可在后续版本加入保护规则
- 错误：参数/上游错误映射到统一错误模型

