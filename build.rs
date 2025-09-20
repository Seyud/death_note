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
    let config_content = fs::read_to_string("cloud_config.toml").unwrap_or_else(|_| {
        println!("警告: cloud_config.toml 不存在，将使用默认的禁用配置");
        // 提供一个默认的、禁用的配置字符串
        r#"
                enabled = false
                [repository]
                url = ""
                branch = "main"
                data_file = "blacklist.toml"
                [cache]
                cache_dir = ".cache/cloud_control"
                cache_file = "cloud_data.json"
                ttl_seconds = 3600
                [update]
                check_interval_seconds = 300
                timeout_seconds = 30
                retry_count = 3
            "#
        .to_string()
    });

    // 验证配置内容是否是有效的TOML
    if let Err(e) = toml::from_str::<toml::Value>(&config_content) {
        panic!("错误: cloud_config.toml 解析失败: {}", e);
    }

    // 生成包含配置字符串的模块
    code.push_str("/// 编译时嵌入的云控配置\n");
    code.push_str("pub mod embedded_cloud_config {\n");
    code.push_str("    /// 获取编译时嵌入的云控配置字符串\n");
    code.push_str("    pub fn get_config_str() -> &'static str {\n");
    code.push_str(&format!("        r#\"{}\"#\n", config_content));
    code.push_str("    }\n");
    code.push_str("}\n");

    // 写入生成的代码
    fs::write(dest_path, code).expect("无法写入生成的云控配置代码");
}
