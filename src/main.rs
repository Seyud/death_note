mod blacklist;
mod guidance_async;
mod identification;

#[tokio::main]
async fn main() {
    println!("death_note - 异步并行识别系统");
    println!();

    // 创建识别管理器
    let mut manager = identification::IdentificationManager::new();
    manager.set_timeout(std::time::Duration::from_secs(3));

    // 注册所有识别器
    manager.add_identifier(identification::CoolapkIdentifier::new());
    manager.add_identifier(identification::TelegramIdentifier::new());
    manager.add_identifier(identification::QQAsyncIdentifier::new());

    // 并行执行所有识别器
    let results = manager.run_all().await;

    // 使用异步制导系统处理结果
    let guidance_system = guidance_async::AsyncGuidanceSystem::new();
    let decision = guidance_system
        .process_identification_results(results)
        .await;
    let guidance_result = guidance_system.execute_guidance(&decision).await;

    // 显示最终结果
    println!();
    println!("📊 系统执行完成:");
    match guidance_result {
        guidance_async::GuidanceResult::Skipped => {
            println!("✅ 系统安全：未检测到威胁，跳过操作");
        }
        guidance_async::GuidanceResult::Executed {
            successes,
            failures,
            blacklisted_count,
        } => {
            println!("🎯 执行完成：处理了 {} 个黑名单ID", blacklisted_count);
            if !successes.is_empty() {
                println!("✅ 成功操作：{:?}", successes);
            }
            if !failures.is_empty() {
                println!("❌ 失败操作：{:?}", failures);
            }
        }
    }
}
