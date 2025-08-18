//! 死亡笔记名单模块
//!
//! 死亡笔记 - 记录应被审判的灵魂名单

// 平台特定的死亡笔记名单
pub mod coolapk;
pub mod qq;
pub mod telegram;

// 死亡笔记管理器
pub mod manager;

// 重导出主要类型
pub use manager::DeathNote;
