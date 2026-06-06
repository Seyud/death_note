# 📓 Death Note

> **以《死亡笔记》为灵感的 Android 多平台用户身份识别与黑名单管理系统**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Async-Tokio-blue.svg)](https://tokio.rs/)

---

## 📋 项目简介

**Death Note** 是一个使用 Rust 编写的系统工具，以《死亡笔记》动漫的世界观为核心设计理念，实现了：

- 🔍 **多平台用户身份识别** — 支持酷安、QQ、Telegram、微信等平台
- 📖 **黑名单管理** — 支持本地编译时嵌入 + 云端动态更新的双数据源
- ⚰️ **Android 分区操作** — 支持 A/B、VAB、A-only 分区方案
- ⚡ **全异步并行架构** — 基于 Tokio 异步运行时，所有识别器并发执行

## 🏗️ 核心架构

### 🔑 概念映射

| 动漫概念 | 系统模块 | 功能描述 |
|---------|---------|---------|
| 📖 死亡笔记 | `blacklist` | 黑名单管理 — 记录应被审判的目标 |
| 👁️‍🗨️ 死神之眼 | `identification` | 身份识别 — 看透用户真实身份 |
| 😈 琉克 | `guidance` | 制导系统 — 智能决策与审判执行 |
| ☁️ 死神界 | `cloud_control` | 云控系统 — 动态数据源管理 |

### 📦 模块结构

```
death_note/
├── src/
│   ├── main.rs                    # 入口：编排完整执行流程
│   ├── lib.rs                     # 库根：导出 4 大模块
│   ├── blacklist/                 # 📖 死亡笔记 — 黑名单系统
│   │   ├── manager.rs             #   DeathNote 管理器（本地 + 云控）
│   │   ├── coolapk.rs             #   酷安黑名单
│   │   ├── qq.rs                  #   QQ 黑名单
│   │   ├── telegram.rs            #   Telegram 黑名单
│   │   └── wechat.rs              #   微信黑名单
│   ├── identification/            # 👁️‍🗨️ 死神之眼 — 识别系统
│   │   ├── traits.rs              #   ShinigamiEye trait（通用接口）
│   │   ├── manager.rs             #   ShinigamiEyeManager（并行协调）
│   │   ├── coolapk_identifier.rs  #   酷安用户识别器
│   │   ├── qq_identifier.rs       #   QQ 用户识别器
│   │   ├── telegram_identifier.rs #   Telegram 用户识别器
│   │   ├── wechat_identifier.rs   #   微信用户识别器
│   │   └── lifespan_calculator.rs #   寿命计算器
│   ├── guidance/                  # 😈 琉克 — 制导系统
│   │   ├── guidance_async.rs      #   RyukGuidanceSystem（审判决策）
│   │   └── partition_ops.rs       #   AndroidPartitionOperator（分区操作）
│   └── cloud_control/             # ☁️ 云控系统
│       ├── manager.rs             #   CloudControlManager（核心管理器）
│       ├── client.rs              #   CloudControlClient（网络客户端）
│       ├── cache.rs               #   CloudControlCache（本地缓存）
│       ├── types.rs               #   类型定义
│       ├── embedded_config.rs     #   编译时配置嵌入
│       └── error.rs               #   错误类型
├── config/                        # 配置文件
│   ├── blacklist_config.toml      #   本地黑名单数据
│   ├── cloud_config.toml          #   云控系统配置
│   ├── build_android.toml         #   Android 构建配置
│   └── examples/                  #   配置示例
├── build.rs                       # 构建脚本（配置 → 代码生成）
├── build_android.py               # Android 交叉编译脚本
└── Cargo.toml                     # Rust 包清单
```

## 🚀 快速开始

### 📋 环境要求

- **Rust** — edition 2024（`rustup update` 获取最新版本）
- **Android NDK** — 用于交叉编译 Android 目标（可选）
- **UPX** — 用于二进制压缩（可选）

### 🔧 桌面构建与运行

```bash
# 克隆项目
git clone https://github.com/your-repo/death_note.git
cd death_note

# 编辑黑名单配置
cp config/examples/blacklist_config.example.toml config/blacklist_config.toml
# 根据需要修改 config/blacklist_config.toml

# 构建
cargo build --release

# 运行（桌面模式，用于开发测试）
cargo run --release
```

### 📱 Android 交叉编译

```bash
# 1. 配置 Android 构建参数
cp config/examples/build_android.example.toml config/build_android.toml
# 编辑 config/build_android.toml，设置 NDK 和 UPX 路径

# 2. 执行构建脚本
python build_android.py

# 3. 输出产物位于 output/death_note
```

## ⚙️ 配置管理

### 📖 黑名单配置

编辑 `config/blacklist_config.toml`：

```toml
[coolapk]
users = [
    "1234567",   # 酷安用户 ID
    "9999999",
]

[qq]
users = [
    "123456789", # QQ 号
    "987654321",
]

[telegram]
users = [
    "100000000", # Telegram 用户 ID
]

[wechat]
users = [
    "wxid_xxx",  # 微信 ID
]

[meta]
version = "1.0"
description = "死亡笔记黑名单配置文件"
last_updated = "2025-01-01"
```

> 💡 黑名单数据在 **编译时** 通过 `build.rs` 嵌入到二进制文件中，生产环境无需携带配置文件。

### ☁️ 云控配置

编辑 `config/cloud_config.toml`：

```toml
[enabled]
enabled = true

[repository]
url = "https://gitee.com/your-username/death-note-cloud"
branch = "main"
data_file = "blacklist.toml"
access_token = ""  # 可选

[cache]
cache_dir = ".cache/cloud_control"
cache_file = "cloud_data.json"
ttl_seconds = 3600  # 缓存 1 小时

[update]
check_interval_seconds = 300  # 每 5 分钟检查更新
timeout_seconds = 30
retry_count = 3
```

### 🔀 双数据源机制

```
┌─────────────────┐     ┌─────────────────┐
│  本地编译名单    │     │  云控动态名单    │
│  (TOML → build) │     │  (Git → HTTP)   │
└────────┬────────┘     └────────┬────────┘
         │                       │
         └───────────┬───────────┘
                     ▼
            ┌────────────────┐
            │  DeathNote     │
            │  统一查询接口  │
            │  任一匹配 → ✓  │
            └────────────────┘
```

- **本地编译名单**：编译时嵌入，零运行时开销，离线可用
- **云控动态名单**：运行时获取，支持远程更新，带 TTL 缓存
- **查询逻辑**：同时检查两个数据源，任一匹配即认定为目标
- **容错设计**：云控系统故障不影响本地功能

## 🔄 执行流程

```
📓 死亡笔记系统启动 (Tokio 异步运行时)
    ↓
☁️ 初始化云控系统
    ├── 加载编译时嵌入的云控配置
    ├── 创建网络客户端与缓存系统
    ├── 优先从缓存加载（减少网络请求）
    └── 缓存无效时从远程仓库获取
    ↓
🎯 创建死神之眼管理器
    ├── 注册酷安 / QQ / Telegram / 微信识别器
    └── 设置 6 秒超时限制
    ↓
👁️‍🗨️ 并行激活所有死神之眼
    ├── 各平台异步扫描 shared_prefs / 用户数据
    └── 提取用户 ID 并计算寿命
    ↓
📖 黑名单检查
    ├── 本地编译名单匹配
    └── 云控动态名单匹配
    ↓
😈 琉克审判决策
    ├── 🍎 每发现 1 个目标消费 1 个苹果
    ├── 😴 调整厌倦值
    └── 🎯 生成 Execute / Skip 决策
    ↓
⚰️ 执行灵魂收割（如适用）
    ├── 检测 Android 分区方案（A/B/VAB/A-only）
    └── 执行 boot 分区恢复操作
    ↓
📓 运行结束
```

## 🎯 最佳实践：NewQStory

> **[NewQStory (QStory)](https://github.com/Xposed-Modules-Repo/lin.xposed)** 是一个 Xposed QQ 增强模块，使用了 Death Note 库，是 Death Note 在实际场景中的最佳实践示例。

### 📌 项目概述

| 项目 | 说明 |
|------|------|
| **名称** | NewQStory（QStory） |
| **类型** | Xposed 模块 |
| **目标应用** | QQ（8.9.68 ~ 9.xx+） |
| **底层依赖** | Death Note 库 |
| **推荐框架** | [LSPosed](https://github.com/LSPosed/LSPosed) / [LSPatch](https://github.com/LSPosed/LSPatch) |
| **仓库地址** | [GitHub - Xposed-Modules-Repo/lin.xposed](https://github.com/Xposed-Modules-Repo/lin.xposed) |

### 🔗 与 Death Note 的关系

QStory 是一个独立的 Xposed QQ 增强模块，其中使用了 Death Note 库来实现用户识别和黑名单管理功能：

```
┌─────────────────────────────────────────────────┐
│                  NewQStory (QStory)              │
│              Xposed QQ 增强模块                   │
│                                                  │
│   + Xposed Hook API                              │
│   + QQ 版本自适配逻辑                             │
│   + 模块功能实现                                  │
│   + Death Note 库（用户识别 · 黑名单管理）        │
└─────────────────────────────────────────────────┘
```

- **📦 Death Note 的使用者** — QStory 在项目中引入 Death Note 库，利用其多平台用户识别和黑名单管理能力
- **🔗 集成参考** — 展示了 Death Note 库在 Xposed 模块场景下的实际集成方式
- **📱 独立项目** — QStory 自身包含完整的 Xposed 模块逻辑，Death Note 为其提供底层能力支持

### 💡 最佳实践参考

QStory 的实践为其他开发者使用 Death Note 库提供了以下参考：

1. **集成方式** — 展示了如何在 Xposed 模块中引入和使用 Death Note 库
2. **版本自适配** — 实现了 QQ 8.9.68 ~ 9.xx+ 的宽版本自适配
3. **框架兼容** — 已完成 LSPosed、LSPatch 等主流框架的适配验证
4. **实际验证** — QStory 的长期使用验证了 Death Note 库在真实场景下的稳定性

## 🛠️ 技术栈

| 类别 | 技术 | 用途 |
|------|------|------|
| 语言 | Rust (edition 2024) | 核心实现 |
| 异步运行时 | Tokio | 异步并行执行 |
| HTTP 客户端 | Reqwest (rustls-tls) | 云控数据获取 |
| XML 解析 | quick-xml | 解析 shared_prefs 文件 |
| 序列化 | serde / serde_json / toml | 配置和数据处理 |
| 正则匹配 | regex | 用户标识符提取 |
| 错误处理 | anyhow / thiserror | 错误类型与传播 |
| 压缩 | UPX | 二进制体积优化 |

## 📊 构建配置

### 🏎️ 开发模式

```toml
[profile.dev]
opt-level = 1           # 快速编译 + 基本优化

[profile.dev.package.regex]
opt-level = 3           # 关键依赖深度优化

[profile.dev.package.reqwest]
opt-level = 3

[profile.dev.package.tokio]
opt-level = 3
```

### 🚀 发布模式

```toml
[profile.release]
opt-level = 3           # 最大优化
lto = true              # 链接时优化
codegen-units = 1       # 单代码生成单元
strip = true            # 符号剥离
panic = "abort"         # panic 直接终止
```

## 📚 相关文档

- [📖 开发文档 (DEVELOPMENT.md)](./DEVELOPMENT.md) — 详细的架构设计与实现说明

## 📜 许可证

本项目基于 [MIT License](../LICENSE) 开源。

---

> ⚠️ **免责声明**：本项目仅供学习与技术研究用途。请勿将其用于任何违法或商业行为。
