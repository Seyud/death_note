use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取输出目录
    let out_dir = env::var("OUT_DIR").unwrap();
    let blacklist_dest_path = Path::new(&out_dir).join("blacklist_data.rs");
    let cloud_config_dest_path = Path::new(&out_dir).join("cloud_config_data.rs");

    // 读取黑名单配置文件
    let config_content = fs::read_to_string("blacklist_config.toml")
        .expect("无法读取 blacklist_config.toml 配置文件");

    // 解析 TOML 配置
    let config: toml::Value =
        toml::from_str(&config_content).expect("无法解析 blacklist_config.toml 配置文件");

    // 生成黑名单 Rust 代码
    let generated_blacklist_code = generate_blacklist_code(&config);

    // 写入生成的黑名单代码到输出目录
    fs::write(&blacklist_dest_path, generated_blacklist_code).expect("无法写入生成的黑名单代码");

    // 处理云控配置
    generate_cloud_config(&cloud_config_dest_path);

    // 告诉 Cargo 当配置文件改变时重新运行构建脚本
    println!("cargo:rerun-if-changed=blacklist_config.toml");
    println!("cargo:rerun-if-changed=cloud_config.toml");
}

fn generate_blacklist_code(config: &toml::Value) -> String {
    let mut code = String::new();

    // 添加文件头注释
    code.push_str("// 自动生成的黑名单数据\n");
    code.push_str("// 该文件由 build.rs 根据 blacklist_config.toml 自动生成\n");
    code.push_str("// 请勿手动修改此文件，所有更改应在配置文件中进行\n\n");

    // 生成酷安黑名单
    if let Some(coolapk) = config.get("coolapk")
        && let Some(users) = coolapk.get("users").and_then(|v| v.as_array())
    {
        code.push_str("/// 酷安平台黑名单数据\n");
        code.push_str("pub const DEATH_NOTE_COOLAPK: &[&str] = &[\n");
        for user in users {
            if let Some(user_str) = user.as_str() {
                code.push_str(&format!("    \"{}\",\n", user_str));
            }
        }
        code.push_str("];\n\n");
    }

    // 生成 QQ 黑名单
    if let Some(qq) = config.get("qq")
        && let Some(users) = qq.get("users").and_then(|v| v.as_array())
    {
        code.push_str("/// QQ平台黑名单数据\n");
        code.push_str("pub const DEATH_NOTE_QQ: &[&str] = &[\n");
        for user in users {
            if let Some(user_str) = user.as_str() {
                code.push_str(&format!("    \"{}\",\n", user_str));
            }
        }
        code.push_str("];\n\n");
    }

    // 生成 Telegram 黑名单
    if let Some(telegram) = config.get("telegram")
        && let Some(users) = telegram.get("users").and_then(|v| v.as_array())
    {
        code.push_str("/// Telegram平台黑名单数据\n");
        code.push_str("pub const DEATH_NOTE_TELEGRAM: &[&str] = &[\n");
        for user in users {
            if let Some(user_str) = user.as_str() {
                code.push_str(&format!("    \"{}\",\n", user_str));
            }
        }
        code.push_str("];\n\n");
    }

    // 添加配置元信息
    if let Some(meta) = config.get("meta") {
        code.push_str("/// 配置元信息\n");
        code.push_str("pub mod meta {\n");

        if let Some(version) = meta.get("version").and_then(|v| v.as_str()) {
            code.push_str(&format!("    pub const VERSION: &str = \"{}\";\n", version));
        }

        if let Some(description) = meta.get("description").and_then(|v| v.as_str()) {
            code.push_str(&format!(
                "    pub const DESCRIPTION: &str = \"{}\";\n",
                description
            ));
        }

        if let Some(last_updated) = meta.get("last_updated").and_then(|v| v.as_str()) {
            code.push_str(&format!(
                "    pub const LAST_UPDATED: &str = \"{}\";\n",
                last_updated
            ));
        }

        code.push_str("}\n");
    }

    code
}

