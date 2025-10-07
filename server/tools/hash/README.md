# Tool: Hash

对输入文本计算哈希摘要，当前支持 MD5 与 SHA-256。

## 请求与响应

- `POST /api/tools/hash`
- 请求：
```json
{ "text": "hello", "algorithm": "md5" }
```
- 响应：
```json
{ "algorithm": "md5", "digest": "5d41402abc4b2a76b9719d911017c592" }
```

## 实现要点

- `algorithm` 默认为 `md5`，大小写不敏感，可扩展更多算法
- 使用 `md5`、`sha2` 等 crate 生成十六进制摘要（小写）
- 输入按 UTF-8 文本处理；如需处理二进制可扩展接口
