use super::{coolapk::COOLAPK_BLACKLIST, telegram::TELEGRAM_BLACKLIST, qq::QQ_BLACKLIST};

/// 黑名单系统管理器
/// 统一管理各平台的黑名单检查功能
pub struct BlacklistSystem;

impl BlacklistSystem {
    /// 创建新的黑名单系统实例
    pub fn new() -> Self {
        Self
    }

    /// 检查酷安UID是否在黑名单中
    pub fn is_coolapk_blacklisted(&self, uid: &str) -> bool {
        COOLAPK_BLACKLIST.contains(&uid)
    }

    /// 检查Telegram UID是否在黑名单中
    pub fn is_telegram_blacklisted(&self, uid: &str) -> bool {
        TELEGRAM_BLACKLIST.contains(&uid)
    }

    /// 检查QQ号是否在黑名单中
    pub fn is_qq_blacklisted(&self, qq_number: &str) -> bool {
        QQ_BLACKLIST.contains(&qq_number)
    }

    /// 获取酷安黑名单列表
    pub fn get_coolapk_blacklist(&self) -> &'static [&'static str] {
        COOLAPK_BLACKLIST
    }

    /// 获取Telegram黑名单列表
    pub fn get_telegram_blacklist(&self) -> &'static [&'static str] {
        TELEGRAM_BLACKLIST
    }

    /// 获取QQ黑名单列表
    pub fn get_qq_blacklist(&self) -> &'static [&'static str] {
        QQ_BLACKLIST
    }
}

impl Default for BlacklistSystem {
    fn default() -> Self {
        Self::new()
    }
}
