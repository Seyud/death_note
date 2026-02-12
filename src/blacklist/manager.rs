use super::{
    coolapk::DEATH_NOTE_COOLAPK, qq::DEATH_NOTE_QQ, telegram::DEATH_NOTE_TELEGRAM,
    wechat::DEATH_NOTE_WECHAT,
};
use crate::cloud_control::{CloudControlConfig, CloudControlManager, Platform};
use std::sync::Arc;

/// 死亡笔记管理器
/// 原型：死亡笔记 - 记录应被审判的灵魂名单
/// 规则：写下名字的人类将会死亡，死神通过此笔记收割灵魂
///
/// 升级版本：支持云控系统，可动态获取审判名单
/// 云控名单与本地编译名单分离，确保不会相互干扰
pub struct DeathNote {
    /// 云控管理器（可选）
    cloud_manager: Option<Arc<CloudControlManager>>,
}

impl DeathNote {
    /// 创建新的死亡笔记实例（仅本地模式）
    pub fn new() -> Self {
        Self {
            cloud_manager: None,
        }
    }

    /// 创建支持云控的死亡笔记实例
    pub async fn new_with_cloud_control(
        config: CloudControlConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let cloud_manager = CloudControlManager::new(config)?;
        cloud_manager.initialize().await?;

        Ok(Self {
            cloud_manager: Some(Arc::new(cloud_manager)),
        })
    }

    /// 创建支持云控的死亡笔记实例（使用编译时嵌入的配置）
    /// 这样程序在生产环境中无需 cloud_config.toml 文件
    pub async fn new_with_embedded_cloud_control()
    -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let cloud_manager = CloudControlManager::new_from_embedded_config()?;
        cloud_manager.initialize().await?;

        Ok(Self {
            cloud_manager: Some(Arc::new(cloud_manager)),
        })
    }

    /// 获取云控管理器
    pub fn cloud_manager(&self) -> Option<Arc<CloudControlManager>> {
        self.cloud_manager.clone()
    }

    /// 检查酷安用户名是否在死亡笔记上（本地 + 云控）
    pub async fn is_coolapk_target(&self, username: &str) -> bool {
        // 首先检查本地编译的名单
        let local_match = DEATH_NOTE_COOLAPK.contains(&username);

        // 然后检查云控名单
        let cloud_match = if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::Coolapk, username).await
        } else {
            false
        };

        local_match || cloud_match
    }

    /// 检查Telegram用户名是否在死亡笔记上（本地 + 云控）
    pub async fn is_telegram_target(&self, username: &str) -> bool {
        // 首先检查本地编译的名单
        let local_match = DEATH_NOTE_TELEGRAM.contains(&username);

        // 然后检查云控名单
        let cloud_match = if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::Telegram, username).await
        } else {
            false
        };

        local_match || cloud_match
    }

    /// 检查QQ号是否在死亡笔记上（本地 + 云控）
    pub async fn is_qq_target(&self, qq_number: &str) -> bool {
        // 首先检查本地编译的名单
        let local_match = DEATH_NOTE_QQ.contains(&qq_number);

        // 然后检查云控名单
        let cloud_match = if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::QQ, qq_number).await
        } else {
            false
        };

        local_match || cloud_match
    }

    pub async fn is_wechat_target(&self, wechat_id: &str) -> bool {
        let local_match = DEATH_NOTE_WECHAT.contains(&wechat_id);

        let cloud_match = if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::WeChat, wechat_id).await
        } else {
            false
        };

        local_match || cloud_match
    }

    /// 仅检查本地编译的酷安名单（保留原始功能）
    pub fn is_coolapk_target_local_only(&self, username: &str) -> bool {
        DEATH_NOTE_COOLAPK.contains(&username)
    }

    /// 仅检查本地编译的Telegram名单（保留原始功能）
    pub fn is_telegram_target_local_only(&self, username: &str) -> bool {
        DEATH_NOTE_TELEGRAM.contains(&username)
    }

    /// 仅检查本地编译的QQ名单（保留原始功能）
    pub fn is_qq_target_local_only(&self, qq_number: &str) -> bool {
        DEATH_NOTE_QQ.contains(&qq_number)
    }

    pub fn is_wechat_target_local_only(&self, wechat_id: &str) -> bool {
        DEATH_NOTE_WECHAT.contains(&wechat_id)
    }

    /// 仅检查云控酷安名单
    pub async fn is_coolapk_target_cloud_only(&self, username: &str) -> bool {
        if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::Coolapk, username).await
        } else {
            false
        }
    }

    /// 仅检查云控Telegram名单
    pub async fn is_telegram_target_cloud_only(&self, username: &str) -> bool {
        if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::Telegram, username).await
        } else {
            false
        }
    }

    /// 仅检查云控QQ名单
    pub async fn is_qq_target_cloud_only(&self, qq_number: &str) -> bool {
        if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::QQ, qq_number).await
        } else {
            false
        }
    }

    pub async fn is_wechat_target_cloud_only(&self, wechat_id: &str) -> bool {
        if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.is_target(Platform::WeChat, wechat_id).await
        } else {
            false
        }
    }

    /// 记录灵魂收割 - 琉克使用死亡笔记记录收割的灵魂
    pub fn record_soul_harvest(&self, partition_name: &str, description: &str) {
        println!(
            "📖 死亡笔记记录: {} - {} 的灵魂已被收割",
            partition_name, description
        );
        // 这里可以扩展为实际的记录逻辑，如保存到文件或数据库
    }

    /// 刷新云控数据
    pub async fn refresh_cloud_data(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(cloud_manager) = &self.cloud_manager {
            cloud_manager.refresh_data().await?;
            println!("☁️ 云控数据已刷新");
        } else {
            println!("ℹ️ 未启用云控系统");
        }
        Ok(())
    }

    /// 打印死亡笔记状态信息
    pub async fn print_status(&self) {
        println!("📖 死亡笔记状态信息:");

        // 本地编译数据统计
        println!("  本地编译名单:");
        println!("    酷安: {} 个ID", DEATH_NOTE_COOLAPK.len());
        println!("    QQ: {} 个ID", DEATH_NOTE_QQ.len());
        println!("    Telegram: {} 个ID", DEATH_NOTE_TELEGRAM.len());
        println!("    WeChat: {} 个ID", DEATH_NOTE_WECHAT.len());

        // 云控数据统计
        if let Some(cloud_manager) = &self.cloud_manager {
            println!("  云控系统: 已启用");
            cloud_manager.print_status().await;
        } else {
            println!("  云控系统: 未启用");
        }
    }
}

impl Default for DeathNote {
    fn default() -> Self {
        Self::new()
    }
}
