# Tool: Detect Language

识别输入文本语言（中文/英文）。

## 请求与响应

- `GET /api/tools/detect?text=...`
- 响应：
```json
{ "detectedLang": "zh", "confidence": 0.99, "provider": "whatlang" }
```

## 策略

1. 启发式：CJK 字符占比 vs ASCII 字母占比（极快）
2. 模型：`whatlang` 或 `lingua` 二判，提高稳定性
3. 合并：短文本或低置信度回退启发式；高置信度直接采用

## 注意

- 混合文本：优先选择主导语言；将来可返回 `mixed`
- 输入长度：设置上限并截断；给出 `truncated` 标记（可选）

