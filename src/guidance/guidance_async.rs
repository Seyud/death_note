use crate::blacklist::manager::DeathNote;
use crate::guidance::partition_ops::{AndroidPartitionOperator, PartitionRestoreResult};
use crate::identification::ShinigamiEyeResult;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// 琉克(Ryuk)
/// 设定：死神琉克 - 因厌倦死神界而将死亡笔记丢弃至人间的死神
/// 特质：以旁观人类使用死亡笔记为乐，喜欢吃苹果
pub struct RyukGuidanceSystem {
    death_note: DeathNote,
    apple_count: AtomicUsize,
    boredom_level: AtomicUsize,
    partition_operator: Option<AndroidPartitionOperator>,
}

impl RyukGuidanceSystem {
    pub fn new() -> Self {
        let partition_operator = AndroidPartitionOperator::new().ok();
        if let Some(ref operator) = partition_operator {
            println!("🔍 Ryuk: 检测到Android设备，灵魂收割装置已就绪...");
            println!(
                "📱 设备类型: {:?}, 当前槽位: {}",
                operator.device_type, operator.current_slot
            );
        } else {
            println!("⚠️ Ryuk: 未检测到Android设备，使用模拟模式...");
        }

        Self {
            death_note: DeathNote::new(),
            apple_count: AtomicUsize::new(0),
            boredom_level: AtomicUsize::new(100), // 初始厌倦值较高，符合琉克性格
            partition_operator,
        }
    }

    /// 琉克吃苹果 - 增加愉悦感，降低厌倦
    pub fn eat_apple(&self) {
        self.apple_count.fetch_add(1, Ordering::Relaxed);
        self.boredom_level.fetch_sub(10, Ordering::Relaxed);
        let apples = self.apple_count.load(Ordering::Relaxed);
        let boredom = self.boredom_level.load(Ordering::Relaxed);
        println!("🍎 Ryuk 吃了一个苹果！总计 {}, 厌倦值: {}", apples, boredom);
    }

    /// 检查琉克是否感到厌倦
    pub fn is_bored(&self) -> bool {
        self.boredom_level.load(Ordering::Relaxed) > 50
    }

    /// 获取琉克吃掉的苹果总数
    pub fn get_apple_count(&self) -> usize {
        self.apple_count.load(Ordering::Relaxed)
    }
}

impl Default for RyukGuidanceSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl RyukGuidanceSystem {
    /// 琉克的审判 - 处理死神之眼识别结果并决定是否执行
    pub async fn ryuk_judgment(
        &self,
        results: HashMap<String, Vec<Box<dyn ShinigamiEyeResult>>>,
    ) -> DeathNoteDecision {
        println!("👁️‍🗨️ Ryuk: *观察死神之眼的发现*...");

        let mut blacklisted_count = 0;
        let mut death_targets = Vec::new();
        let mut summary = HashMap::new();

        // 统计黑名单用户并收集目标
        for (source, source_results) in &results {
            let mut source_targets = Vec::new();

            for result in source_results {
                if result.is_blacklisted() {
                    blacklisted_count += 1;
                    death_targets.push(DeathNoteTarget {
                        source: result.source().to_string(),
                        name: result.name().to_string(),
                        lifespan: result.lifespan().to_string(),
                    });
                    source_targets.push(result.name().to_string());
                    println!(
                        "⚰️ 发现黑名单目标: {} (寿命: {})",
                        result.name(),
                        result.lifespan()
                    );
                }
            }

            if !source_targets.is_empty() {
                summary.insert(source.clone(), source_targets);
            }
        }

        // 根据发现的黑名单用户数量消费苹果
        if blacklisted_count > 0 {
            println!(
                "🍎 发现 {} 个黑名单目标，琉克开始享用苹果...",
                blacklisted_count
            );
            for i in 0..blacklisted_count {
                self.eat_apple();
                println!(
                    "🍎 琉克吃掉第 {} 个苹果 (共需 {} 个)",
                    i + 1,
                    blacklisted_count
                );
            }

            DeathNoteDecision::Execute {
                death_targets,
                summary,
            }
        } else {
            println!("😴 Ryuk: 没有发现黑名单目标，继续观察...");
            self.boredom_level.fetch_add(5, Ordering::Relaxed);
            DeathNoteDecision::Skip
        }
    }