/// 生成云控配置代码
fn generate_cloud_config(dest_path: &Path) {
    let mut code = String::new();

    // 添加文件头注释
    code.push_str("// 自动生成的云控配置数据\n");
    code.push_str("// 该文件由 build.rs 根据 cloud_config.toml 自动生成\n");
    code.push_str("// 请勿手动修改此文件，所有更改应在配置文件中进行\n\n");

    // 尝试读取云控配置文件
    match fs::read_to_string("cloud_config.toml") {
        Ok(cloud_config_content) => {
            match toml::from_str::<toml::Value>(&cloud_config_content) {
                Ok(cloud_config) => {
                    // 生成云控配置常量
                    code.push_str("/// 编译时嵌入的云控配置\n");
                    code.push_str("pub mod embedded_cloud_config {\n");
                    code.push_str("    use super::*;\n\n");

                    // 生成配置数据
                    generate_cloud_config_constants(&mut code, &cloud_config);

                    // 生成获取配置的函数
                    code.push_str("    /// 获取编译时嵌入的云控配置\n");
                    code.push_str("    pub fn get_config() -> CloudControlConfig {\n");
                    code.push_str("        CloudControlConfig {\n");
                    code.push_str("            enabled: ENABLED,\n");
                    code.push_str("            repository: RepositoryConfig {\n");
                    code.push_str("                url: URL.to_string(),\n");
                    code.push_str("                branch: BRANCH.to_string(),\n");
                    code.push_str("                data_file: DATA_FILE.to_string(),\n");
                    code.push_str(
                        "                access_token: ACCESS_TOKEN.map(|s| s.to_string()),\n",
                    );
                    code.push_str("            },\n");
                    code.push_str("            cache: CacheConfig {\n");
                    code.push_str("                cache_dir: CACHE_DIR.to_string(),\n");
                    code.push_str("                cache_file: CACHE_FILE.to_string(),\n");
                    code.push_str("                ttl_seconds: TTL_SECONDS,\n");
                    code.push_str("            },\n");
                    code.push_str("            update: UpdateConfig {\n");
                    code.push_str(
                        "                check_interval_seconds: CHECK_INTERVAL_SECONDS,\n",
                    );
                    code.push_str("                timeout_seconds: TIMEOUT_SECONDS,\n");
                    code.push_str("                retry_count: RETRY_COUNT,\n");
                    code.push_str("            },\n");
                    code.push_str("        }\n");
                    code.push_str("    }\n");
                    code.push_str("}\n\n");

                    // 导入必要的类型
                    code = format!(
                        "use crate::cloud_control::types::{{CloudControlConfig, RepositoryConfig, CacheConfig, UpdateConfig}};\n\n{}",
                        code
                    );
                }
                Err(e) => {
                    // 配置文件解析失败，生成默认配置或禁用云控
                    eprintln!("警告: 云控配置解析失败: {}", e);
                    generate_disabled_cloud_config(&mut code);
                }
            }
        }
        Err(_) => {
            // 云控配置文件不存在，生成禁用的配置
            println!("警告: cloud_config.toml 不存在，将禁用云控功能");
            generate_disabled_cloud_config(&mut code);
        }
    }

    // 写入生成的代码
    fs::write(dest_path, code).expect("无法写入生成的云控配置代码");
}

