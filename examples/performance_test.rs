use death_note::identification::coolapk_identifier::CoolapkShinigamiEye;
use death_note::identification::manager::ShinigamiEyeManager;
use death_note::identification::qq_identifier::QQShinigamiEye;
use death_note::identification::telegram_identifier::TelegramShinigamiEye;
use std::time::Instant;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("🔍 异步并行识别系统性能测试");
    println!("========================================");

    // 创建死神之眼管理器
    let mut manager = ShinigamiEyeManager::new();
    manager.set_vision_duration(Duration::from_secs(5));

    // 添加各种死神之眼
    manager.add_shinigami_eye(CoolapkShinigamiEye::new());
    manager.add_shinigami_eye(TelegramShinigamiEye::new());
    manager.add_shinigami_eye(QQShinigamiEye::new());

    println!("🚀 启动并行识别...");

    let start = Instant::now();
    let results = manager.activate_all().await;
    let duration = start.elapsed();

    println!("⚡ 并行识别完成！");
    println!("📊 执行时间: {:?}", duration);
    println!("📈 识别结果数量: {} 个识别器返回结果", results.len());

    for (name, results) in results {
        println!("   - {}: {} 个结果", name, results.len());
        for result in results {
            let status = if result.is_blacklisted() {
                "(黑名单)"
            } else {
                "(正常)"
            };
            println!(
                "     └─ UID: {} (来源: {}) (寿命: {}年) {}",
                result.name(),
                result.source(),
                result.lifespan(),
                status
            );
        }
    }

    println!("\n💡 性能对比:");
    println!("   - 同步执行预估时间: ~{}ms", 800 * 3);
    println!("   - 实际并行时间: {:?}", duration);
    println!(
        "   - 性能提升: ~{}x",
        (800 * 3) as f64 / duration.as_millis() as f64
    );
}
