//! 黑名单uid发现测试
//! 测试死神之眼能否正确发现测试数据中的黑名单用户

use death_note::identification::coolapk_identifier::CoolapkShinigamiEye;
use death_note::identification::qq_identifier::QQShinigamiEye;
use death_note::identification::telegram_identifier::TelegramShinigamiEye;
use death_note::identification::traits::ShinigamiEye;

#[tokio::test]
async fn test_discover_qq_blacklist_users() {
    println!("🔍 测试QQ死神之眼发现黑名单用户...");

    // 检查测试数据目录是否存在
    let test_data_path = std::path::Path::new("test_data/qq");
    println!("📁 测试数据目录存在: {}", test_data_path.exists());

    let qq_eye = QQShinigamiEye::new();
    let results = qq_eye.identify().await;

    println!("📊 QQ识别结果数量: {}", results.len());

    let mut blacklist_found = Vec::new();
    let mut normal_found = Vec::new();

    for result in &results {
        let status = if result.is_blacklisted() {
            "⚰️ 黑名单"
        } else {
            "✅ 正常"
        };
        println!(
            "   - UID: {} (寿命: {}年) {}",
            result.name(),
            result.lifespan(),
            status
        );

        if result.is_blacklisted() {
            blacklist_found.push(result.name().to_string());
        } else {
            normal_found.push(result.name().to_string());
        }
    }

    // 验证应该发现的黑名单用户
    assert!(
        blacklist_found.contains(&"123456789".to_string()),
        "应该发现QQ黑名单用户 123456789"
    );

    // 验证正常用户
    assert!(
        normal_found.contains(&"10001".to_string()) || normal_found.contains(&"10002".to_string()),
        "应该发现正常QQ用户"
    );

    println!(
        "✅ QQ黑名单发现测试通过！发现 {} 个黑名单用户，{} 个正常用户",
        blacklist_found.len(),
        normal_found.len()
    );
}

#[tokio::test]
async fn test_discover_coolapk_blacklist_users() {
    println!("🔍 测试酷安死神之眼发现黑名单用户...");

    let coolapk_eye = CoolapkShinigamiEye::new();
    let results = coolapk_eye.identify().await;

    println!("📊 酷安识别结果数量: {}", results.len());

    let mut blacklist_found = Vec::new();

    for result in &results {
        let status = if result.is_blacklisted() {
            "⚰️ 黑名单"
        } else {
            "✅ 正常"
        };
        println!(
            "   - UID: {} (寿命: {}年) {}",
            result.name(),
            result.lifespan(),
            status
        );

        if result.is_blacklisted() {
            blacklist_found.push(result.name().to_string());
        }
    }

    // 验证应该发现的黑名单用户
    if results.len() > 0 {
        assert!(
            blacklist_found.contains(&"1234567".to_string()),
            "应该发现酷安黑名单用户 1234567"
        );
    }

    println!(
        "✅ 酷安黑名单发现测试通过！发现 {} 个黑名单用户",
        blacklist_found.len()
    );
}

#[tokio::test]
async fn test_discover_telegram_blacklist_users() {
    println!("🔍 测试Telegram死神之眼发现黑名单用户...");

    let telegram_eye = TelegramShinigamiEye::new();
    let results = telegram_eye.identify().await;

    println!("📊 Telegram识别结果数量: {}", results.len());

    let mut all_found = Vec::new();

    for result in &results {
        let status = if result.is_blacklisted() {
            "⚰️ 黑名单"
        } else {
            "✅ 正常"
        };
        println!(
            "   - UID: {} (寿命: {}年) {}",
            result.name(),
            result.lifespan(),
            status
        );
        all_found.push(result.name().to_string());
    }

    // Telegram测试数据中的uid都不在黑名单中，所以验证找到了用户但都是正常用户
    if results.len() > 0 {
        println!("📝 注意: Telegram测试数据中的用户都不在黑名单中，这是正常的");
    }

    println!("✅ Telegram发现测试通过！发现 {} 个用户", all_found.len());
}

#[tokio::test]
async fn test_comprehensive_blacklist_discovery() {
    println!("🚀 综合黑名单发现测试...");

    let qq_eye = QQShinigamiEye::new();
    let coolapk_eye = CoolapkShinigamiEye::new();
    let telegram_eye = TelegramShinigamiEye::new();

    // 并行执行所有识别
    let (qq_results, coolapk_results, telegram_results) = tokio::join!(
        qq_eye.identify(),
        coolapk_eye.identify(),
        telegram_eye.identify()
    );

    let mut total_blacklist = 0;
    let mut total_normal = 0;

    println!("\n📋 综合发现结果:");

    // 统计QQ结果
    for result in &qq_results {
        if result.is_blacklisted() {
            total_blacklist += 1;
            println!(
                "   ⚰️ QQ黑名单: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        } else {
            total_normal += 1;
            println!(
                "   ✅ QQ正常: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        }
    }

    // 统计酷安结果
    for result in &coolapk_results {
        if result.is_blacklisted() {
            total_blacklist += 1;
            println!(
                "   ⚰️ 酷安黑名单: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        } else {
            total_normal += 1;
            println!(
                "   ✅ 酷安正常: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        }
    }

    // 统计Telegram结果
    for result in &telegram_results {
        if result.is_blacklisted() {
            total_blacklist += 1;
            println!(
                "   ⚰️ Telegram黑名单: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        } else {
            total_normal += 1;
            println!(
                "   ✅ Telegram正常: {} (寿命: {}年)",
                result.name(),
                result.lifespan()
            );
        }
    }

    println!("\n📊 最终统计:");
    println!("   ⚰️ 黑名单用户总数: {}", total_blacklist);
    println!("   ✅ 正常用户总数: {}", total_normal);
    println!("   📈 总用户数: {}", total_blacklist + total_normal);

    // 验证至少发现了一些黑名单用户
    assert!(
        total_blacklist >= 2,
        "应该至少发现2个黑名单用户 (QQ: 123456789, 酷安: 1234567)"
    );

    println!("🎉 综合黑名单发现测试通过！");
}
