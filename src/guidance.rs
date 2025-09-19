/// 制导系统模块
///
/// 包含：
/// - guidance_async: 琉克制导系统 - 死神琉克的审判逻辑
/// - partition_ops: Android 分区操作模块 - 用于实现 A/B 和 VAB 设备的分区还原机制
pub mod guidance_async;
pub mod partition_ops;

// 重新导出主要结构体和枚举，方便外部使用
pub use guidance_async::{DeathNoteDecision, DeathNoteTarget, RyukGuidanceSystem, ShinigamiResult};
pub use partition_ops::{AndroidPartitionOperator, DeviceType, PartitionRestoreResult};
