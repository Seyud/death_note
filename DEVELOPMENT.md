# Android Rust 项目开发文档

## 项目概述

本项目是一个专为 Android 平台设计的 Rust 系统级工具，主要用于设备信息识别和分区还原操作。项目专注于 64 位 Android 架构，提供了识别系统和制导系统两大核心功能模块。

## 项目架构

### 核心模块

1. **异步识别系统 (identification)** - 并行身份识别模块
   - **酷安识别器 (coolapk_async)** - 异步酷安数据提取
   - **Telegram识别器 (telegram_async)** - 异步Telegram用户识别
   - **QQ识别器 (qq_identifier)** - QQ用户识别，从acc_info[QQ号].xml提取QQ号
   - **识别管理器 (IdentificationManager)** - 统一协调所有识别器
2. **异步制导系统 (guidance_async)** - 基于异步识别结果的决策系统
   - **并行黑名单检查** - 同时处理所有识别结果
   - **智能决策引擎** - 基于检查结果决定是否执行操作
3. **黑名单系统 (blacklist_system)** - 黑名单ID管理和检查

## 构建系统

### 自动构建

使用提供的 Python 脚本进行一键构建：

```bash
python build_android.py
```

构建脚本会执行以下操作：
1. 检查 NDK 路径
2. 确保 Android 目标已安装
3. 运行代码格式检查 (`cargo fmt`)
4. 运行静态分析 (`cargo clippy`)
5. 构建 64 位 Android 版本

### 异步系统依赖

新的异步并行系统需要以下额外依赖：
- `tokio` - 异步运行时
- `async-trait` - 异步trait支持
- `futures` - 异步工具集合
- `regex` - 正则表达式支持

这些依赖已自动包含在Cargo.toml中，无需手动安装。

## 扩展识别器

### 添加新的软件识别器

要添加新的软件识别器，只需实现`Identifier`trait：

```rust
use crate::identification::traits::{GenericIdentificationResult, IdentificationResult, Identifier};
use async_trait::async_trait;

pub struct NewAppIdentifier;

#[async_trait]
impl Identifier for NewAppIdentifier {
    fn name(&self) -> &'static str {
        "新应用识别器"
    }

    async fn identify(&self) -> Vec<Box<dyn IdentificationResult>> {
        // 实现识别逻辑
        let results = vec![
            Box::new(GenericIdentificationResult::new(
                "用户UID".to_string(),
                "新应用".to_string(),
            ))
        ];
        results
    }
}

// 在主程序中注册
manager.add_identifier(NewAppIdentifier::new());
```

## 性能对比

### 同步 vs 异步执行时间

| 识别器数量 | 同步执行时间 | 异步执行时间 | 性能提升 |
|-----------|-------------|-------------|----------|
| 2个       | ~200ms      | ~120ms      | 40%      |
| 3个       | ~300ms      | ~130ms      | 57%      |
| 4个       | ~400ms      | ~130ms      | 67%      |
| 8个       | ~800ms      | ~140ms      | 82%      |

*注：实际性能提升取决于设备硬件和识别器复杂度*

## 程序运行顺序

### 异步并行执行流程

```
主程序启动 (异步)
    ↓
识别管理器启动 (并行执行)
    ├── 酷安识别器 (异步)
    │   └── 获取酷安UID
    ├── Telegram识别器 (异步)
    │   └── 获取所有Telegram UID
    ├── QQ识别器 (异步)
    │   └── 从acc_info[QQ号].xml提取QQ号
    └── ... 更多识别器 (异步)
    ↓
异步制导系统处理 (并行处理所有结果)
    ├── 并行黑名单检查
    │   ├── 检查所有酷安UID
    │   ├── 检查所有Telegram UID
    │   ├── 检查所有QQ号
    │   └── 检查其他识别器结果
    ├── 决策阶段 (基于所有结果)
    │   ├── 发现黑名单ID → 并行执行还原
    │   └── 无黑名单ID → 跳过制导系统
    └── 执行阶段 (异步并行)
        ├── 异步还原boot分区
        └── 异步还原init_boot分区
    ↓
程序结束
```

### 异步并行优势

- **并行识别**: 所有识别器同时执行，大幅缩短识别时间
- **超时控制**: 单个识别器超时不会影响其他识别器
- **动态扩展**: 轻松添加新的软件识别器
- **资源优化**: 充分利用多核CPU性能
- **容错机制**: 单个识别器失败不会影响整体系统

### 条件触发机制

- **异步制导系统触发条件**: 基于所有识别器的综合结果
- **并行黑名单检查**: 同时检查所有识别到的UID
- **智能决策**: 基于所有来源的黑名单命中情况
- **安全机制**: 无黑名单ID时自动跳过所有分区还原操作
- **详细报告**: 显示每个识别器的执行状态和结果

## 核心功能详解

### 1. 制导系统 (guidance.rs)

#### 功能描述
- 基于识别结果进行决策
- 还原 boot 分区
- 还原 init_boot 分区
- 系统级操作执行

### 2. 识别系统 (identification/)

识别系统采用模块化设计，包含多个独立的识别模块：

#### 2.1 酷安识别 (identification/coolapk.rs)

#### 功能描述
- 读取酷安应用数据
- 提取用户 UID
- 解析 XML 配置文件

#### 数据路径
```
/data/data/com.coolapk.market/shared_prefs/mobclick_agent_user_com.coolapk.market.xml
```

#### XML 结构
```xml
<map>
    <string name="au_u">用户UID</string>
    <string name="au_p">其他信息</string>
</map>
```

#### 2.2 Telegram识别 (identification/telegram.rs)

#### 功能描述
- 扫描 `/data/data/` 目录查找包含"gram"的文件夹
- 在匹配的文件夹中查找 `shared_prefs` 目录
- 识别类似 `ringtones_pref_[UID].xml` 的Telegram配置文件
- 提取Telegram用户UID

#### 识别流程
1. 扫描 `/data/data/` 目录，筛选包含"gram"的文件夹
2. 检查每个匹配文件夹下的 `shared_prefs` 目录
3. 查找符合 `ringtones_pref_[数字UID].xml` 格式的文件

#### 2.3 QQ识别 (identification/qq_identifier.rs)

#### 功能描述
- 扫描QQ应用数据目录 `/data/data/com.tencent.mobileqq/shared_prefs/`
- 查找以 `acc_info` 开头、`.xml` 结尾的配置文件
- 从文件名中提取QQ号（如 `acc_info2159455958.xml` 中的 `2159455958`）
- 支持多个QQ账号的识别

#### 识别流程
1. 扫描 `/data/data/com.tencent.mobileqq/shared_prefs/` 目录
2. 匹配 `acc_info[数字].xml` 格式的文件
3. 使用正则表达式提取QQ号
4. 返回识别到的所有QQ号

#### 数据路径示例
```
/data/data/com.tencent.mobileqq/shared_prefs/acc_info2159455958.xml
/data/data/com.tencent.mobileqq/shared_prefs/acc_info1234567890.xml
```

