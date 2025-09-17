//! 云控系统测试
//! 测试云控功能和与现有黑名单系统的集成

use death_note::blacklist::DeathNote;
use death_note::cloud_control::{CloudControlConfig, CloudControlManager};

#[tokio::test]
async fn test_cloud_control_basic() {
    // 创建测试配置
    let mut config = CloudControlConfig::default();
    config.enabled = false; // 测试时禁用真实网络请求

    // 创建云控管理器
    let cloud_manager = CloudControlManager::new(config);
    assert!(cloud_manager.is_ok());
}

#[tokio::test]
async fn test_death_note_with_cloud_control() {
    // 测试仅本地模式
    let death_note_local = DeathNote::new();

    // 这些应该使用同步方法
    let is_local_target = death_note_local.is_coolapk_target_local_only("1234567");
    println!("本地模式检查结果: {}", is_local_target);

    // 测试云控模式（使用禁用的配置，避免真实网络请求）
    let mut config = CloudControlConfig::default();
    config.enabled = false;

    match DeathNote::new_with_cloud_control(config).await {
        Ok(death_note_cloud) => {
            // 这些应该使用异步方法
            let is_cloud_target = death_note_cloud.is_coolapk_target("1234567").await;
            println!("云控模式检查结果: {}", is_cloud_target);

            // 打印状态信息
            death_note_cloud.print_status().await;
        }
        Err(e) => {
            println!("创建云控死亡笔记失败: {}", e);
        }
    }
}

#[tokio::test]
async fn test_data_separation() {
    println!("测试数据分离机制...");

    // 创建本地模式实例
    let death_note_local = DeathNote::new();

    // 测试本地数据
    let local_coolapk = death_note_local.is_coolapk_target_local_only("1234567");
    let local_qq = death_note_local.is_qq_target_local_only("123456789");
    let local_telegram = death_note_local.is_telegram_target_local_only("100000000");

    println!("本地数据检查:");
    println!("  酷安 1234567: {}", local_coolapk);
    println!("  QQ 123456789: {}", local_qq);
    println!("  Telegram 100000000: {}", local_telegram);

    // 创建禁用云控的实例进行测试
    let mut config = CloudControlConfig::default();
    config.enabled = false;

    if let Ok(death_note_cloud) = DeathNote::new_with_cloud_control(config).await {
        // 测试云控数据（应该为false，因为禁用了）
        let cloud_coolapk = death_note_cloud
            .is_coolapk_target_cloud_only("cloud_user_1")
            .await;
        let cloud_qq = death_note_cloud.is_qq_target_cloud_only("888888888").await;
        let cloud_telegram = death_note_cloud
            .is_telegram_target_cloud_only("400000000")
            .await;

        println!("云控数据检查（禁用状态）:");
        println!("  酷安 cloud_user_1: {}", cloud_coolapk);
        println!("  QQ 888888888: {}", cloud_qq);
        println!("  Telegram 400000000: {}", cloud_telegram);
    }
}
