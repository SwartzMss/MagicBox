# MagicBox

多功能在线工具服务（百宝箱）。当前规划以 Rust 后端为核心，优先支持：

- 中英互译（内置自动语言检测，Provider：DeepSeek，可扩展 OpenAI/DeepL/本地）
- JSON 格式化
- MD5 计算

后续将持续扩展更多工具（如术语表、批量文档处理、加解密等）。

## Monorepo 目录规划

```
MagicBox/
├─ server/                  # Rust 后端（Axum/Tokio/Tower）
│  ├─ .env.example          # 环境变量示例
│  ├─ README.md             # 后端总体设计与运行说明
│  ├─ providers/            # 可插拔 Provider（翻译 等）
│  │  ├─ README.md
│  │  └─ deepseek/README.md # DeepSeek 接入说明
│  ├─ routes/README.md      # 路由与 API 列表
│  ├─ services/README.md    # 缓存/日志 等横切能力
│  └─ tools/                # 具体工具的实现规划
│     ├─ translate/README.md
│     ├─ json_format/README.md
│     └─ hash_md5/README.md
├─ web/                     # 前端（占位，后续接入 Next.js/Vite）
│  └─ README.md
└─ docs/
   └─ ROADMAP.md           # 路线图与里程碑
```

> 说明：本次提交仅包含目录规划与各子项目 README 设计文档，便于后续按模块逐步落地代码。

## 快速导览

- 后端总体：`server/README.md`
- DeepSeek 接入：`server/providers/deepseek/README.md`
- API 总览：`server/routes/README.md`
- 工具规划：
  - 翻译：`server/tools/translate/README.md`
  - JSON 格式化：`server/tools/json_format/README.md`
  - MD5：`server/tools/hash_md5/README.md`
- 路线图：`docs/ROADMAP.md`

## 下一步

1) 在 `server/` 按 READMEs 初始化 Axum 项目骨架与模块目录
2) 优先实现翻译（内置自动检测，DeepSeek Provider），随后 JSON/MD5
3) 接入缓存与日志
4) 规划并实现前端界面（左右对照、自动检测标签、复制/交换/清空）
