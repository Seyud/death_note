use death_note::identification::coolapk_identifier::CoolapkIdentifier;
use death_note::identification::manager::IdentificationManager;
use death_note::identification::qq_identifier::QQAsyncIdentifier;
use death_note::identification::telegram_identifier::TelegramIdentifier;
use std::time::Instant;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    println!("🔍 异步并行识别系统性能测试");
    println!("========================================");

    // 创建识别管理器
    let mut manager = IdentificationManager::new();
    manager.set_timeout(Duration::from_secs(5));

    // 添加各种识别器
    manager.add_identifier(CoolapkIdentifier::new());
    manager.add_identifier(TelegramIdentifier::new());
    manager.add_identifier(QQAsyncIdentifier::new());

    println!("🚀 启动并行识别...");

    let start = Instant::now();
    let results = manager.run_all().await;
    let duration = start.elapsed();

    println!("⚡ 并行识别完成！");
    println!("📊 执行时间: {:?}", duration);
    println!("📈 识别结果数量: {} 个识别器返回结果", results.len());

    for (name, results) in results {
        println!("   - {}: {} 个结果", name, results.len());
        for result in results {
            println!("     └─ UID: {} (来源: {})", result.uid(), result.source());
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
