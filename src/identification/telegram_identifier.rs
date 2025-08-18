//! 死神之眼 - Telegram版本
//! 琉克的死神之眼能够看透Telegram用户的真名和寿命

use crate::identification::traits::{GenericShinigamiEyeResult, ShinigamiEye, ShinigamiEyeResult};
use async_trait::async_trait;
use regex::Regex;
use std::path::Path;
use tokio::fs;

/// Telegram死神之眼
/// 能够看透Telegram用户真名和剩余寿命的死神之眼
pub struct TelegramShinigamiEye;

impl TelegramShinigamiEye {
    pub fn new() -> Self {
        Self
    }

    /// 使用死神之眼获取Telegram用户的真名和寿命
    async fn perceive_telegram_users_async(
        &self,
    ) -> Result<Vec<Box<dyn ShinigamiEyeResult>>, Box<dyn std::error::Error + Send + Sync>> {
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
    ) -> Result<Vec<Box<dyn ShinigamiEyeResult>>, Box<dyn std::error::Error + Send + Sync>> {
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
                let lifespan = self.calculate_lifespan(&uid);
                let result =
                    GenericShinigamiEyeResult::new(uid.clone(), "Telegram".to_string(), lifespan);

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

    /// 根据UID计算剩余寿命
    fn calculate_lifespan(&self, uid: &str) -> String {
        // 使用UID的哈希值计算剩余寿命，模拟死神之眼的寿命感知
        let hash = uid.chars().map(|c| c as u32).sum::<u32>();
        let years = (hash % 50) + 1; // 1-50年的剩余寿命
        format!("{}年", years)
    }
}

impl Default for TelegramShinigamiEye {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShinigamiEye for TelegramShinigamiEye {
    fn name(&self) -> &'static str {
        "Telegram死神之眼"
    }

    async fn identify(&self) -> Vec<Box<dyn ShinigamiEyeResult>> {
        match self.perceive_telegram_users_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("Telegram死神之眼感知错误: {}", e);
                Vec::new()
            }
        }
    }
}
