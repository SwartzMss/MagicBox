# Tool: MD5

计算输入文本的 MD5（十六进制小写）。

## 请求与响应

- `POST /api/tools/hash/md5`
- 请求：
```json
{ "text": "hello" }
```
- 响应：
```json
{ "md5": "5d41402abc4b2a76b9719d911017c592" }
```

## 实现要点

- 使用 `md-5` 或 `md5` crate
- 输入编码按 UTF-8 处理
- 返回统一 32 位十六进制字符串（小写）

