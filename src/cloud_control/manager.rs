use crate::cloud_control::{
    cache::CloudControlCache,
    client::CloudControlClient,
    error::CloudControlError,
    types::{CloudControlConfig, CloudControlData, Platform},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// 云控管理器
/// 负责协调客户端和缓存，提供统一的云控数据访问接口
pub struct CloudControlManager {
    config: CloudControlConfig,
    client: CloudControlClient,
    cache: CloudControlCache,
    /// 内存中的云控数据，使用Arc<RwLock>支持多线程访问
    data: Arc<RwLock<Option<CloudControlData>>>,
}

impl CloudControlManager {
    /// 创建新的云控管理器
    pub fn new(config: CloudControlConfig) -> Result<Self, CloudControlError> {
        let client = CloudControlClient::new(config.clone())?;
        let cache = CloudControlCache::new(&config)?;

        Ok(Self {
            config,
            client,
            cache,
            data: Arc::new(RwLock::new(None)),
        })
    }

    /// 从编译时嵌入的配置创建云控管理器
    /// 这允许程序在生产环境中无需 cloud_config.toml 文件
    pub fn new_from_embedded_config() -> Result<Self, CloudControlError> {
        let config_str = crate::cloud_control::get_embedded_config();
        let config: CloudControlConfig =
            toml::from_str(config_str).map_err(CloudControlError::EmbeddedConfigParse)?;
        Self::new(config)
    }

    /// 初始化管理器，优先从缓存加载数据
    pub async fn initialize(&self) -> Result<(), CloudControlError> {
        if !self.config.enabled {
            println!("ℹ️ 云控系统已禁用");
            return Ok(());
        }

        // 尝试从缓存加载
        if let Some(cache_entry) = self.cache.load() {
            let mut data = self.data.write().await;
            *data = Some(cache_entry.data);
            println!("📂 云控数据已从缓存加载");
            return Ok(());
        }

        // 缓存无效，从网络获取
        self.refresh_data().await
    }

    /// 刷新云控数据
    pub async fn refresh_data(&self) -> Result<(), CloudControlError> {
        if !self.config.enabled {
            return Ok(());
        }

        println!("🔄 正在刷新云控数据...");

        match self.client.fetch_data_with_retry().await {
            Ok(new_data) => {
                // 保存到缓存
                if let Err(e) = self.cache.save(new_data.clone(), None) {
                    println!("⚠️ 保存缓存失败: {}", e);
                }

                // 更新内存数据
                let mut data = self.data.write().await;
                *data = Some(new_data);

                println!("✅ 云控数据刷新成功");
                Ok(())
            }
            Err(e) => {
                println!("❌ 云控数据刷新失败: {}", e);
                Err(e)
            }
        }
    }

    /// 检查指定平台的用户是否在云控名单中
    pub async fn is_target(&self, platform: Platform, identifier: &str) -> bool {
        if !self.config.enabled {
            return false;
        }

        let data = self.data.read().await;
        if let Some(cloud_data) = data.as_ref()
            && let Some(platform_list) = cloud_data.platforms.get(&platform)
        {
            return platform_list.contains(&identifier.to_string());
        }

        false
    }

    /// 获取指定平台的所有云控ID
    pub async fn get_platform_ids(&self, platform: Platform) -> Vec<String> {
        if !self.config.enabled {
            return Vec::new();
        }

        let data = self.data.read().await;
        if let Some(cloud_data) = data.as_ref()
            && let Some(platform_list) = cloud_data.platforms.get(&platform)
        {
            return platform_list.clone();
        }

        Vec::new()
    }

    /// 获取所有平台的云控数据统计
    pub async fn get_statistics(&self) -> HashMap<Platform, usize> {
        let mut stats = HashMap::new();

        if !self.config.enabled {
            return stats;
        }

        let data = self.data.read().await;
        if let Some(cloud_data) = data.as_ref() {
            for (platform, ids) in &cloud_data.platforms {
                stats.insert(platform.clone(), ids.len());
            }
        }

        stats
    }

    /// 获取云控数据版本信息
    pub async fn get_version_info(&self) -> Option<(String, String)> {
        if !self.config.enabled {
            return None;
        }

        let data = self.data.read().await;
        data.as_ref()
            .map(|d| (d.version.clone(), d.last_updated.clone()))
    }

    /// 检查是否需要更新
    pub async fn should_update(&self) -> bool {
        if !self.config.enabled {
            return false;
        }

        // 检查是否有内存数据
        let data = self.data.read().await;
        if data.is_none() {
            return true;
        }

        // 检查缓存是否过期
        !self.cache.is_valid()
    }

    /// 清除所有缓存和内存数据
    pub async fn clear_all_data(&self) -> Result<(), CloudControlError> {
        // 清除内存数据
        let mut data = self.data.write().await;
        *data = None;

        // 清除缓存
        self.cache.clear()?;

        println!("🗑️ 所有云控数据已清除");
        Ok(())
    }

    /// 获取配置信息
    pub fn get_config(&self) -> &CloudControlConfig {
        &self.config
    }

    /// 打印云控状态信息
    pub async fn print_status(&self) {
        println!("📊 云控系统状态:");
        println!("  启用状态: {}", self.config.enabled);

        if !self.config.enabled {
            return;
        }

        println!("  仓库地址: {}", self.config.repository.url);
        println!("  数据分支: {}", self.config.repository.branch);
        println!("  数据文件: {}", self.config.repository.data_file);

        if let Some((version, last_updated)) = self.get_version_info().await {
            println!("  数据版本: {}", version);
            println!("  更新时间: {}", last_updated);
        }

        let stats = self.get_statistics().await;
        if !stats.is_empty() {
            println!("  平台统计:");
            for (platform, count) in stats {
                println!("    {}: {} 个ID", platform.as_str(), count);
            }
        }

        println!(
            "  缓存状态: {}",
            if self.cache.is_valid() {
                "有效"
            } else {
                "无效"
            }
        );
    }
}
