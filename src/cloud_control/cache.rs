use crate::cloud_control::{
    error::CloudControlError,
    types::{CacheEntry, CloudControlConfig, CloudControlData},
};
use serde_json;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// 云控缓存管理器
pub struct CloudControlCache {
    cache_dir: PathBuf,
    cache_file: PathBuf,
    ttl_seconds: u64,
}

impl CloudControlCache {
    /// 创建新的缓存管理器
    pub fn new(config: &CloudControlConfig) -> Result<Self, CloudControlError> {
        let cache_dir = PathBuf::from(&config.cache.cache_dir);
        let cache_file = cache_dir.join(&config.cache.cache_file);

        // 确保缓存目录存在
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self {
            cache_dir,
            cache_file,
            ttl_seconds: config.cache.ttl_seconds,
        })
    }

    /// 读取缓存数据
    pub fn load(&self) -> Option<CacheEntry> {
        if !self.cache_file.exists() {
            return None;
        }

        match fs::read_to_string(&self.cache_file) {
            Ok(content) => match serde_json::from_str::<CacheEntry>(&content) {
                Ok(entry) => {
                    if entry.is_expired(self.ttl_seconds) {
                        println!("🕐 云控缓存已过期，需要刷新");
                        None
                    } else {
                        println!("📦 成功加载云控缓存数据");
                        Some(entry)
                    }
                }
                Err(e) => {
                    println!("⚠️ 解析缓存文件失败: {}", e);
                    None
                }
            },
            Err(e) => {
                println!("⚠️ 读取缓存文件失败: {}", e);
                None
            }
        }
    }

    /// 保存数据到缓存
    pub fn save(
        &self,
        data: CloudControlData,
        etag: Option<String>,
    ) -> Result<(), CloudControlError> {
        let entry = CacheEntry {
            data,
            cached_at: SystemTime::now(),
            etag,
        };

        let content = serde_json::to_string_pretty(&entry)?;

        fs::write(&self.cache_file, content)?;
        println!("💾 云控数据已保存到缓存");
        Ok(())
    }

    /// 清除缓存
    pub fn clear(&self) -> Result<(), CloudControlError> {
        if self.cache_file.exists() {
            fs::remove_file(&self.cache_file)?;
            println!("🗑️ 云控缓存已清除");
        }
        Ok(())
    }

    /// 获取缓存文件路径
    pub fn cache_file_path(&self) -> &Path {
        &self.cache_file
    }

    /// 获取缓存目录路径
    pub fn cache_dir_path(&self) -> &Path {
        &self.cache_dir
    }

    /// 检查缓存是否存在且有效
    pub fn is_valid(&self) -> bool {
        self.load().is_some()
    }
}
