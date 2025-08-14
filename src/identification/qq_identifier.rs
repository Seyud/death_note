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

        let qq_package = "com.tencent.mobileqq";
        let data_path = format!("/data/data/{}/shared_prefs/", qq_package);

        if !Path::new(&data_path).exists() {
            return Ok(results);
        }

        // 读取shared_prefs目录下的所有文件
        let mut entries = fs::read_dir(&data_path).await?;

        // 查找acc_info开头的配置文件
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("acc_info") && file_name.ends_with(".xml") {
                    // 从文件名提取QQ号
                    let re = Regex::new(r"acc_info(\d+)\.xml").unwrap();
                    if let Some(captures) = re.captures(file_name) {
                        if let Some(qq_number) = captures.get(1) {
                            let qq_uid = qq_number.as_str().to_string();

                            let result = Box::new(
                                GenericIdentificationResult::new(qq_uid.clone(), "QQ".to_string())
                                    .with_package_name(qq_package.to_string())
                                    .with_config_path(path.to_string_lossy().to_string())
                                    .with_additional_info(
                                        "应用类型".to_string(),
                                        "腾讯社交应用".to_string(),
                                    )
                                    .with_additional_info(
                                        "识别方式".to_string(),
                                        "配置文件名解析".to_string(),
                                    ),
                            )
                                as Box<dyn IdentificationResult>;

                            results.push(result);
                        }
                    }
                }
            }
        }

        Ok(results)
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
