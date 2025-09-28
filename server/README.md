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

- `PORT=8080`
- `TRANSLATE_PROVIDER=deepseek`
- `DEEPSEEK_API_KEY=sk-...`
- `DEEPSEEK_BASE_URL=https://api.deepseek.com`
- `DEEPSEEK_MODEL=deepseek-chat`
- `CACHE_TTL_SECONDS=300`
- `CACHE_MAX_ENTRIES=10000`
- `LOG_LEVEL=info`
- `ENABLE_CORS=1`

## 目录（规划）

- `providers/`：Provider 抽象与实现（DeepSeek 等）
- `routes/`：路由与 API handler 文档
- `services/`：缓存、日志等横切能力
- `tools/`：工具模块规划（translate/json_format/hash_md5）

## API 约定

- 基础路径：`/api`
- 健康检查：`GET /api/health -> { ok: true }`
- 翻译：`POST /api/tools/translate`
- JSON 格式化：`POST /api/tools/json/format`
- MD5：`POST /api/tools/hash/md5`

详见各模块 README：

- `routes/README.md`：端点与入参/出参
- `providers/deepseek/README.md`：DeepSeek 调用细节
- `tools/*/README.md`：各工具实现与测试要点

## 实现建议（分阶段）

1. 搭建 Axum 骨架：路由树、AppState、CORS、日志
2. 翻译模块：内置自动语言检测（启发式/模型可选）、缓存与重试（DeepSeek Provider）
3. JSON 格式化、MD5 工具端点
4. 统一错误模型与追踪日志
5. 预留扩展：术语表、批量任务、文档解析器

## 运行（占位说明）

本仓库当前仅包含设计文档与目录规划。落地代码后：

```bash
cp .env.example .env
cargo run # in server/
```
