//! 编译时云控配置模块
//!
//! 该模块包含在编译时从 cloud_config.toml 嵌入的配置数据
//! 使程序在生产环境中无需依赖运行时配置文件

// 包含编译时生成的云控配置数据
include!(concat!(env!("OUT_DIR"), "/cloud_config_data.rs"));
/// 获取编译时嵌入的云控配置
///
/// 这个函数返回在编译时从 cloud_config.toml 读取并嵌入的配置
/// 如果编译时没有 cloud_config.toml 文件或配置无效，将返回禁用的配置
pub fn get_embedded_config() -> crate::cloud_control::CloudControlConfig {
    embedded_cloud_config::get_config()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_config_accessibility() {
        let config = get_embedded_config();
        // 基本配置结构测试
        assert!(!config.repository.url.is_empty() || !config.enabled);
        assert!(!config.repository.branch.is_empty());
        assert!(!config.repository.data_file.is_empty());
        assert!(!config.cache.cache_dir.is_empty());
        assert!(!config.cache.cache_file.is_empty());
        assert!(config.cache.ttl_seconds > 0);
        assert!(config.update.check_interval_seconds > 0);
        assert!(config.update.timeout_seconds > 0);
        assert!(config.update.retry_count > 0);
    }
}
