use anyhow::Context;
// use death_note::blacklist::manager::DeathNote;
use death_note::cloud_control::CloudControlManager;
use death_note::guidance::guidance_async::RyukGuidanceSystem;
use death_note::identification::{
    coolapk_identifier::CoolapkShinigamiEye, manager::ShinigamiEyeManager,
    qq_identifier::QQShinigamiEye, telegram_identifier::TelegramShinigamiEye,
    wechat_identifier::WeChatShinigamiEye,
};
use std::collections::HashMap;
use std::time::Duration;

/// 加载云控配置（从编译时嵌入的配置）
async fn load_cloud_config() -> anyhow::Result<Option<CloudControlManager>> {
    match CloudControlManager::new_from_embedded_config() {
        Ok(manager) => {
            println!("✅ 云控配置已从编译时嵌入数据加载");
            Ok(Some(manager))
        }
        Err(e) => {
            // 对于主程序，云控是可选功能，可以容忍创建失败，只打印错误信息。
            println!("❌ 云控管理器创建失败: {}", e);
            Ok(None)
        }
    }
}

/// 应用云控黑名单到识别结果
async fn apply_cloud_blacklist(
    results: &mut HashMap<String, Vec<Box<dyn death_note::identification::ShinigamiEyeResult>>>,
    cloud_manager: &CloudControlManager,
) {
    use death_note::cloud_control::Platform;

    let mut total_cloud_marked = 0;

    for (source, targets) in results.iter_mut() {
        let platform = match source.as_str() {
            "Coolapk死神之眼" => Some(Platform::Coolapk),
            "QQ死神之眼" => Some(Platform::QQ),
            "Telegram死神之眼" => Some(Platform::Telegram),
            "WeChat死神之眼" => Some(Platform::WeChat),
            _ => None,
        };

        if let Some(platform) = platform {
            for target in targets.iter_mut() {
                if cloud_manager
                    .is_target(platform.clone(), target.name())
                    .await
                {
                    // 这里我们需要一个方法来标记目标为云控黑名单
                    // 由于trait限制，我们暂时只能在输出时体现
                    total_cloud_marked += 1;
                }
            }
        }
    }

    if total_cloud_marked > 0 {
        println!("☁️ 云控系统标记了 {} 个目标为黑名单", total_cloud_marked);
    } else {
        println!("☁️ 云控系统未发现额外的黑名单目标");
    }
}

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
                let lifespan_display = if result.is_blacklisted() {
                    format!("{}年(黑名单)", result.lifespan())
                } else {
                    format!("{}年", result.lifespan())
                };
                println!(
                    "      {}. {} (寿命: {})",
                    index + 1,
                    result.name(),
                    lifespan_display
                );
            }
            println!();
        }
    }

    println!("⚰️ 死神之眼总计发现 {} 个目标", total_targets);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("📓 死亡笔记打开了...");
    println!("😈 Ryuk: 终于有点有趣的事情了...");
    println!();

    // 初始化云控系统（使用编译时嵌入的配置）
    println!("☁️ 正在初始化云控系统...");
    let cloud_manager = load_cloud_config().await.context("无法加载云控配置")?;

    if let Some(ref manager) = cloud_manager {
        if let Err(e) = manager.initialize().await {
            println!("⚠️ 云控系统初始化失败: {:#}", e);
        } else {
            // 显示云控状态
            manager.print_status().await;
        }
    }
    println!();

    // 创建死神之眼管理器
    let mut eye_manager = ShinigamiEyeManager::new();
    eye_manager.set_vision_duration(Duration::from_secs(5));

    // 添加各个平台的死神之眼
    eye_manager.add_shinigami_eye(CoolapkShinigamiEye::new());
    eye_manager.add_shinigami_eye(QQShinigamiEye::new());
    eye_manager.add_shinigami_eye(TelegramShinigamiEye::new());
    eye_manager.add_shinigami_eye(WeChatShinigamiEye::new());

    // 创建琉克制导系统
    let ryuk = RyukGuidanceSystem::new();

    // 激活所有死神之眼进行识别
    println!("👁️‍🗨️ 激活死神之眼观察人类世界...");
    let mut results = eye_manager.activate_all().await;

    // 如果有云控系统，进行云控检查和黑名单标记
    if let Some(ref manager) = cloud_manager {
        println!("☁️ 正在进行云控黑名单检查...");
        apply_cloud_blacklist(&mut results, manager).await;
    }

    // 显示死神之眼的发现
    display_shinigami_discoveries(&results);

    // 琉克进行审判
    println!("📖 翻查死亡笔记");
    println!("⚰️ 琉克开始翻阅死亡笔记进行审判...");
    let decision = ryuk.ryuk_judgment(results).await;

    // 执行审判
    let final_result = ryuk.execute_shinigami_judgment(&decision).await;

    // 显示最终审判结果
    println!();
    println!("⚰️ 死神审判完成:");
    match final_result {
        death_note::guidance::guidance_async::ShinigamiResult::Skipped => {
            println!("😴 Ryuk: 今天没有有趣的灵魂...");
            println!("😈 Ryuk: 人类的世界真是越来越无聊了");
        }
        death_note::guidance::guidance_async::ShinigamiResult::Executed {
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
