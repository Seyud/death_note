mod blacklist;
mod guidance_async;
mod identification;

use std::collections::HashMap;

/// 显示所有识别到的UID
fn display_all_identified_uids(
    results: &HashMap<String, Vec<Box<dyn identification::IdentificationResult>>>,
) {
    println!();
    println!("📋 识别结果汇总:");

    if results.is_empty() {
        println!("   ❌ 未识别到任何UID");
        return;
    }

    let mut total_count = 0;

    for (source, source_results) in results {
        if !source_results.is_empty() {
            println!("   📱 {} ({} 个UID):", source, source_results.len());
            total_count += source_results.len();

            for (index, result) in source_results.iter().enumerate() {
                // 只显示 UID
                println!("      {}. {}", index + 1, result.uid());
            }
            println!();
        }
    }

    println!("✅ 总计识别到 {} 个UID", total_count);
}

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

    // 显示所有识别到的UID
    display_all_identified_uids(&results);

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
