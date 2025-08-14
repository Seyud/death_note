//! 识别器trait定义
//! 为所有识别器提供统一的异步接口

use async_trait::async_trait;
use std::fmt;

/// 识别结果trait，所有识别结果都必须实现此trait
pub trait IdentificationResult: fmt::Debug + Send + Sync {
    fn uid(&self) -> &str;
    fn source(&self) -> &str;
    fn details(&self) -> String;
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
    pub package_name: Option<String>,
    pub config_path: Option<String>,
    pub additional_info: Vec<(String, String)>,
}

impl IdentificationResult for GenericIdentificationResult {
    fn uid(&self) -> &str {
        &self.uid
    }

    fn source(&self) -> &str {
        &self.source
    }

    fn details(&self) -> String {
        let mut details = format!("来源: {}", self.source);

        if let Some(pkg) = &self.package_name {
            details.push_str(&format!(", 包名: {}", pkg));
        }

        if let Some(path) = &self.config_path {
            details.push_str(&format!(", 配置文件: {}", path));
        }

        for (key, value) in &self.additional_info {
            details.push_str(&format!(", {}: {}", key, value));
        }

        details
    }
}

impl GenericIdentificationResult {
    pub fn new(uid: String, source: String) -> Self {
        Self {
            uid,
            source,
            package_name: None,
            config_path: None,
            additional_info: Vec::new(),
        }
    }

    pub fn with_package_name(mut self, package_name: String) -> Self {
        self.package_name = Some(package_name);
        self
    }

    pub fn with_config_path(mut self, config_path: String) -> Self {
        self.config_path = Some(config_path);
        self
    }

    pub fn with_additional_info(mut self, key: String, value: String) -> Self {
        self.additional_info.push((key, value));
        self
    }
}
