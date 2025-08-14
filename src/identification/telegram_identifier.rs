//! 异步Telegram识别器
//! 将原有的同步Telegram识别改造为异步版本

use crate::identification::traits::{
    GenericIdentificationResult, IdentificationResult, Identifier,
};
use async_trait::async_trait;
use regex::Regex;
use std::path::Path;
use tokio::fs;

/// Telegram识别器
pub struct TelegramIdentifier;

impl TelegramIdentifier {
    pub fn new() -> Self {
        Self
    }

    /// 异步扫描并识别Telegram相关的配置信息
    async fn identify_telegram_async(
        &self,
    ) -> Result<Vec<Box<dyn IdentificationResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();
        let data_path = Path::new("/data/data");

        if !data_path.exists() {
            return Ok(results);
        }

        // 异步读取目录
        let mut entries = fs::read_dir(data_path).await?;

        // 并发处理所有包含"gram"的文件夹
        let mut tasks = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if let Some(folder_name) = path.file_name().and_then(|n| n.to_str()) {
                if folder_name.contains("gram") {
                    let task = self.process_telegram_folder_async(path);
                    tasks.push(task);
                }
            }
        }

        // 并发执行所有任务
        let task_results = futures::future::join_all(tasks).await;

        // 收集结果
        for result in task_results {
            if let Ok(Some(telegram_info)) = result {
                results.push(telegram_info);
            }
        }

        Ok(results)
    }

    /// 异步处理单个Telegram文件夹
    async fn process_telegram_folder_async(
        &self,
        folder_path: std::path::PathBuf,
    ) -> Result<Option<Box<dyn IdentificationResult>>, Box<dyn std::error::Error + Send + Sync>>
    {
        let shared_prefs_path = folder_path.join("shared_prefs");

        if !shared_prefs_path.exists() || !shared_prefs_path.is_dir() {
            return Ok(None);
        }

        // 异步读取shared_prefs目录
        let mut entries = fs::read_dir(&shared_prefs_path).await?;

        // 查找ringtones_pref_[UID].xml格式的文件
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
                if let Some(uid) = self.extract_uid_from_filename(file_name) {
                    let package_name = folder_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let result =
                        GenericIdentificationResult::new(uid.clone(), "Telegram".to_string())
                            .with_package_name(package_name.clone())
                            .with_config_path(file_path.to_string_lossy().to_string())
                            .with_additional_info(
                                "应用类型".to_string(),
                                "Telegram客户端".to_string(),
                            );

                    return Ok(Some(Box::new(result)));
                }
            }
        }

        Ok(None)
    }

    /// 从文件名中提取UID
    fn extract_uid_from_filename(&self, filename: &str) -> Option<String> {
        let re = Regex::new(r"ringtones_pref_(\d+)\.xml").ok()?;
        re.captures(filename)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string())
    }
}

#[async_trait]
impl Identifier for TelegramIdentifier {
    fn name(&self) -> &'static str {
        "Telegram识别器"
    }

    async fn identify(&self) -> Vec<Box<dyn IdentificationResult>> {
        match self.identify_telegram_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("Telegram识别错误: {}", e);
                Vec::new()
            }
        }
    }
}
