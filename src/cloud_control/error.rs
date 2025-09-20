use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudControlError {
    #[error("嵌入式配置解析失败")]
    EmbeddedConfigParse(#[from] toml::de::Error),

    #[error("配置文件读取失败")]
    ConfigFileRead(#[from] std::io::Error),

    #[error("网络请求失败")]
    NetworkRequest(#[from] reqwest::Error),

    #[error("JSON解析失败")]
    JsonParse(#[from] serde_json::Error),

    #[error("缓存文件处理失败: {0}")]
    CacheFile(String),

    #[error("云端数据无效: {0}")]
    InvalidCloudData(String),
}
