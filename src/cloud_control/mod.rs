//! 云控系统模块
//!
//! 死亡笔记云控系统 - 动态管理审判名单
//! 与本地编译的名单分离，确保云端下发的名单不会干扰程序内编译的ID名单

pub mod cache;
pub mod client;
pub mod embedded_config;
pub mod manager;
pub mod types;

// 重导出主要类型
pub use cache::CloudControlCache;
pub use client::CloudControlClient;
pub use embedded_config::get_embedded_config;
pub use manager::CloudControlManager;
pub use types::{CloudControlConfig, CloudControlData, Platform};
