# MagicBox Roadmap

## Phase 1 — MVP

- 目录与设计文档（本次完成）
- Axum 骨架、健康检查、统一错误模型
- 语言检测：启发式 + whatlang
- 翻译：DeepSeek Provider + 缓存 + 重试
- JSON 格式化、MD5 工具

## Phase 2 — 体验优化

- 前端界面（Next.js/Vite），自动检测标签、复制/交换/清空
- 限流、日志完善、基本监控（请求量/错误率）
- 术语表/禁译词（简单规则）

## Phase 3 — 可扩展性

- 批量/文档（txt/md/docx）
- Markdown 结构保留翻译
- 本地离线 Provider（MarianMT/M2M100）
- 权限/配额（按 IP 或 Token）

## Phase 4 — 进阶工具

- 加解密/签名、Base64、UUID、正则测试
- 文本批处理（去重、去空格、大小写转换）

