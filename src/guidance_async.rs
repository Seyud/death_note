use crate::blacklist::manager::DeathNote;
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
}

impl RyukGuidanceSystem {
    pub fn new() -> Self {
        Self {
            death_note: DeathNote::new(),
            apple_count: AtomicUsize::new(0),
            boredom_level: AtomicUsize::new(100), // 初始厌倦值较高，符合琉克性格
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
        println!("👁️‍🗨️ Ryuk: *无聊地翻着死亡笔记*...");

        if self.is_bored() {
            println!("😈 Ryuk: 人类的世界真是无聊啊...让我看看有什么有趣的事情");
            self.eat_apple();
        }

        // 统计所有识别结果
        let mut total_targets = 0;
        for (source, source_results) in &results {
            total_targets += source_results.len();
            if !source_results.is_empty() {
                println!(
                    "   📊 {}: {} 个目标被死神之眼发现",
                    source,
                    source_results.len()
                );
            }
        }

        if total_targets > 0 {
            println!("   🔍 开始检查死亡笔记...");
        }

        let mut death_targets = Vec::new();
        let mut summary = HashMap::new();

        // 遍历所有识别结果，寻找死亡笔记上的名字
        for (source, source_results) in &results {
            let mut source_targets = Vec::new();

            for result in source_results {
                let name = result.name();
                let source_name = result.source();
                let lifespan = result.lifespan();

                // 根据来源类型检查死亡笔记
                let is_in_death_note = match source_name {
                    "酷安" => self.death_note.is_coolapk_target(name),
                    "Telegram" => self.death_note.is_telegram_target(name),
                    "QQ" => self.death_note.is_qq_target(name),
                    _ => false,
                };

                if is_in_death_note {
                    death_targets.push(DeathNoteTarget {
                        source: source_name.to_string(),
                        name: name.to_string(),
                        lifespan: lifespan.to_string(),
                    });
                    source_targets.push(name.to_string());
                    println!("   ⚰️ {}的名字出现在死亡笔记上: {}", source_name, name);
                }
            }

            if !source_targets.is_empty() {
                summary.insert(source.clone(), source_targets);
            }
        }

        if death_targets.is_empty() {
            println!("😴 Ryuk: 没有值得行动的名字，继续观察...");
            self.boredom_level.fetch_add(5, Ordering::Relaxed);
            DeathNoteDecision::Skip
        } else {
            println!(
                "😈 Ryuk: 有趣！{} 个名字被记录在死亡笔记上",
                death_targets.len()
            );
            for target in &death_targets {
                println!(
                    "   📝 {}: {} (剩余寿命: {})",
                    target.source, target.name, target.lifespan
                );
            }
            self.eat_apple(); // 发现有趣的事情，吃苹果庆祝

            DeathNoteDecision::Execute {
                death_targets,
                summary,
            }
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

                let mut souls_collected = Vec::new();
                let mut escaped_souls = Vec::new();

                // 执行死亡笔记的审判 - 分区还原作为"灵魂收割"的象征
                println!("🔮 启动灵魂收割仪式...");

                let boot_result = self.harvest_boot_partition_async().await;
                let init_boot_result = self.harvest_init_boot_partition_async().await;

                match boot_result {
                    Ok(_) => {
                        println!("⚰️ boot分区灵魂收割完成");
                        souls_collected.push("boot".to_string());
                    }
                    Err(e) => {
                        println!("💨 boot分区灵魂逃脱: {}", e);
                        escaped_souls.push(("boot".to_string(), e.to_string()));
                    }
                }

                match init_boot_result {
                    Ok(_) => {
                        println!("⚰️ init_boot分区灵魂收割完成");
                        souls_collected.push("init_boot".to_string());
                    }
                    Err(e) => {
                        println!("💨 init_boot分区灵魂逃脱: {}", e);
                        escaped_souls.push(("init_boot".to_string(), e.to_string()));
                    }
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
        }
    }

    /// 异步收割boot分区灵魂
    async fn harvest_boot_partition_async(&self) -> Result<(), std::io::Error> {
        println!("🔮 Ryuk正在收割boot分区的灵魂...");
        // 模拟死神收割灵魂的异步操作
        tokio::time::sleep(tokio::time::Duration::from_millis(666)).await;

        // 琉克特有的审判方式
        if self.is_bored() {
            println!("😈 Ryuk: 这个灵魂看起来很有趣...");
        }

        // TODO: 实际实现 - 使用tokio的异步文件操作或系统调用
        // 象征性地将boot分区还原作为"灵魂收割"

        Ok(())
    }

    /// 异步收割init_boot分区灵魂
    async fn harvest_init_boot_partition_async(&self) -> Result<(), std::io::Error> {
        println!("🔮 Ryuk正在收割init_boot分区的灵魂...");
        // 模拟死神收割灵魂的异步操作
        tokio::time::sleep(tokio::time::Duration::from_millis(666)).await;

        // 琉克特有的审判方式
        if self.is_bored() {
            println!("😈 Ryuk: 又一个灵魂回归死神界...");
        }

        // TODO: 实际实现 - 使用tokio的异步文件操作或系统调用
        // 象征性地将init_boot分区还原作为"灵魂收割"

        Ok(())
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
