# Tool: Timestamp

Unix 时间戳与 ISO-8601 互转工具，支持当前时间、秒/毫秒解析，以及本地时间显示。

## 请求与响应

- `POST /api/tools/timestamp`
- 请求示例：
```json
{ "mode": "fromUnix", "value": "1700000000", "unit": "seconds" }
```
- 响应示例：
```json
{
  "unixSeconds": 1700000000,
  "unixMillis": 1700000000000,
  "iso8601": "2023-11-14T22:13:20Z",
  "localTime": "2023-11-15 06:13:20",
  "zoneOffset": "+00:00"
}
```

## 模式说明

- `now`：忽略 `value`，返回当前 UTC 时间
- `fromUnix`：`value` 为 Unix 时间戳，`unit` 为 `seconds` 或 `milliseconds`
- `fromIso`：`value` 可为带偏移的 ISO-8601 字符串，或 `<yyyy-MM-dd HH:mm:ss>`，可选传入 `timezone`（例如 `+08:00`）覆盖本地时区

## 实现要点

- 使用 `time` crate 解析/格式化
- 根据输入长度判断秒或毫秒
- 对非法输入返回 `400 BadRequest`
- 本地时间使用系统时区（若不可用则退回 UTC）
