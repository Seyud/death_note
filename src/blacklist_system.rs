/// 黑名单系统
/// 管理黑名单ID，并提供检查功能
pub struct BlacklistSystem {
    coolapk_blacklist: Vec<String>,
    telegram_blacklist: Vec<String>,
    qq_blacklist: Vec<String>,
}

impl BlacklistSystem {
    pub fn new() -> Self {
        // 初始化黑名单列表
        let coolapk_blacklist = vec!["1234567".to_string(), "9999999".to_string()];

        let telegram_blacklist = vec![
            "8179799086".to_string(), // 示例黑名单ID
            "123456789".to_string(),
            "987654321".to_string(),
        ];

        let qq_blacklist = vec![
            "123456789".to_string(), // 示例QQ黑名单ID
            "987654321".to_string(),
            "555555555".to_string(),
        ];

        BlacklistSystem {
            coolapk_blacklist,
            telegram_blacklist,
            qq_blacklist,
        }
    }

    /// 检查酷安UID是否在黑名单中
    pub fn is_coolapk_blacklisted(&self, uid: &str) -> bool {
        self.coolapk_blacklist.contains(&uid.to_string())
    }

    /// 检查Telegram UID是否在黑名单中
    pub fn is_telegram_blacklisted(&self, uid: &str) -> bool {
        self.telegram_blacklist.contains(&uid.to_string())
    }

    /// 检查QQ号是否在黑名单中
    pub fn is_qq_blacklisted(&self, qq_number: &str) -> bool {
        self.qq_blacklist.contains(&qq_number.to_string())
    }
}

impl Default for BlacklistSystem {
    fn default() -> Self {
        Self::new()
    }
}
