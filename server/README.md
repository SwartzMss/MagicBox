# MagicBox Server (Rust)

Rust 后端服务，基于 Axum/Tokio/Tower，提供多工具 API（翻译［内置自动检测］、JSON 格式化、MD5 等）。

## 技术栈与依赖

- Web 框架：`axum`、`tower`
- 运行时：`tokio`
- 序列化：`serde`、`serde_json`
- HTTP 客户端：`reqwest`
- 配置：`dotenvy` 或 `config`
- 缓存：`moka`
- 日志：`tracing`、`tracing-subscriber`
- 错误：`thiserror`、`anyhow`
  

## 环境变量（.env）

见 `.env.example`，常用项：

- `PORT=18080`
- `TRANSLATE_PROVIDER=deepseek`
- `DEEPSEEK_API_KEY=sk-...`
- `DEEPSEEK_BASE_URL=https://api.deepseek.com`
- `DEEPSEEK_MODEL=deepseek-chat`
- `CACHE_TTL_SECONDS=300`
- `CACHE_MAX_ENTRIES=10000`
- `LOG_LEVEL=info`
- `ENABLE_CORS=1`

## 目录概览

- `app.rs`：`AppState`、Provider 与缓存装配
- `error.rs`：统一的 API 错误模型
- `providers/`：翻译 Provider 抽象与实现（DeepSeek 等）
- `routes/`：路由树与各工具 handler
  - `routes/health.rs`：健康检查
  - `routes/tools/`：翻译、JSON、MD5 等工具接口
- `tools/`：额外设计文档（translate/json_format/hash_md5）

## API 约定

- 基础路径：`/api`
- 健康检查：`GET /api/health -> { ok: true }`
- 翻译：`POST /api/tools/translate`
- JSON 格式化：`POST /api/tools/json/format`
- MD5：`POST /api/tools/hash/md5`

详见各模块 README：

- `providers/deepseek/README.md`：DeepSeek 调用细节
- `tools/*/README.md`：各工具实现与测试要点

## 开发流程建议

1. 维护核心模块解耦：Provider、路由、工具逻辑分离
2. 扩展翻译能力：更多语言、Provider 可配置、异常透明化
3. 为工具接口补充单元/集成测试，校验缓存与错误分支
4. 规划批处理、术语表等新工具，保持路由模块化结构

## 运行

```bash
cd server
cp .env.example .env
cargo run
```
