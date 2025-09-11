use std::path::Path;
use std::process::Command;
use tokio::process::Command as AsyncCommand;

/// Android 分区操作模块
/// 用于实现 A/B 和 VAB 设备的分区还原机制
pub struct AndroidPartitionOperator {
    pub device_type: DeviceType,
    pub current_slot: String,
}

/// 设备类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    /// A/B 设备（支持无缝更新）
    AB,
    /// VAB 设备（虚拟 A/B）
    VAB,
    /// A-only 设备（传统单分区）
    AOnly,
}

impl AndroidPartitionOperator {
    /// 创建新的分区操作实例
    pub fn new() -> Result<Self, std::io::Error> {
        let device_type = Self::detect_device_type()?;
        let current_slot = Self::get_current_slot(&device_type)?;

        Ok(Self {
            device_type,
            current_slot,
        })
    }

    /// 检测设备类型
    fn detect_device_type() -> Result<DeviceType, std::io::Error> {
        // 检查是否存在 A/B 分区标识
        let ab_check = Command::new("getprop").arg("ro.boot.slot_suffix").output();

        match ab_check {
            Ok(output) if !output.stdout.is_empty() => {
                let slot_suffix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !slot_suffix.is_empty() {
                    // 进一步检查是否为 VAB
                    let vab_check = Command::new("getprop")
                        .arg("ro.virtual_ab.enabled")
                        .output();

                    if let Ok(vab_output) = vab_check {
                        let vab_string = String::from_utf8_lossy(&vab_output.stdout);
                        let vab_enabled = vab_string.trim();
                        if vab_enabled == "true" {
                            return Ok(DeviceType::VAB);
                        }
                    }
                    Ok(DeviceType::AB)
                } else {
                    Ok(DeviceType::AOnly)
                }
            }
            _ => Ok(DeviceType::AOnly),
        }
    }

    /// 获取当前槽位
    fn get_current_slot(device_type: &DeviceType) -> Result<String, std::io::Error> {
        match device_type {
            DeviceType::AB | DeviceType::VAB => {
                let output = Command::new("getprop")
                    .arg("ro.boot.slot_suffix")
                    .output()?;

                let slot_suffix = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if let Some(stripped) = slot_suffix.strip_prefix('_') {
                    Ok(stripped.to_string()) // 移除前缀下划线
                } else {
                    Ok("a".to_string()) // 默认为 a 槽位
                }
            }
            DeviceType::AOnly => Ok("single".to_string()),
        }
    }

    /// 获取另一个槽位
    fn get_other_slot(&self) -> String {
        match self.current_slot.as_str() {
            "a" => "b".to_string(),
            "b" => "a".to_string(),
            _ => "a".to_string(), // 默认返回 a
        }
    }

    /// 异步执行分区还原操作
    pub async fn restore_partitions_async(&self) -> Result<PartitionRestoreResult, std::io::Error> {
        println!(
            "🔍 检测到设备类型: {:?}, 当前槽位: {}",
            self.device_type, self.current_slot
        );

        match self.device_type {
            DeviceType::AB | DeviceType::VAB => self.restore_ab_device_async().await,
            DeviceType::AOnly => self.restore_a_only_device_async().await,
        }
    }

    /// A/B 或 VAB 设备的分区还原
    async fn restore_ab_device_async(&self) -> Result<PartitionRestoreResult, std::io::Error> {
        let other_slot = self.get_other_slot();
        let mut restored_partitions = Vec::new();
        let mut failed_partitions = Vec::new();

        println!(
            "🔄 A/B 设备还原: 从槽位 {} 复制到槽位 {}",
            other_slot, self.current_slot
        );

        // 还原 boot 分区
        match self
            .copy_partition_async("boot", &other_slot, &self.current_slot)
            .await
        {
            Ok(_) => {
                restored_partitions.push("boot".to_string());
                println!("✅ boot 分区还原成功");
            }
            Err(e) => {
                failed_partitions.push(("boot".to_string(), e.to_string()));
                println!("❌ boot 分区还原失败: {}", e);
            }
        }

        // 还原 init_boot 分区（如果存在）
        if self.partition_exists(&format!("init_boot_{}", other_slot)) {
            match self
                .copy_partition_async("init_boot", &other_slot, &self.current_slot)
                .await
            {
                Ok(_) => {
                    restored_partitions.push("init_boot".to_string());
                    println!("✅ init_boot 分区还原成功");
                }
                Err(e) => {
                    failed_partitions.push(("init_boot".to_string(), e.to_string()));
                    println!("❌ init_boot 分区还原失败: {}", e);
                }
            }
        } else {
            println!("ℹ️ init_boot 分区不存在，跳过");
        }

        Ok(PartitionRestoreResult {
            device_type: self.device_type.clone(),
            restored_partitions,
            failed_partitions,
            operation_type: "A/B槽位交换".to_string(),
        })
    }

