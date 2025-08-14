//! 识别管理器
//! 负责协调所有识别器的并行执行

use crate::identification::traits::{IdentificationResult, Identifier};
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

/// 识别管理器，负责并行执行所有识别器
pub struct IdentificationManager {
    identifiers: Vec<Arc<dyn Identifier>>,
    timeout_duration: Duration,
}

impl IdentificationManager {
    pub fn new() -> Self {
        Self {
            identifiers: Vec::new(),
            timeout_duration: Duration::from_secs(5), // 默认5秒超时
        }
    }
}

impl Default for IdentificationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl IdentificationManager {
    /// 添加识别器
    pub fn add_identifier<T: Identifier + 'static>(&mut self, identifier: T) {
        self.identifiers.push(Arc::new(identifier));
    }

    /// 设置超时时间
    pub fn set_timeout(&mut self, duration: Duration) {
        self.timeout_duration = duration;
    }

    /// 并行执行所有识别器
    pub async fn run_all(&self) -> HashMap<String, Vec<Box<dyn IdentificationResult>>> {
        let mut results = HashMap::new();

        // 过滤启用的识别器
        let enabled_identifiers: Vec<_> = self
            .identifiers
            .iter()
            .filter(|id| id.is_enabled())
            .collect();

        if enabled_identifiers.is_empty() {
            return results;
        }

        println!(
            "🚀 启动并行识别系统，共 {} 个识别器",
            enabled_identifiers.len()
        );

        // 创建异步任务
        let tasks: Vec<_> = enabled_identifiers
            .iter()
            .map(|identifier| {
                let identifier = Arc::clone(identifier);
                async move {
                    let name = identifier.name();
                    match timeout(self.timeout_duration, identifier.identify()).await {
                        Ok(results) => {
                            println!("✅ {} 识别完成，找到 {} 个结果", name, results.len());
                            (name.to_string(), results)
                        }
                        Err(_) => {
                            println!("⚠️ {} 识别超时", name);
                            (name.to_string(), Vec::new())
                        }
                    }
                }
            })
            .collect();

        // 并行执行所有任务
        let all_results = join_all(tasks).await;

        // 收集结果
        for (name, result) in all_results {
            if !result.is_empty() {
                results.insert(name, result);
            }
        }

        results
    }
}
