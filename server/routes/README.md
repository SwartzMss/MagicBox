# Routes & APIs

基础路径：`/api`

## 健康检查

- `GET /api/health`
- 响应：`{ "ok": true }`

## 翻译

- `POST /api/tools/translate`
- 请求：
```json
{ "text": "你好", "sourceLang": "auto", "targetLang": "en" }
```
- 响应：
```json
{
  "detectedLang": "zh",
  "targetLang": "en",
  "translation": "Hello",
  "provider": "deepseek",
  "cached": false
}
```

## 语言检测

- `GET /api/tools/detect?text=...`
- 响应：
```json
{ "detectedLang": "zh", "confidence": 0.99, "provider": "whatlang" }
```

## JSON 格式化

- `POST /api/tools/json/format`
- 请求：
```json
{ "json": "{\"a\":1}", "indent": 2 }
```
- 响应：
```json
{ "formatted": "{\n  \"a\": 1\n}" }
```

## MD5

- `POST /api/tools/hash/md5`
- 请求：
```json
{ "text": "hello" }
```
- 响应：
```json
{ "md5": "5d41402abc4b2a76b9719d911017c592" }
```

## 错误模型（统一）

```json
{ "code": "BadRequest", "message": "invalid payload", "details": null }
```

