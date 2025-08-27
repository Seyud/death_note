//! 识别器trait定义
//! 为所有识别器提供统一的异步接口

use async_trait::async_trait;
use std::fmt;

/// 死神之眼识别结果特征 (Shinigami Eye Result)
/// 原型：死神之眼 - 可以看到人类的真名和剩余寿命的能力
pub trait ShinigamiEyeResult: fmt::Debug + Send + Sync {
    /// 获取目标真名（对应UID）
    fn name(&self) -> &str;
    /// 获取来源平台
    fn source(&self) -> &str;
    /// 获取剩余寿命（死神之眼可见，单位：年）
    fn lifespan(&self) -> u32;
    /// 检查是否在黑名单中（死亡笔记上的名字）
    fn is_blacklisted(&self) -> bool;
}

/// 死神之眼识别器特征 (Shinigami Eye Identifier)
/// 原型：死神之眼能力 - 能够看透人类身份的特殊视觉
#[async_trait]
pub trait ShinigamiEye: Send + Sync {
    /// 识别器名称（对应死神角色）
    fn name(&self) -> &'static str;

    /// 使用死神之眼执行识别
    async fn identify(&self) -> Vec<Box<dyn ShinigamiEyeResult>>;

    /// 是否启用死神之眼
    fn is_enabled(&self) -> bool {
        true
    }
}

/// 死神之眼通用识别结果结构体
#[derive(Debug, Clone)]
pub struct GenericShinigamiEyeResult {
    pub name: String,
    pub source: String,
    pub lifespan: u32,
    pub is_blacklisted: bool,
}

impl ShinigamiEyeResult for GenericShinigamiEyeResult {
    fn name(&self) -> &str {
        &self.name
    }
    fn source(&self) -> &str {
        &self.source
    }
    fn lifespan(&self) -> u32 {
        self.lifespan
    }
    fn is_blacklisted(&self) -> bool {
        self.is_blacklisted
    }
}

impl GenericShinigamiEyeResult {
    pub fn new(name: String, source: String, lifespan: u32, is_blacklisted: bool) -> Self {
        Self {
            name,
            source,
            lifespan,
            is_blacklisted,
        }
    }
}
