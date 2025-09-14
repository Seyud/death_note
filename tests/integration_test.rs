//! 寿命和苹果机制集成测试

use death_note::blacklist::manager::DeathNote;
use death_note::guidance::{DeathNoteDecision, RyukGuidanceSystem};
use death_note::identification::lifespan_calculator::LifespanCalculator;
use death_note::identification::traits::{GenericShinigamiEyeResult, ShinigamiEyeResult};
use std::collections::HashMap;

#[tokio::test]
async fn test_lifespan_and_apple_mechanism() {
    println!("🧪 测试寿命和苹果机制...");

    // 创建测试组件
    let lifespan_calculator = LifespanCalculator::new();
    let death_note = DeathNote::new();

    // 创建测试用户数据
    let normal_user = "666888"; // 非黑名单用户
    let blacklisted_user = "1234567"; // 黑名单用户

    // 测试寿命计算
    println!("📏 测试寿命计算机制...");
    let normal_lifespan = lifespan_calculator.calculate_lifespan(normal_user, false);
    let blacklisted_lifespan = lifespan_calculator.calculate_lifespan(blacklisted_user, true);

    println!("   正常用户 {} 寿命: {} 年", normal_user, normal_lifespan);
    println!(
        "   黑名单用户 {} 寿命: {} 年",
        blacklisted_user, blacklisted_lifespan
    );

    // 验证寿命规则
    assert!(
        normal_lifespan >= 50 && normal_lifespan <= 80,
        "正常用户寿命应在50-80之间"
    );
    assert_eq!(blacklisted_lifespan, 0, "黑名单用户寿命应为0");

    // 创建识别结果
    let mut results = HashMap::new();
    let mut test_results: Vec<Box<dyn ShinigamiEyeResult>> = Vec::new();

    // 添加正常用户
    test_results.push(Box::new(GenericShinigamiEyeResult::new(
        normal_user.to_string(),
        "酷安".to_string(),
        normal_lifespan,
        death_note.is_coolapk_target(normal_user),
    )));

    // 添加黑名单用户
    test_results.push(Box::new(GenericShinigamiEyeResult::new(
        blacklisted_user.to_string(),
        "酷安".to_string(),
        blacklisted_lifespan,
        death_note.is_coolapk_target(blacklisted_user),
    )));

    results.insert("测试".to_string(), test_results);

    // 测试琉克的审判和苹果消费
    println!("🍎 测试苹果消费机制...");
    let ryuk = RyukGuidanceSystem::new();

    // 记录初始苹果数量
    let initial_apples = ryuk.get_apple_count();
    println!("   初始苹果数量: {}", initial_apples);

    // 执行审判
    let decision = ryuk.ryuk_judgment(results).await;

    // 检查决策结果
    match &decision {
        DeathNoteDecision::Execute { death_targets, .. } => {
            println!("✅ 琉克决定执行审判");
            println!("   黑名单目标数量: {}", death_targets.len());
            assert_eq!(death_targets.len(), 1, "应该有1个黑名单目标");

            // 验证苹果消费
            let final_apples = ryuk.get_apple_count();
            let consumed_apples = final_apples - initial_apples;
            println!("   消费苹果数量: {}", consumed_apples);
            assert_eq!(
                consumed_apples,
                death_targets.len(),
                "消费的苹果数应等于黑名单目标数"
            );
        }
        DeathNoteDecision::Skip => {
            panic!("测试失败：应该执行审判但被跳过了");
        }
    }

    println!("🎉 所有测试通过！新的寿命和苹果机制工作正常");
}

#[test]
fn test_lifespan_consistency() {
    println!("🔄 测试寿命一致性...");

    let calculator = LifespanCalculator::new();
    let test_uid = "test_consistency_user";

    // 多次计算同一用户的寿命
    let lifespans: Vec<u32> = (0..10)
        .map(|_| calculator.calculate_lifespan(test_uid, false))
        .collect();

    // 验证所有寿命值都相同
    let first_lifespan = lifespans[0];
    for (i, &lifespan) in lifespans.iter().enumerate() {
        assert_eq!(
            lifespan,
            first_lifespan,
            "第{}次计算的寿命({})与第1次计算的寿命({})不一致",
            i + 1,
            lifespan,
            first_lifespan
        );
    }

    println!(
        "   ✅ 用户 {} 的寿命保持一致: {} 年",
        test_uid, first_lifespan
    );
    println!("🎉 寿命一致性测试通过！");
}
