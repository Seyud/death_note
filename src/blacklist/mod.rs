//! 黑名单系统模块
//! 
//! 提供模块化的黑名单管理功能，支持各种应用平台的黑名单检查

// 平台特定的黑名单配置
pub mod coolapk;
pub mod telegram;
pub mod qq;

// 黑名单管理器
pub mod manager;

// 重导出主要类型，保持API简洁
pub use manager::BlacklistSystem;
