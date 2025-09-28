# Tool: JSON Format

将输入的 JSON 字符串进行校验与格式化美化（pretty print）。

## 请求与响应

- `POST /api/tools/json/format`
- 请求：
```json
{ "json": "{\"a\":1}", "indent": 2 }
```
- 响应：
```json
{ "formatted": "{\n  \"a\": 1\n}" }
```

## 实现要点

- 使用 `serde_json` 解析与格式化
- 允许自定义缩进（默认 2）
- 错误：无效 JSON -> `BadRequest`，返回错误位置与片段（可选）