    /// 执行死亡笔记的审判
    pub async fn execute_shinigami_judgment(
        &self,
        decision: &DeathNoteDecision,
    ) -> ShinigamiResult {
        match decision {
            DeathNoteDecision::Skip => {
                println!("😴 Ryuk: *无聊地飘在空中*... 今天没有有趣的事情发生");
                if self.is_bored() {
                    println!("😈 Ryuk: 人类真是无趣啊...");
                }
                ShinigamiResult::Skipped
            }
            DeathNoteDecision::Execute {
                death_targets,
                summary,
            } => {
                println!("⚰️ Ryuk: 呵呵呵...审判的时刻到了");
                println!("📋 死亡笔记上记录的来源种类: {}", summary.len());
                self.eat_apple(); // 审判开始，吃个苹果

                let _souls_collected: Vec<String> = Vec::new();
                let _escaped_souls: Vec<(String, String)> = Vec::new();

                // 执行死亡笔记的审判 - 分区还原作为"灵魂收割"的象征
                println!("🔮 启动灵魂收割仪式...");

                let partition_result = self.harvest_boot_partition_async().await;

                match partition_result {
                    Ok(result) => {
                        let souls_collected = result.restored_partitions.clone();
                        let escaped_souls = result.failed_partitions.clone();

                        println!(
                            "⚖️ 分区还原结果: {} 成功, {} 失败",
                            result.success_count(),
                            result.failure_count()
                        );
                        println!(
                            "📱 设备类型: {:?}, 操作类型: {}",
                            result.device_type, result.operation_type
                        );

                        if result.is_success() {
                            println!("✅ 所有分区灵魂收割完成");
                        } else if result.success_count() > 0 {
                            println!("⚠️ 部分分区灵魂逃脱");
                        } else {
                            println!("❌ 所有分区灵魂都逃脱了");
                        }

                        let total_targets = death_targets.len();
                        println!(
                            "😈 Ryuk: 审判完成！共收割 {} 个灵魂，{} 个目标被记录",
                            souls_collected.len(),
                            total_targets
                        );

                        ShinigamiResult::Executed {
                            souls_collected,
                            escaped_souls,
                            targets_judged: total_targets,
                        }
                    }
                    Err(e) => {
                        println!("❌ 灵魂收割仪式失败: {}", e);
                        ShinigamiResult::Executed {
                            souls_collected: vec![],
                            escaped_souls: vec![("all_partitions".to_string(), e.to_string())],
                            targets_judged: death_targets.len(),
                        }
                    }
                }
            }
        }
    }

    /// 异步收割boot分区灵魂
    async fn harvest_boot_partition_async(&self) -> Result<PartitionRestoreResult, std::io::Error> {
        println!("🔮 Ryuk正在收割boot分区的灵魂...");
        // 模拟死神收割灵魂的异步操作
        tokio::time::sleep(tokio::time::Duration::from_millis(666)).await;

        // 琉克特有的审判方式
        if self.is_bored() {
            println!("😈 Ryuk: 这个灵魂看起来很有趣...");
        }

        // 使用死亡笔记记录灵魂收割
        self.death_note
            .record_soul_harvest("boot", "Android Boot Partition");

        // 实际实现 - 使用Android分区操作器
        if let Some(ref operator) = self.partition_operator {
            println!("🔧 Ryuk: 开始真正的灵魂收割仪式...");
            let result = operator.restore_partitions_async().await?;

            if result.is_success() {
                println!(
                    "✅ Ryuk: 灵魂收割完成！恢复了 {} 个分区",
                    result.success_count()
                );
            } else {
                println!(
                    "⚠️ Ryuk: 部分灵魂逃脱了，{} 个成功，{} 个失败",
                    result.success_count(),
                    result.failure_count()
                );
            }

            Ok(result)
        } else {
            println!("🎭 Ryuk: 模拟模式 - 象征性的灵魂收割");
            // 返回模拟结果
            Ok(PartitionRestoreResult {
                device_type: crate::guidance::partition_ops::DeviceType::AOnly,
                restored_partitions: vec!["boot".to_string()],
                failed_partitions: vec![],
                operation_type: "模拟操作".to_string(),
            })
        }
    }
}

/// 死亡笔记决策 - 琉克是否决定执行审判
#[derive(Debug)]
pub enum DeathNoteDecision {
    Skip, // 琉克选择观察，不干预
    Execute {
        death_targets: Vec<DeathNoteTarget>,
        summary: HashMap<String, Vec<String>>,
    }, // 琉克执行死亡笔记的审判
}

/// 死亡笔记目标详情 - 被琉克记录在死亡笔记上的目标
#[derive(Debug)]
pub struct DeathNoteTarget {
    pub source: String,   // 来源平台
    pub name: String,     // 目标名称
    pub lifespan: String, // 剩余寿命（死神之眼可见）
}

/// 死神审判结果 - 琉克的最终审判
#[derive(Debug)]
pub enum ShinigamiResult {
    Skipped, // 琉克选择旁观，死神界的生活太无聊了
    Executed {
        souls_collected: Vec<String>,         // 成功收割的灵魂（分区）
        escaped_souls: Vec<(String, String)>, // 逃脱的灵魂及原因
        targets_judged: usize,                // 被审判的目标总数
    }, // 琉克执行了审判，灵魂回归死神界
}
