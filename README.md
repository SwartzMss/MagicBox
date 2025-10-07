# MagicBox

多功能在线工具服务（百宝箱）。目前以 Rust 后端为核心，已提供：

- 中英互译（内置自动语言检测，Provider：DeepSeek，可扩展 OpenAI/DeepL/本地）
- JSON 格式化/校验
- MD5 计算

静态前端页面托管于后端，包含侧边导航、翻译/JSON/MD5 三个模块，便于快速验证 API。后续计划继续扩展更多工具（如术语表、批量文档处理、加解密等）。

## Monorepo 目录规划

```
MagicBox/
├─ server/                  # Rust 后端（Axum/Tokio/Tower）
│  ├─ .env.example          # 环境变量示例
│  ├─ README.md             # 后端总体设计与运行说明
│  ├─ providers/            # 可插拔 Provider（翻译等）
│  │  ├─ README.md
│  │  └─ deepseek/README.md # DeepSeek 接入说明
│  └─ tools/                # 具体工具的实现与规划
│     ├─ translate/README.md
│     ├─ json_format/README.md
│     └─ hash_md5/README.md
├─ web/                     # 前端（静态资源与后续框架接入）
│  ├─ public/               # 静态资源（由后端托管到根路径 "/"）
│  │  └─ index.html         # 简易工具页（调用 /api/*）
│  └─ README.md
└─ docs/
   └─ ROADMAP.md           # 路线图与里程碑
```

## 快速导览

- 后端总体：`server/README.md`
- DeepSeek 接入：`server/providers/deepseek/README.md`
- 工具规划：
  - 翻译：`server/tools/translate/README.md`
  - JSON 格式化：`server/tools/json_format/README.md`
  - MD5：`server/tools/hash_md5/README.md`
- 路线图：`docs/ROADMAP.md`

## 下一步

1) 增加集成与单元测试（尤其是缓存/错误处理）
2) 丰富翻译配置（模型可切换、更多目标语言、异常可视化）
3) 扩展更多工具（术语表、批量处理等），保持前端导航一致
4) 规划前端框架版（如 React/Svelte），与当前静态页并存以便渐进迁移
