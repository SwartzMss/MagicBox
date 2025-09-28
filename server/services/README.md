# Services (横切能力)

## 缓存（moka）

- 键：`hash(provider + source + target + text)`
- 值：翻译结果字符串或检测结果对象
- TTL：`CACHE_TTL_SECONDS`
- 容量：`CACHE_MAX_ENTRIES`

## 限流（tower-governor）

- 策略：按 IP 每分钟 `RATE_LIMIT_PER_MINUTE`
- 路由级别应用在 `/api/tools/*`

## 日志与追踪（tracing）

- 请求 ID、耗时、状态码、错误原因
- `LOG_LEVEL` 控制输出级别

## 超时与重试

- 上游调用（DeepSeek）：请求超时 5s，指数退避重试 3 次
- 对客户端：明确 `Retry-After`（当可重试时）

