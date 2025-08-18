//! 死神之眼 - QQ版本
//! 琉克的死神之眼能够看透QQ用户的真名和寿命

use crate::identification::traits::{GenericShinigamiEyeResult, ShinigamiEye, ShinigamiEyeResult};
use async_trait::async_trait;
use regex::Regex;
use std::path::Path;
use tokio::fs;

/// QQ死神之眼
/// 能够看透QQ用户真名和剩余寿命的死神之眼
pub struct QQShinigamiEye;

impl QQShinigamiEye {
    pub fn new() -> Self {
        Self
    }

    /// 使用死神之眼获取QQ用户的真名和寿命
    async fn perceive_qq_users_async(
        &self,
    ) -> Result<Vec<Box<dyn ShinigamiEyeResult>>, Box<dyn std::error::Error + Send + Sync>> {
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
                    let lifespan = self.calculate_lifespan(&qq_uid);

                    let result = Box::new(GenericShinigamiEyeResult::new(
                        qq_uid.clone(),
                        "QQ".to_string(),
                        lifespan,
                    )) as Box<dyn ShinigamiEyeResult>;

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
                                    let lifespan = self.calculate_lifespan(&qq_uid);

                                    let result = Box::new(GenericShinigamiEyeResult::new(
                                        qq_uid.clone(),
                                        "QQ".to_string(),
                                        lifespan,
                                    ))
                                        as Box<dyn ShinigamiEyeResult>;

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

    /// 根据QQ号计算剩余寿命
    fn calculate_lifespan(&self, qq_uid: &str) -> String {
        // 使用QQ号的哈希值计算剩余寿命，模拟死神之眼的寿命感知
        let hash = qq_uid.chars().map(|c| c as u32).sum::<u32>();
        let years = (hash % 60) + 5; // 5-65年的剩余寿命
        format!("{}年", years)
    }
}

impl Default for QQShinigamiEye {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShinigamiEye for QQShinigamiEye {
    fn name(&self) -> &'static str {
        "QQ死神之眼"
    }

    async fn identify(&self) -> Vec<Box<dyn ShinigamiEyeResult>> {
        match self.perceive_qq_users_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("QQ死神之眼感知错误: {}", e);
                Vec::new()
            }
        }
    }
}
