//! 死亡笔记Telegram页
//! 黑名单数据从构建时配置文件中生成

// 引入构建时生成的黑名单数据
include!(concat!(env!("OUT_DIR"), "/blacklist_data.rs"));
