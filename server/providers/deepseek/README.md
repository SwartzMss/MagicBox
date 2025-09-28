# DeepSeek Provider

使用 DeepSeek 作为翻译引擎，默认按 OpenAI 兼容的 Chat Completions 接口进行调用；若官方接口不同，可通过 `DEEPSEEK_BASE_URL` 和请求体结构进行调整。

## 环境变量

- `DEEPSEEK_API_KEY`：必填，`Bearer` 令牌
- `DEEPSEEK_BASE_URL`：默认 `https://api.deepseek.com`
- `DEEPSEEK_MODEL`：默认 `deepseek-chat`

## HTTP 调用（假设 OpenAI 兼容）

- URL: `${DEEPSEEK_BASE_URL}/v1/chat/completions`
- Header: `Authorization: Bearer ${DEEPSEEK_API_KEY}`，`Content-Type: application/json`
- Body 示例：

```json
{
  "model": "deepseek-chat",
  "temperature": 0.2,
  "top_p": 1,
  "stream": false,
  "messages": [
    {"role":"system","content":"You are a translation engine. Preserve formatting, code blocks and placeholders. Only output the translated text."},
    {"role":"user","content":"Source: auto\nTarget: en\nText:\n你好，世界"}
  ]
}
```

- 解析：`choices[0].message.content` 作为译文
- 超时：5s；重试：指数退避 3 次

## 语言方向

`source` 为 `auto` 时，先走语言检测模块得到 `zh|en`，`target` 为另一种语言；也可由前端直接指定。

## 错误分类

- 4xx：参数错误、配额、鉴权失败 -> 转为 `BadRequest` 或 `Unauthorized`
- 5xx：服务错误 -> `BadGateway`（携带重试建议）

## 缓存键

`hash("deepseek" + source + target + text)`，TTL 由 `CACHE_TTL_SECONDS` 控制。

