//! 识别器trait定义
//! 为所有识别器提供统一的异步接口

use async_trait::async_trait;
use std::fmt;

/// 识别结果trait，所有识别结果都必须实现此trait
pub trait IdentificationResult: fmt::Debug + Send + Sync {
    fn uid(&self) -> &str;
    fn source(&self) -> &str;
}

/// 识别器trait，所有识别器都必须实现此trait
#[async_trait]
pub trait Identifier: Send + Sync {
    /// 识别器名称
    fn name(&self) -> &'static str;

    /// 执行识别操作
    async fn identify(&self) -> Vec<Box<dyn IdentificationResult>>;

    /// 是否启用此识别器
    fn is_enabled(&self) -> bool {
        true
    }
}

/// 统一的识别结果结构体
#[derive(Debug, Clone)]
pub struct GenericIdentificationResult {
    pub uid: String,
    pub source: String,
}

impl IdentificationResult for GenericIdentificationResult {
    fn uid(&self) -> &str {
        &self.uid
    }
    fn source(&self) -> &str {
        &self.source
    }
}

impl GenericIdentificationResult {
    pub fn new(uid: String, source: String) -> Self {
        Self { uid, source }
    }
}
