use super::{coolapk::DEATH_NOTE_COOLAPK, qq::DEATH_NOTE_QQ, telegram::DEATH_NOTE_TELEGRAM};

/// 死亡笔记管理器
/// 原型：死亡笔记 - 记录应被审判的灵魂名单
/// 规则：写下名字的人类将会死亡，死神通过此笔记收割灵魂
pub struct DeathNote;

impl DeathNote {
    /// 创建新的死亡笔记实例
    pub fn new() -> Self {
        println!("📖 翻查死亡笔记");
        Self
    }

    /// 检查酷安用户名是否在死亡笔记上
    pub fn is_coolapk_target(&self, username: &str) -> bool {
        DEATH_NOTE_COOLAPK.contains(&username)
    }

    /// 检查Telegram用户名是否在死亡笔记上
    pub fn is_telegram_target(&self, username: &str) -> bool {
        DEATH_NOTE_TELEGRAM.contains(&username)
    }

    /// 检查QQ号是否在死亡笔记上
    pub fn is_qq_target(&self, qq_number: &str) -> bool {
        DEATH_NOTE_QQ.contains(&qq_number)
    }
}

impl Default for DeathNote {
    fn default() -> Self {
        Self::new()
    }
}
