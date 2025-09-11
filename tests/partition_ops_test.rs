use death_note::guidance::guidance_async::RyukGuidanceSystem;
use death_note::guidance::partition_ops::{AndroidPartitionOperator, DeviceType};

#[tokio::test]
async fn test_partition_operations() {
    println!("🧪 测试 Android 分区操作功能");

    // 测试分区操作器创建
    let operator_result = AndroidPartitionOperator::new();
    match operator_result {
        Ok(operator) => {
            println!("✅ 分区操作器创建成功");
            println!("📱 设备类型: {:?}", operator.device_type);
            println!("🔄 当前槽位: {}", operator.current_slot);

            // 测试分区还原（模拟模式）
            match operator.restore_partitions_async().await {
                Ok(result) => {
                    println!("✅ 分区还原测试完成");
                    println!("📊 操作类型: {}", result.operation_type);
                    println!("✅ 成功分区: {:?}", result.restored_partitions);
                    if !result.failed_partitions.is_empty() {
                        println!("❌ 失败分区: {:?}", result.failed_partitions);
                    }
                }
                Err(e) => {
                    println!("❌ 分区还原失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("⚠️ 分区操作器创建失败（可能非Android环境）: {}", e);
        }
    }
}

#[tokio::test]
async fn test_ryuk_guidance_with_partitions() {
    println!("🧪 测试 Ryuk 制导系统与分区操作集成");

    let ryuk = RyukGuidanceSystem::new();

    // 测试 Ryuk 的基本功能
    println!("🍎 Ryuk 初始苹果数量: {}", ryuk.get_apple_count());
    println!("😴 Ryuk 是否感到厌倦: {}", ryuk.is_bored());

    // 让 Ryuk 吃几个苹果
    for i in 1..=3 {
        ryuk.eat_apple();
        println!("🍎 Ryuk 吃掉第 {} 个苹果", i);
    }

    println!("🍎 Ryuk 总苹果数: {}", ryuk.get_apple_count());
    println!("😴 Ryuk 厌倦状态: {}", ryuk.is_bored());
}

#[test]
fn test_device_type_detection() {
    println!("🧪 测试设备类型检测");

    // 测试设备类型枚举
    let device_types = vec![DeviceType::AB, DeviceType::VAB, DeviceType::AOnly];

    for device_type in device_types {
        println!("📱 设备类型: {:?}", device_type);

        // 测试相等性比较
        let same_type = device_type.clone();
        assert_eq!(device_type, same_type);
        println!("✅ 设备类型比较测试通过");
    }
}

#[tokio::test]
async fn test_partition_restore_result() {
    use death_note::guidance::partition_ops::PartitionRestoreResult;

    println!("🧪 测试分区还原结果");

    // 测试成功的分区还原结果
    let success_result = PartitionRestoreResult {
        device_type: DeviceType::AB,
        restored_partitions: vec!["boot".to_string(), "init_boot".to_string()],
        failed_partitions: vec![],
        operation_type: "A/B槽位交换".to_string(),
    };

    assert!(success_result.is_success());
    assert_eq!(success_result.success_count(), 2);
    assert_eq!(success_result.failure_count(), 0);
    println!("✅ 成功场景测试通过");

    // 测试部分失败的分区还原结果
    let partial_failure_result = PartitionRestoreResult {
        device_type: DeviceType::VAB,
        restored_partitions: vec!["boot".to_string()],
        failed_partitions: vec![("init_boot".to_string(), "分区不存在".to_string())],
        operation_type: "VAB分区操作".to_string(),
    };

    assert!(!partial_failure_result.is_success());
    assert_eq!(partial_failure_result.success_count(), 1);
    assert_eq!(partial_failure_result.failure_count(), 1);
    println!("✅ 部分失败场景测试通过");

    // 测试完全失败的分区还原结果
    let complete_failure_result = PartitionRestoreResult {
        device_type: DeviceType::AOnly,
        restored_partitions: vec![],
        failed_partitions: vec![
            ("boot".to_string(), "权限不足".to_string()),
            ("recovery".to_string(), "分区损坏".to_string()),
        ],
        operation_type: "A-only分区交换".to_string(),
    };

    assert!(!complete_failure_result.is_success());
    assert_eq!(complete_failure_result.success_count(), 0);
    assert_eq!(complete_failure_result.failure_count(), 2);
    println!("✅ 完全失败场景测试通过");
}
