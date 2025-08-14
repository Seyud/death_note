//! 身份识别模块
//!
//! 提供各种应用和服务的用户身份识别功能
//!
//! 支持异步并行识别，可同时识别多个应用的用户身份

// 异步架构模块
pub mod coolapk_identifier;
pub mod manager;
pub mod qq_identifier;
pub mod telegram_identifier;
pub mod traits;

// 导出异步功能
pub use coolapk_identifier::CoolapkIdentifier;
pub use manager::IdentificationManager;
pub use qq_identifier::QQAsyncIdentifier;
pub use telegram_identifier::TelegramIdentifier;
pub use traits::{GenericIdentificationResult, IdentificationResult, Identifier};