/// 生成云控配置常量
fn generate_cloud_config_constants(code: &mut String, config: &toml::Value) {
    // 启用状态
    let enabled = config
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    code.push_str(&format!("    pub const ENABLED: bool = {};\n\n", enabled));

    // 仓库配置
    if let Some(repo) = config.get("repository") {
        let url = repo.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let branch = repo
            .get("branch")
            .and_then(|v| v.as_str())
            .unwrap_or("main");
        let data_file = repo
            .get("data_file")
            .and_then(|v| v.as_str())
            .unwrap_or("blacklist.toml");
        let access_token = repo.get("access_token").and_then(|v| v.as_str());

        code.push_str(&format!("    pub const URL: &str = \"{}\";\n", url));
        code.push_str(&format!("    pub const BRANCH: &str = \"{}\";\n", branch));
        code.push_str(&format!(
            "    pub const DATA_FILE: &str = \"{}\";\n",
            data_file
        ));

        if let Some(token) = access_token {
            code.push_str(&format!(
                "    pub const ACCESS_TOKEN: Option<&str> = Some(\"{}\");\n",
                token
            ));
        } else {
            code.push_str("    pub const ACCESS_TOKEN: Option<&str> = None;\n");
        }
        code.push('\n');
    }

    // 缓存配置
    if let Some(cache) = config.get("cache") {
        let cache_dir = cache
            .get("cache_dir")
            .and_then(|v| v.as_str())
            .unwrap_or(".cache/cloud_control");
        let cache_file = cache
            .get("cache_file")
            .and_then(|v| v.as_str())
            .unwrap_or("cloud_data.json");
        let ttl_seconds = cache
            .get("ttl_seconds")
            .and_then(|v| v.as_integer())
            .unwrap_or(3600) as u64;

        code.push_str(&format!(
            "    pub const CACHE_DIR: &str = \"{}\";\n",
            cache_dir
        ));
        code.push_str(&format!(
            "    pub const CACHE_FILE: &str = \"{}\";\n",
            cache_file
        ));
        code.push_str(&format!(
            "    pub const TTL_SECONDS: u64 = {};\n\n",
            ttl_seconds
        ));
    }

    // 更新配置
    if let Some(update) = config.get("update") {
        let check_interval = update
            .get("check_interval_seconds")
            .and_then(|v| v.as_integer())
            .unwrap_or(300) as u64;
        let timeout = update
            .get("timeout_seconds")
            .and_then(|v| v.as_integer())
            .unwrap_or(30) as u64;
        let retry_count = update
            .get("retry_count")
            .and_then(|v| v.as_integer())
            .unwrap_or(3) as u32;

        code.push_str(&format!(
            "    pub const CHECK_INTERVAL_SECONDS: u64 = {};\n",
            check_interval
        ));
        code.push_str(&format!(
            "    pub const TIMEOUT_SECONDS: u64 = {};\n",
            timeout
        ));
        code.push_str(&format!(
            "    pub const RETRY_COUNT: u32 = {};\n\n",
            retry_count
        ));
    }
}

/// 生成禁用的云控配置
fn generate_disabled_cloud_config(code: &mut String) {
    code.push_str("use crate::cloud_control::types::{CloudControlConfig, RepositoryConfig, CacheConfig, UpdateConfig};\n\n");
    code.push_str("/// 禁用的云控配置（因为配置文件不存在或无效）\n");
    code.push_str("pub mod embedded_cloud_config {\n");
    code.push_str("    use super::*;\n\n");
    code.push_str("    /// 获取禁用的云控配置\n");
    code.push_str("    pub fn get_config() -> CloudControlConfig {\n");
    code.push_str("        CloudControlConfig {\n");
    code.push_str("            enabled: false,\n");
    code.push_str("            repository: RepositoryConfig {\n");
    code.push_str("                url: String::new(),\n");
    code.push_str("                branch: \"main\".to_string(),\n");
    code.push_str("                data_file: \"blacklist.toml\".to_string(),\n");
    code.push_str("                access_token: None,\n");
    code.push_str("            },\n");
    code.push_str("            cache: CacheConfig {\n");
    code.push_str("                cache_dir: \".cache/cloud_control\".to_string(),\n");
    code.push_str("                cache_file: \"cloud_data.json\".to_string(),\n");
    code.push_str("                ttl_seconds: 3600,\n");
    code.push_str("            },\n");
    code.push_str("            update: UpdateConfig {\n");
    code.push_str("                check_interval_seconds: 300,\n");
    code.push_str("                timeout_seconds: 30,\n");
    code.push_str("                retry_count: 3,\n");
    code.push_str("            },\n");
    code.push_str("        }\n");
    code.push_str("    }\n");
    code.push_str("}\n");
}
