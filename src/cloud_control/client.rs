use crate::cloud_control::types::{CloudControlConfig, CloudControlData, DataSource, Platform};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use tokio::time::{Duration, timeout};

/// 云控客户端错误类型
#[derive(Debug)]
pub enum CloudControlError {
    /// 网络错误
    NetworkError(String),
    /// 解析错误
    ParseError(String),
    /// 配置错误
    ConfigError(String),
    /// 超时错误
    TimeoutError,
    /// 其他错误
    Other(String),
}

impl fmt::Display for CloudControlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CloudControlError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            CloudControlError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            CloudControlError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            CloudControlError::TimeoutError => write!(f, "请求超时"),
            CloudControlError::Other(msg) => write!(f, "其他错误: {}", msg),
        }
    }
}

impl Error for CloudControlError {}

/// 云控客户端
pub struct CloudControlClient {
    config: CloudControlConfig,
    http_client: reqwest::Client,
}

impl CloudControlClient {
    /// 创建新的云控客户端
    pub fn new(config: CloudControlConfig) -> Result<Self, CloudControlError> {
        let mut builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(config.update.timeout_seconds))
            .user_agent("DeathNote-CloudControl/1.0");

        // 如果有访问令牌，添加到默认头部
        if let Some(token) = &config.repository.access_token {
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("token {}", token))
                    .map_err(|e| CloudControlError::ConfigError(format!("Invalid token: {}", e)))?,
            );
            builder = builder.default_headers(headers);
        }

        let http_client = builder
            .build()
            .map_err(|e| CloudControlError::NetworkError(format!("创建HTTP客户端失败: {}", e)))?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// 获取云控数据
    pub async fn fetch_data(&self) -> Result<CloudControlData, CloudControlError> {
        let url = self.build_raw_file_url()?;

        println!("🌐 正在从云端获取数据: {}", url);

        // 使用超时包装请求
        let response = timeout(
            Duration::from_secs(self.config.update.timeout_seconds),
            self.http_client.get(&url).send(),
        )
        .await
        .map_err(|_| CloudControlError::TimeoutError)?
        .map_err(|e| CloudControlError::NetworkError(format!("请求失败: {}", e)))?;

        if !response.status().is_success() {
            return Err(CloudControlError::NetworkError(format!(
                "HTTP错误: {}",
                response.status()
            )));
        }

        let content = response
            .text()
            .await
            .map_err(|e| CloudControlError::NetworkError(format!("读取响应失败: {}", e)))?;

        self.parse_data(&content)
    }

    /// 带重试机制获取数据
    pub async fn fetch_data_with_retry(&self) -> Result<CloudControlData, CloudControlError> {
        let mut last_error = CloudControlError::Other("未知错误".to_string());

        for attempt in 1..=self.config.update.retry_count {
            match self.fetch_data().await {
                Ok(data) => {
                    if attempt > 1 {
                        println!("✅ 第{}次重试成功获取云控数据", attempt);
                    }
                    return Ok(data);
                }
                Err(e) => {
                    last_error = e;
                    if attempt < self.config.update.retry_count {
                        println!("⚠️ 第{}次获取失败，{}秒后重试...", attempt, attempt * 2);
                        tokio::time::sleep(Duration::from_secs(attempt as u64 * 2)).await;
                    }
                }
            }
        }

        Err(last_error)
    }

    /// 检查数据是否有更新（简单版本，实际可以使用ETag或Last-Modified）
    pub async fn check_for_updates(
        &self,
        current_version: Option<&str>,
    ) -> Result<bool, CloudControlError> {
        // 这里简化实现，实际可以实现更智能的检查
        // 比如使用HEAD请求检查ETag或Last-Modified
        match self.fetch_data().await {
            Ok(data) => {
                if let Some(version) = current_version {
                    Ok(data.version != version)
                } else {
                    Ok(true) // 没有当前版本信息，认为需要更新
                }
            }
            Err(e) => Err(e),
        }
    }

    /// 构建原始文件URL
    fn build_raw_file_url(&self) -> Result<String, CloudControlError> {
        let repo_url = &self.config.repository.url;
        let branch = &self.config.repository.branch;
        let file_path = &self.config.repository.data_file;

        // 支持Gitee原始文件URL格式
        if repo_url.contains("gitee.com") {
            // 从仓库URL提取用户名和仓库名
            let parts: Vec<&str> = repo_url.trim_end_matches(".git").split('/').collect();

            if parts.len() < 2 {
                return Err(CloudControlError::ConfigError(
                    "无效的Gitee仓库URL格式".to_string(),
                ));
            }

            let owner = parts[parts.len() - 2];
            let repo = parts[parts.len() - 1];

            Ok(format!(
                "https://gitee.com/{}/{}/raw/{}/{}",
                owner, repo, branch, file_path
            ))
        } else {
            Err(CloudControlError::ConfigError(
                "暂不支持的代码托管平台".to_string(),
            ))
        }
    }

    /// 解析TOML数据
    fn parse_data(&self, content: &str) -> Result<CloudControlData, CloudControlError> {
        let toml_value: toml::Value = toml::from_str(content)
            .map_err(|e| CloudControlError::ParseError(format!("TOML解析失败: {}", e)))?;

        let mut platforms = HashMap::new();

        // 解析各平台数据
        for platform in [Platform::Coolapk, Platform::QQ, Platform::Telegram] {
            if let Some(platform_data) = toml_value.get(platform.as_str())
                && let Some(users) = platform_data.get("users").and_then(|v| v.as_array())
            {
                let user_list: Vec<String> = users
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect();

                if !user_list.is_empty() {
                    platforms.insert(platform, user_list);
                }
            }
        }

        // 解析元信息
        let version = toml_value
            .get("meta")
            .and_then(|m| m.get("version"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let last_updated = toml_value
            .get("meta")
            .and_then(|m| m.get("last_updated"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();

        let description = toml_value
            .get("meta")
            .and_then(|m| m.get("description"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let source = Some(DataSource {
            repository: self.config.repository.url.clone(),
            branch: self.config.repository.branch.clone(),
            commit: None, // 暂不获取提交哈希
        });

        Ok(CloudControlData {
            version,
            last_updated,
            description,
            platforms,
            source,
        })
    }
}
