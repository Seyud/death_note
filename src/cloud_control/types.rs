use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// 平台枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Coolapk,
    QQ,
    Telegram,
    WeChat,
}

impl Platform {
    /// 获取平台名称字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Platform::Coolapk => "coolapk",
            Platform::QQ => "qq",
            Platform::Telegram => "telegram",
            Platform::WeChat => "wechat",
        }
    }
}

/// 云控数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudControlData {
    /// 数据版本
    pub version: String,
    /// 最后更新时间
    pub last_updated: String,
    /// 描述信息
    pub description: Option<String>,
    /// 各平台的黑名单数据
    pub platforms: HashMap<Platform, Vec<String>>,
    /// 数据源信息
    pub source: Option<DataSource>,
}

/// 数据源信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// 仓库URL
    pub repository: String,
    /// 分支名
    pub branch: String,
    /// 提交哈希
    pub commit: Option<String>,
}

/// 云控配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudControlConfig {
    /// 是否启用云控
    pub enabled: bool,
    /// 仓库配置
    pub repository: RepositoryConfig,
    /// 缓存配置
    pub cache: CacheConfig,
    /// 更新配置
    pub update: UpdateConfig,
}

/// 仓库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    /// 仓库URL
    pub url: String,
    /// 分支名
    pub branch: String,
    /// 数据文件路径
    pub data_file: String,
    /// 访问令牌（如果需要）
    pub access_token: Option<String>,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存目录
    pub cache_dir: String,
    /// 缓存文件名
    pub cache_file: String,
    /// 缓存有效期（秒）
    pub ttl_seconds: u64,
}

/// 更新配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateConfig {
    /// 检查更新间隔（秒）
    pub check_interval_seconds: u64,
    /// 连接超时（秒）
    pub timeout_seconds: u64,
    /// 重试次数
    pub retry_count: u32,
}

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// 云控数据
    pub data: CloudControlData,
    /// 缓存时间
    pub cached_at: SystemTime,
    /// ETag或最后修改时间，用于检测变化
    pub etag: Option<String>,
}

impl CacheEntry {
    /// 检查缓存是否过期
    pub fn is_expired(&self, ttl_seconds: u64) -> bool {
        match self.cached_at.elapsed() {
            Ok(elapsed) => elapsed.as_secs() > ttl_seconds,
            Err(_) => true, // 时间异常，认为已过期
        }
    }
}

impl Default for CloudControlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            repository: RepositoryConfig {
                url: "https://gitee.com/Seyud/death-note-cloud-control-list.git".to_string(),
                branch: "master".to_string(),
                data_file: "blacklist.toml".to_string(),
                access_token: None,
            },
            cache: CacheConfig {
                cache_dir: ".cache/cloud_control".to_string(),
                cache_file: "cloud_data.json".to_string(),
                ttl_seconds: 3600, // 1小时
            },
            update: UpdateConfig {
                check_interval_seconds: 300, // 5分钟
                timeout_seconds: 30,
                retry_count: 3,
            },
        }
    }
}
