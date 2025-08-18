//! 识别管理器
//! 负责协调所有识别器的并行执行

use crate::identification::traits::{ShinigamiEye, ShinigamiEyeResult};
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::time::{Duration, timeout};

/// 死神之眼管理器 (Shinigami Eye Manager)
/// 原型：死神之眼能力管理 - 协调多个死神之眼并行识别
pub struct ShinigamiEyeManager {
    shinigami_eyes: Vec<Arc<dyn ShinigamiEye>>,
    vision_duration: Duration,
}

impl ShinigamiEyeManager {
    pub fn new() -> Self {
        Self {
            shinigami_eyes: Vec::new(),
            vision_duration: Duration::from_secs(6), // 死神之眼6秒限制
        }
    }
}

impl Default for ShinigamiEyeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ShinigamiEyeManager {
    /// 添加死神之眼识别器
    pub fn add_shinigami_eye<T: ShinigamiEye + 'static>(&mut self, eye: T) {
        self.shinigami_eyes.push(Arc::new(eye));
    }

    /// 设置死神之眼持续时间（不能超过6秒）
    pub fn set_vision_duration(&mut self, duration: Duration) {
        if duration <= Duration::from_secs(6) {
            self.vision_duration = duration;
        } else {
            println!("⚠️ 死神之眼持续时间不能超过6秒，使用默认值6秒");
            self.vision_duration = Duration::from_secs(6);
        }
    }

    /// 并行激活所有死神之眼
    pub async fn activate_all(&self) -> HashMap<String, Vec<Box<dyn ShinigamiEyeResult>>> {
        let mut results = HashMap::new();

        // 过滤激活的死神之眼
        let active_eyes: Vec<_> = self
            .shinigami_eyes
            .iter()
            .filter(|eye| eye.is_enabled())
            .collect();

        if active_eyes.is_empty() {
            println!("😴 没有激活的死神之眼，琉克感到无聊...");
            return results;
        }

        println!("👁️‍🗨️ 激活死神之眼，共 {} 只眼睛在观察", active_eyes.len());

        // 创建异步任务
        let tasks: Vec<_> = active_eyes
            .iter()
            .map(|eye| {
                let eye = Arc::clone(eye);
                async move {
                    let name = eye.name();
                    match timeout(self.vision_duration, eye.identify()).await {
                        Ok(results) => {
                            println!("👁️ {} 死神之眼激活，发现 {} 个目标", name, results.len());
                            for result in &results {
                                println!(
                                    "   👤 {}: {} (寿命: {})",
                                    result.source(),
                                    result.name(),
                                    result.lifespan()
                                );
                            }
                            (name.to_string(), results)
                        }
                        Err(_) => {
                            println!("⚠️ {} 死神之眼超时，失去连接", name);
                            (name.to_string(), Vec::new())
                        }
                    }
                }
            })
            .collect();

        // 并行执行所有死神之眼
        let all_results = join_all(tasks).await;

        // 收集死神之眼的观察结果
        for (name, result) in all_results {
            if !result.is_empty() {
                results.insert(name, result);
            }
        }

        if results.is_empty() {
            println!("😈 琉克：这些人类都躲起来了，真无聊...");
        } else {
            let total_targets: usize = results.values().map(|v| v.len()).sum();
            println!("👁️‍🗨️ 死神之眼观察完成，共发现 {} 个目标", total_targets);
        }

        results
    }
}
