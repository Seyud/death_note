use death_note::guidance::guidance_async::{
    DeathNoteDecision, DeathNoteTarget, RyukGuidanceSystem,
};
use death_note::guidance::partition_ops::AndroidPartitionOperator;
use std::collections::HashMap;
use tokio;

/// 演示 Android 分区还原功能的示例
///
/// 此示例展示了如何使用 Ryuk 制导系统来执行 Android 分区还原操作：
/// - A/B 设备：从另一个槽位复制 boot/init_boot 分区
/// - VAB 设备：虚拟 A/B 分区操作
/// - A-only 设备：boot 和 recovery 分区交换
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 死亡笔记 - Android 分区还原演示");
    println!("{}", "=".repeat(50));

    // 创建 Ryuk 制导系统实例
    let ryuk = RyukGuidanceSystem::new();
    println!("\n📖 Ryuk 制导系统已初始化");

    // 检查分区操作能力
    check_partition_capabilities().await;

    // 演示 Ryuk 的基本功能
    demonstrate_ryuk_features(&ryuk).await;

    // 模拟发现黑名单用户的场景
    let mock_decision = create_mock_blacklist_decision();

    // 执行死神审判（包含分区还原）
    println!("\n⚖️ 开始执行死神审判...");
    let result = ryuk.execute_shinigami_judgment(&mock_decision).await;

    // 展示审判结果
    display_judgment_result(&result);

    println!("\n🍎 Ryuk 最终状态:");
    println!("   苹果总数: {}", ryuk.get_apple_count());
    println!("   厌倦状态: {}", if ryuk.is_bored() { "是" } else { "否" });

    println!("\n✨ 演示完成！");
    Ok(())
}

/// 检查分区操作能力
async fn check_partition_capabilities() {
    println!("\n🔍 检查 Android 分区操作能力...");

    match AndroidPartitionOperator::new() {
        Ok(operator) => {
            println!("✅ Android 分区操作器初始化成功");
            println!("   设备类型: {:?}", operator.device_type);
            println!("   当前槽位: {}", operator.current_slot);

            // 在非特权环境中，实际的分区操作会失败，但我们可以展示检测逻辑
            println!("   ℹ️  在非特权环境中，实际分区操作需要 root 权限");

            match operator.device_type {
                death_note::guidance::partition_ops::DeviceType::AB => {
                    println!("   📱 A/B 设备: 支持槽位间分区复制");
                }
                death_note::guidance::partition_ops::DeviceType::VAB => {
                    println!("   📱 VAB 设备: 支持虚拟 A/B 分区操作");
                }
                death_note::guidance::partition_ops::DeviceType::AOnly => {
                    println!("   📱 A-only 设备: 支持 boot/recovery 分区交换");
                }
            }
        }
        Err(e) => {
            println!("⚠️  分区操作器初始化失败: {}", e);
            println!("   这通常表示不在 Android 环境中或权限不足");
        }
    }
}

/// 演示 Ryuk 的特色功能
async fn demonstrate_ryuk_features(ryuk: &RyukGuidanceSystem) {
    println!("\n🍎 演示 Ryuk 的特色功能...");

    println!(
        "   初始厌倦状态: {}",
        if ryuk.is_bored() { "厌倦" } else { "愉悦" }
    );

    // 让 Ryuk 吃几个苹果
    for i in 1..=3 {
        ryuk.eat_apple();
        println!("   🍎 Ryuk 吃掉第 {} 个苹果", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!(
        "   当前厌倦状态: {}",
        if ryuk.is_bored() { "厌倦" } else { "愉悦" }
    );
}

/// 创建模拟的黑名单决策
fn create_mock_blacklist_decision() -> DeathNoteDecision {
    let mut summary = HashMap::new();
    summary.insert("QQ".to_string(), vec!["12345678".to_string()]);
    summary.insert("Telegram".to_string(), vec!["evil_user".to_string()]);
    summary.insert("CoolApk".to_string(), vec!["bad_actor".to_string()]);

    let death_targets = vec![
        DeathNoteTarget {
            source: "QQ".to_string(),
            name: "12345678".to_string(),
            lifespan: "7天".to_string(),
        },
        DeathNoteTarget {
            source: "Telegram".to_string(),
            name: "evil_user".to_string(),
            lifespan: "3天".to_string(),
        },
        DeathNoteTarget {
            source: "CoolApk".to_string(),
            name: "bad_actor".to_string(),
            lifespan: "1天".to_string(),
        },
    ];

    DeathNoteDecision::Execute {
        death_targets,
        summary,
    }
}

/// 展示审判结果
fn display_judgment_result(result: &death_note::guidance::guidance_async::ShinigamiResult) {
    println!("\n📊 死神审判结果:");

    match result {
        death_note::guidance::guidance_async::ShinigamiResult::Skipped => {
            println!("   结果: 跳过审判");
            println!("   原因: Ryuk 感到无聊，没有发现值得审判的目标");
        }
        death_note::guidance::guidance_async::ShinigamiResult::Executed {
            souls_collected,
            escaped_souls,
            targets_judged,
        } => {
            println!("   结果: 审判已执行");
            println!("   被审判目标总数: {}", targets_judged);
            println!("   成功收割的灵魂: {:?}", souls_collected);

            if !escaped_souls.is_empty() {
                println!("   逃脱的灵魂:");
                for (soul, reason) in escaped_souls {
                    println!("     - {}: {}", soul, reason);
                }
            }

            let success_rate = if souls_collected.len() + escaped_souls.len() > 0 {
                (souls_collected.len() as f64
                    / (souls_collected.len() + escaped_souls.len()) as f64)
                    * 100.0
            } else {
                0.0
            };

            println!("   审判成功率: {:.1}%", success_rate);
        }
    }
}
