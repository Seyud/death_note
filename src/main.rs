// use death_note::blacklist::manager::DeathNote;
use death_note::guidance_async::RyukGuidanceSystem;
use death_note::identification::{
    coolapk_identifier::CoolapkShinigamiEye, manager::ShinigamiEyeManager,
    qq_identifier::QQShinigamiEye, telegram_identifier::TelegramShinigamiEye,
};
use std::collections::HashMap;
use std::time::Duration;

/// 显示所有被死神之眼发现的目标
fn display_shinigami_discoveries(
    results: &HashMap<String, Vec<Box<dyn death_note::identification::ShinigamiEyeResult>>>,
) {
    println!();
    println!("👁️‍🗨️ 死神之眼观察结果:");

    if results.is_empty() {
        println!("   😴 死神之眼未发现任何目标，琉克感到无聊...");
        return;
    }

    let mut total_targets = 0;

    for (source, source_results) in results {
        if !source_results.is_empty() {
            println!("   📱 {} ({} 个目标):", source, source_results.len());
            total_targets += source_results.len();

            for (index, result) in source_results.iter().enumerate() {
                println!(
                    "      {}. {} (寿命: {})",
                    index + 1,
                    result.name(),
                    result.lifespan()
                );
            }
            println!();
        }
    }

    println!("⚰️ 死神之眼总计发现 {} 个目标", total_targets);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📓 死亡笔记打开了...");
    println!("😈 Ryuk: 终于有点有趣的事情了...");
    println!();

    // 创建死神之眼管理器
    let mut eye_manager = ShinigamiEyeManager::new();
    eye_manager.set_vision_duration(Duration::from_secs(5));

    // 添加各个平台的死神之眼
    eye_manager.add_shinigami_eye(CoolapkShinigamiEye::new());
    eye_manager.add_shinigami_eye(QQShinigamiEye::new());
    eye_manager.add_shinigami_eye(TelegramShinigamiEye::new());

    // 创建琉克制导系统
    let ryuk = RyukGuidanceSystem::new();

    // 激活所有死神之眼进行识别
    println!("👁️‍🗨️ 激活死神之眼观察人类世界...");
    let results = eye_manager.activate_all().await;

    // 显示死神之眼的发现
    display_shinigami_discoveries(&results);

    // 琉克进行审判
    println!("⚰️ 琉克开始翻阅死亡笔记进行审判...");
    let decision = ryuk.ryuk_judgment(results).await;

    // 执行审判
    let final_result = ryuk.execute_shinigami_judgment(&decision).await;

    // 显示最终审判结果
    println!();
    println!("⚰️ 死神审判完成:");
    match final_result {
        death_note::guidance_async::ShinigamiResult::Skipped => {
            println!("😴 Ryuk: 今天没有有趣的灵魂...");
            println!("😈 Ryuk: 人类的世界真是越来越无聊了");
        }
        death_note::guidance_async::ShinigamiResult::Executed {
            souls_collected,
            escaped_souls,
            targets_judged,
        } => {
            println!("⚰️ 审判完成！");
            println!("📝 被审判的目标数量: {}", targets_judged);
            println!("💀 成功收割的灵魂: {:?}", souls_collected);
            if !escaped_souls.is_empty() {
                println!("💨 逃脱的灵魂: {:?}", escaped_souls);
            }
            println!("🍎 Ryuk: 今天的苹果真甜...");
            println!("😈 Ryuk: 死神界的苹果还是比不上人类的灵魂有趣");
        }
    }

    println!();
    println!("📓 死亡笔记系统运行结束");
    println!("👁️‍🗨️ 死神之眼已关闭");

    Ok(())
}
