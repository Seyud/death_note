//! 死神之眼识别模块
//!
//! 琉克的死神之眼 - 能够看透人类真名和剩余寿命的神秘能力
//!
//! 支持异步并行激活，可同时观察多个平台的人类世界

// 死神之眼模块
pub mod coolapk_identifier;
pub mod manager;
pub mod qq_identifier;
pub mod telegram_identifier;
pub mod traits;

// 导出死神之眼功能
pub use coolapk_identifier::CoolapkShinigamiEye;
pub use manager::ShinigamiEyeManager;
pub use qq_identifier::QQShinigamiEye;
pub use telegram_identifier::TelegramShinigamiEye;
// 仅导出必要的结果trait
pub use traits::ShinigamiEyeResult;