    /// A-only 设备的分区还原
    async fn restore_a_only_device_async(&self) -> Result<PartitionRestoreResult, std::io::Error> {
        let mut restored_partitions = Vec::new();
        let mut failed_partitions = Vec::new();

        println!("🔄 A-only 设备还原: boot ↔ recovery 分区交换");

        // 创建临时目录存储分区镜像
        let temp_dir = "/data/local/tmp/partition_backup";
        self.create_temp_dir(temp_dir).await?;

        // 1. 提取 boot 分区
        let boot_backup_path = format!("{}/boot_backup.img", temp_dir);
        match self
            .extract_partition_async("boot", &boot_backup_path)
            .await
        {
            Ok(_) => println!("✅ boot 分区备份成功"),
            Err(e) => {
                failed_partitions.push(("boot_extract".to_string(), e.to_string()));
                println!("❌ boot 分区备份失败: {}", e);
            }
        }

        // 2. 提取 recovery 分区
        let recovery_backup_path = format!("{}/recovery_backup.img", temp_dir);
        match self
            .extract_partition_async("recovery", &recovery_backup_path)
            .await
        {
            Ok(_) => println!("✅ recovery 分区备份成功"),
            Err(e) => {
                failed_partitions.push(("recovery_extract".to_string(), e.to_string()));
                println!("❌ recovery 分区备份失败: {}", e);
            }
        }

        // 3. 交换分区：boot 备份刷入 recovery，recovery 备份刷入 boot
        if Path::new(&boot_backup_path).exists() && Path::new(&recovery_backup_path).exists() {
            // 将 boot 备份刷入 recovery 分区
            match self
                .flash_partition_async("recovery", &boot_backup_path)
                .await
            {
                Ok(_) => {
                    restored_partitions.push("recovery".to_string());
                    println!("✅ boot → recovery 刷入成功");
                }
                Err(e) => {
                    failed_partitions.push(("recovery_flash".to_string(), e.to_string()));
                    println!("❌ boot → recovery 刷入失败: {}", e);
                }
            }

            // 将 recovery 备份刷入 boot 分区
            match self
                .flash_partition_async("boot", &recovery_backup_path)
                .await
            {
                Ok(_) => {
                    restored_partitions.push("boot".to_string());
                    println!("✅ recovery → boot 刷入成功");
                }
                Err(e) => {
                    failed_partitions.push(("boot_flash".to_string(), e.to_string()));
                    println!("❌ recovery → boot 刷入失败: {}", e);
                }
            }
        }

        // 清理临时文件
        let _ = self.cleanup_temp_dir(temp_dir).await;

        Ok(PartitionRestoreResult {
            device_type: self.device_type.clone(),
            restored_partitions,
            failed_partitions,
            operation_type: "A-only分区交换".to_string(),
        })
    }

    /// 复制分区（A/B 设备）
    async fn copy_partition_async(
        &self,
        partition_name: &str,
        from_slot: &str,
        to_slot: &str,
    ) -> Result<(), std::io::Error> {
        let from_partition = format!("{}_{}", partition_name, from_slot);
        let to_partition = format!("{}_{}", partition_name, to_slot);

        println!("📋 复制分区: {} → {}", from_partition, to_partition);

        // 使用 dd 命令复制分区
        let output = AsyncCommand::new("dd")
            .arg(format!("if=/dev/block/by-name/{}", from_partition))
            .arg(format!("of=/dev/block/by-name/{}", to_partition))
            .arg("bs=4096")
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(std::io::Error::other(format!("dd 命令失败: {}", error_msg)))
        }
    }

    /// 提取分区镜像
    async fn extract_partition_async(
        &self,
        partition_name: &str,
        output_path: &str,
    ) -> Result<(), std::io::Error> {
        println!("📤 提取分区: {} → {}", partition_name, output_path);

        let output = AsyncCommand::new("dd")
            .arg(format!("if=/dev/block/by-name/{}", partition_name))
            .arg(format!("of={}", output_path))
            .arg("bs=4096")
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(std::io::Error::other(format!(
                "分区提取失败: {}",
                error_msg
            )))
        }
    }

    /// 刷入分区镜像
    async fn flash_partition_async(
        &self,
        partition_name: &str,
        image_path: &str,
    ) -> Result<(), std::io::Error> {
        println!("📥 刷入分区: {} ← {}", partition_name, image_path);

        let output = AsyncCommand::new("dd")
            .arg(format!("if={}", image_path))
            .arg(format!("of=/dev/block/by-name/{}", partition_name))
            .arg("bs=4096")
            .output()
            .await?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(std::io::Error::other(format!(
                "分区刷入失败: {}",
                error_msg
            )))
        }
    }

    /// 检查分区是否存在
    fn partition_exists(&self, partition_name: &str) -> bool {
        Path::new(&format!("/dev/block/by-name/{}", partition_name)).exists()
    }

    /// 创建临时目录
    async fn create_temp_dir(&self, path: &str) -> Result<(), std::io::Error> {
        tokio::fs::create_dir_all(path).await
    }

    /// 清理临时目录
    async fn cleanup_temp_dir(&self, path: &str) -> Result<(), std::io::Error> {
        tokio::fs::remove_dir_all(path).await
    }
}

impl Default for AndroidPartitionOperator {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            device_type: DeviceType::AOnly,
            current_slot: "single".to_string(),
        })
    }
}

/// 分区还原结果
#[derive(Debug)]
pub struct PartitionRestoreResult {
    pub device_type: DeviceType,
    pub restored_partitions: Vec<String>,
    pub failed_partitions: Vec<(String, String)>,
    pub operation_type: String,
}

impl PartitionRestoreResult {
    /// 检查操作是否完全成功
    pub fn is_success(&self) -> bool {
        self.failed_partitions.is_empty() && !self.restored_partitions.is_empty()
    }

    /// 获取成功还原的分区数量
    pub fn success_count(&self) -> usize {
        self.restored_partitions.len()
    }

    /// 获取失败的分区数量
    pub fn failure_count(&self) -> usize {
        self.failed_partitions.len()
    }
}
