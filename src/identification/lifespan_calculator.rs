//! 寿命计算器组件
//! 实现死神之眼的寿命感知能力 - 为不同用户分配一致的寿命值

use rand::prelude::*;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// 寿命计算器 - 死神之眼的核心能力
/// 原型：死神之眼能够看到人类剩余寿命的特殊视觉
pub struct LifespanCalculator;

impl LifespanCalculator {
    /// 创建新的寿命计算器
    pub fn new() -> Self {
        Self
    }

    /// 计算用户的剩余寿命
    ///
    /// # 规则
    /// - 非黑名单用户：50-80年随机寿命
    /// - 黑名单用户：寿命为0（即将死亡）
    /// - 同一用户保持一致性：基于UID哈希生成种子
    ///
    /// # 参数
    /// - `uid`: 用户唯一标识
    /// - `is_blacklisted`: 是否在黑名单中
    ///
    /// # 返回值
    /// 用户的剩余寿命（年）
    pub fn calculate_lifespan(&self, uid: &str, is_blacklisted: bool) -> u32 {
        if is_blacklisted {
            // 黑名单用户寿命为0，等待死神审判
            0
        } else {
            // 为非黑名单用户生成50-80年的随机寿命
            let seed = self.generate_seed_from_uid(uid);
            let mut rng = StdRng::seed_from_u64(seed);
            rng.gen_range(50..=80)
        }
    }

    /// 基于UID生成一致性种子
    ///
    /// 使用哈希函数确保同一UID总是产生相同的种子，
    /// 从而保证同一用户的寿命在多次计算中保持一致
    fn generate_seed_from_uid(&self, uid: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        uid.hash(&mut hasher);
        hasher.finish()
    }
}

impl Default for LifespanCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blacklisted_user_zero_lifespan() {
        let calculator = LifespanCalculator::new();
        let lifespan = calculator.calculate_lifespan("123456", true);
        assert_eq!(lifespan, 0);
    }

    #[test]
    fn test_normal_user_lifespan_range() {
        let calculator = LifespanCalculator::new();
        let lifespan = calculator.calculate_lifespan("test_user", false);
        assert!(lifespan >= 50 && lifespan <= 80);
    }

    #[test]
    fn test_lifespan_consistency() {
        let calculator = LifespanCalculator::new();
        let uid = "consistent_user";

        // 多次计算应返回相同结果
        let lifespan1 = calculator.calculate_lifespan(uid, false);
        let lifespan2 = calculator.calculate_lifespan(uid, false);
        let lifespan3 = calculator.calculate_lifespan(uid, false);

        assert_eq!(lifespan1, lifespan2);
        assert_eq!(lifespan2, lifespan3);
    }

    #[test]
    fn test_different_users_different_lifespans() {
        let calculator = LifespanCalculator::new();

        // 不同用户应该有不同的寿命（大概率）
        let lifespans: Vec<u32> = (0..10)
            .map(|i| calculator.calculate_lifespan(&format!("user_{}", i), false))
            .collect();

        // 检查是否有足够的差异性（至少50%不同）
        let unique_count = lifespans
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert!(unique_count >= 5, "寿命应该有足够的随机性");
    }
}
