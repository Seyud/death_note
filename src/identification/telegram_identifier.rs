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

        // 扫描系统数据路径
        let data_path = Path::new("/data/data");
        if data_path.exists() {
            let mut entries = fs::read_dir(data_path).await?;

            // 并发处理所有包含"gram"的文件夹
            let mut tasks = Vec::new();
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if let Some(folder_name) = path.file_name().and_then(|n| n.to_str())
                    && folder_name.contains("gram")
                {
                    let task = self.process_telegram_folder_async(path);
                    tasks.push(task);
                }
            }

            // 并发执行所有任务
            let task_results = futures::future::join_all(tasks).await;

            // 收集结果（使用 flatten 简化 Result 处理，避免 manual_flatten）
            for telegram_infos in task_results.into_iter().flatten() {
                results.extend(telegram_infos);
            }
        }

        // 仅在测试编译时扫描本地 test_data（不会进入发布二进制）
        #[cfg(test)]
        {
            let test_data_path = Path::new("test_data");
            if test_data_path.exists() {
                let mut test_entries = fs::read_dir(test_data_path).await?;

                while let Some(entry) = test_entries.next_entry().await? {
                    let path = entry.path();
                    if let Some(folder_name) = path.file_name().and_then(|n| n.to_str())
                        && folder_name.contains("gram")
                    {
                        // 扫描测试数据下的 telegram 文件夹
                        let mut app_entries = fs::read_dir(&path).await?;

                        while let Some(app_entry) = app_entries.next_entry().await? {
                            let app_path = app_entry.path();
                            if let Some(app_name) = app_path.file_name().and_then(|n| n.to_str())
                                && app_name.contains("telegram")
                            {
                                let task = self.process_telegram_folder_async(app_path);
                                if let Ok(telegram_infos) = task.await {
                                    results.extend(telegram_infos);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// 异步处理单个Telegram文件夹
    async fn process_telegram_folder_async(
        &self,
        folder_path: std::path::PathBuf,
    ) -> Result<Vec<Box<dyn IdentificationResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results = Vec::new();
        let shared_prefs_path = folder_path.join("shared_prefs");

        if !shared_prefs_path.exists() || !shared_prefs_path.is_dir() {
            return Ok(results);
        }

        // 异步读取shared_prefs目录
        let mut entries = fs::read_dir(&shared_prefs_path).await?;

        // 查找所有ringtones_pref_[UID].xml格式的文件
        while let Some(entry) = entries.next_entry().await? {
            let file_path = entry.path();
            if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str())
                && let Some(uid) = self.extract_uid_from_filename(file_name)
            {
                let result = GenericIdentificationResult::new(uid.clone(), "Telegram".to_string());

                results.push(Box::new(result));
            }
        }

        Ok(results)
    }

    /// 从文件名中提取UID
    fn extract_uid_from_filename(&self, filename: &str) -> Option<String> {
        let re = Regex::new(r"ringtones_pref_(\d+)\.xml").ok()?;
        re.captures(filename)
            .and_then(|captures| captures.get(1))
            .map(|m| m.as_str().to_string())
    }
}

impl Default for TelegramIdentifier {
    fn default() -> Self {
        Self::new()
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
