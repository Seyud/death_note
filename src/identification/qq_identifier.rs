//! 示例识别器模板
//! 展示如何添加新的软件识别器

use crate::identification::traits::{
    GenericIdentificationResult, IdentificationResult, Identifier,
};
use async_trait::async_trait;
use regex::Regex;
use std::path::Path;
use tokio::fs;

/// QQ识别器示例
pub struct QQAsyncIdentifier;

impl QQAsyncIdentifier {
    pub fn new() -> Self {
        Self
    }

    async fn identify_qq_async(
        &self,
    ) -> Result<Vec<Box<dyn IdentificationResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();

        // 预编译复用的正则，避免在循环中多次编译（clippy::regex_creation_in_loops）
        let acc_info_re = Regex::new(r"acc_info(\d+)\.xml").unwrap();

        // 扫描系统数据路径
        let qq_package = "com.tencent.mobileqq";
        let data_path = format!("/data/data/{}/shared_prefs/", qq_package);

        if Path::new(&data_path).exists() {
            // 读取shared_prefs目录下的所有文件
            let mut entries = fs::read_dir(&data_path).await?;
            // 查找acc_info开头的配置文件（使用已预编译的 acc_info_re）
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
                    && file_name.starts_with("acc_info")
                    && file_name.ends_with(".xml")
                    && let Some(captures) = acc_info_re.captures(file_name)
                    && let Some(qq_number) = captures.get(1)
                {
                    let qq_uid = qq_number.as_str().to_string();

                    let result = Box::new(GenericIdentificationResult::new(
                        qq_uid.clone(),
                        "QQ".to_string(),
                    )) as Box<dyn IdentificationResult>;

                    results.push(result);
                }
            }
        }

        // 仅在测试编译时扫描本地 test_data（不进入发布二进制）
        #[cfg(test)]
        {
            let test_data_path = Path::new("test_data/qq");
            if test_data_path.exists() {
                let mut test_entries = fs::read_dir(test_data_path).await?;

                while let Some(entry) = test_entries.next_entry().await? {
                    let app_path = entry.path();
                    if let Some(app_name) = app_path.file_name().and_then(|n| n.to_str())
                        && app_name.contains("mobileqq")
                    {
                        let shared_prefs_path = app_path.join("shared_prefs");
                        if shared_prefs_path.exists() {
                            let mut prefs_entries = fs::read_dir(&shared_prefs_path).await?;
                            // 复用 acc_info_re 避免在循环中重新编译
                            while let Some(prefs_entry) = prefs_entries.next_entry().await? {
                                let path = prefs_entry.path();
                                if let Some(file_name) = path.file_name().and_then(|n| n.to_str())
                                    && file_name.starts_with("acc_info")
                                    && file_name.ends_with(".xml")
                                    && let Some(captures) = acc_info_re.captures(file_name)
                                    && let Some(qq_number) = captures.get(1)
                                {
                                    let qq_uid = qq_number.as_str().to_string();

                                    let result = Box::new(GenericIdentificationResult::new(
                                        qq_uid.clone(),
                                        "QQ".to_string(),
                                    ))
                                        as Box<dyn IdentificationResult>;

                                    results.push(result);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }
}

impl Default for QQAsyncIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Identifier for QQAsyncIdentifier {
    fn name(&self) -> &'static str {
        "QQ识别器"
    }

    async fn identify(&self) -> Vec<Box<dyn IdentificationResult>> {
        match self.identify_qq_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("QQ识别错误: {}", e);
                Vec::new()
            }
        }
    }
}
