//! 云控系统演示程序
//! 演示如何使用死亡笔记的云控功能

use death_note::blacklist::DeathNote;
use death_note::cloud_control::CloudControlConfig;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🖤 死亡笔记云控系统演示");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // 演示本地模式
    println!("\n📖 1. 本地模式演示");
    demonstrate_local_mode().await;

    // 演示云控模式
    println!("\n☁️ 2. 云控模式演示");
    demonstrate_cloud_mode().await?;

    // 演示数据分离
    println!("\n🔀 3. 数据分离演示");
    demonstrate_data_separation().await?;

    // 交互模式
    println!("\n🎮 4. 交互模式");
    interactive_mode().await?;

    Ok(())
}

async fn demonstrate_local_mode() {
    println!("创建本地模式死亡笔记实例...");
    let death_note = DeathNote::new();

    // 测试本地编译的数据
    let test_users = vec![
        ("酷安", "1234567"),
        ("QQ", "123456789"),
        ("Telegram", "100000000"),
    ];

    for (platform, user_id) in test_users {
        let result = match platform {
            "酷安" => death_note.is_coolapk_target_local_only(user_id),
            "QQ" => death_note.is_qq_target_local_only(user_id),
            "Telegram" => death_note.is_telegram_target_local_only(user_id),
            _ => false,
        };

        let status = if result {
            "✅ 在名单中"
        } else {
            "❌ 不在名单中"
        };
        println!("  {} 用户 '{}': {}", platform, user_id, status);
    }
}

async fn demonstrate_cloud_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("创建云控模式死亡笔记实例...");

    // 使用默认配置（真实的云控配置）
    let config = CloudControlConfig::default();

    match DeathNote::new_with_cloud_control(config).await {
        Ok(death_note) => {
            println!("✅ 云控模式初始化成功");

            // 打印状态信息
            death_note.print_status().await;

            // 测试一些用户
            let test_cases = vec![
                ("酷安", "cloud_user_1"),
                ("酷安", "1234567"), // 本地存在的用户
                ("QQ", "888888888"),
                ("QQ", "123456789"), // 本地存在的用户
            ];

            for (platform, user_id) in test_cases {
                let result = match platform {
                    "酷安" => death_note.is_coolapk_target(user_id).await,
                    "QQ" => death_note.is_qq_target(user_id).await,
                    "Telegram" => death_note.is_telegram_target(user_id).await,
                    _ => false,
                };

                let status = if result {
                    "✅ 在名单中"
                } else {
                    "❌ 不在名单中"
                };
                println!("  {} 用户 '{}': {}", platform, user_id, status);
            }
        }
        Err(e) => {
            println!("❌ 云控模式初始化失败: {}", e);
            println!("这可能是因为网络问题或云控仓库不存在");
        }
    }

    Ok(())
}

async fn demonstrate_data_separation() -> Result<(), Box<dyn std::error::Error>> {
    println!("演示数据分离机制...");

    let config = CloudControlConfig::default();

    match DeathNote::new_with_cloud_control(config).await {
        Ok(death_note) => {
            let test_user = "1234567"; // 存在于本地的用户

            // 仅检查本地
            let local_only = death_note.is_coolapk_target_local_only(test_user);

            // 仅检查云控
            let cloud_only = death_note.is_coolapk_target_cloud_only(test_user).await;

            // 综合检查
            let combined = death_note.is_coolapk_target(test_user).await;

            println!("  用户 '{}' 检查结果:", test_user);
            println!("    仅本地: {}", if local_only { "✅" } else { "❌" });
            println!("    仅云控: {}", if cloud_only { "✅" } else { "❌" });
            println!("    综合结果: {}", if combined { "✅" } else { "❌" });

            println!("\n  数据源分离说明:");
            println!("    - 本地数据来自编译时的 blacklist_config.toml");
            println!("    - 云控数据来自远程仓库的 blacklist.toml");
            println!("    - 综合查询会检查两个数据源，任一匹配即返回 true");
        }
        Err(e) => {
            println!("❌ 无法演示数据分离: {}", e);
        }
    }

    Ok(())
}

async fn interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("进入交互模式，输入 'quit' 退出");

    let config = CloudControlConfig::default();
    let death_note = match DeathNote::new_with_cloud_control(config).await {
        Ok(dn) => dn,
        Err(e) => {
            println!("⚠️ 云控模式不可用，使用本地模式: {}", e);
            DeathNote::new()
        }
    };

    loop {
        print!("\n请输入检查命令 (格式: platform user_id，如 'coolapk test_user'): ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" || input == "exit" {
            break;
        }

        if input == "status" {
            death_note.print_status().await;
            continue;
        }

        if input == "refresh" {
            if let Err(e) = death_note.refresh_cloud_data().await {
                println!("❌ 刷新失败: {}", e);
            }
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.len() != 2 {
            println!("❌ 格式错误，请使用: platform user_id");
            println!("   支持的平台: coolapk, qq, telegram");
            println!("   特殊命令: status (查看状态), refresh (刷新云控), quit (退出)");
            continue;
        }

        let platform = parts[0].to_lowercase();
        let user_id = parts[1];

        println!("🔍 检查中...");

        let result = match platform.as_str() {
            "coolapk" => {
                if death_note.cloud_manager().is_some() {
                    death_note.is_coolapk_target(user_id).await
                } else {
                    death_note.is_coolapk_target_local_only(user_id)
                }
            }
            "qq" => {
                if death_note.cloud_manager().is_some() {
                    death_note.is_qq_target(user_id).await
                } else {
                    death_note.is_qq_target_local_only(user_id)
                }
            }
            "telegram" => {
                if death_note.cloud_manager().is_some() {
                    death_note.is_telegram_target(user_id).await
                } else {
                    death_note.is_telegram_target_local_only(user_id)
                }
            }
            _ => {
                println!("❌ 不支持的平台: {}", platform);
                continue;
            }
        };

        let status = if result {
            "✅ 在死亡笔记上 - 此灵魂将被收割"
        } else {
            "❌ 不在死亡笔记上 - 安全"
        };

        println!("📖 {} 用户 '{}': {}", platform, user_id, status);

        if result {
            death_note.record_soul_harvest(&platform, user_id);
        }
    }

    println!("\n👋 感谢使用死亡笔记云控系统演示！");
    Ok(())
}
