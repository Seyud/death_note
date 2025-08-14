//! 异步酷安识别器
//! 将原有的同步酷安识别改造为异步版本

use crate::identification::traits::{
    GenericIdentificationResult, IdentificationResult, Identifier,
};
use async_trait::async_trait;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;
use std::path::Path;

/// 酷安识别器
pub struct CoolapkIdentifier;

impl CoolapkIdentifier {
    pub fn new() -> Self {
        Self
    }

    /// 异步获取酷安UID
    async fn get_coolapk_uid_async(
        &self,
    ) -> Result<Vec<Box<dyn IdentificationResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results: Vec<Box<dyn IdentificationResult>> = Vec::new();

        // 扫描系统数据路径
        let file_path =
            "/data/data/com.coolapk.market/shared_prefs/mobclick_agent_user_com.coolapk.market.xml";

        if Path::new(file_path).exists() {
            let content = tokio::fs::read_to_string(file_path).await?;
            if let Ok(uid) = self.extract_uid_from_xml(&content) {
                let result = GenericIdentificationResult::new(uid.clone(), "酷安".to_string());

                results.push(Box::new(result) as Box<dyn IdentificationResult>);
            }
        }

        // 仅在测试编译时扫描本地 test_data（不进入发布二进制）
        #[cfg(test)]
        {
            let test_data_path = Path::new("test_data/coolapk");
            if test_data_path.exists() {
                let mut test_entries = tokio::fs::read_dir(test_data_path).await?;

                while let Some(entry) = test_entries.next_entry().await? {
                    let app_path = entry.path();
                    if let Some(app_name) = app_path.file_name().and_then(|n| n.to_str())
                        && app_name.contains("coolapk")
                    {
                        let shared_prefs_path = app_path.join("shared_prefs");
                        if shared_prefs_path.exists() {
                            let mut prefs_entries = tokio::fs::read_dir(&shared_prefs_path).await?;

                            while let Some(prefs_entry) = prefs_entries.next_entry().await? {
                                let file_path = prefs_entry.path();
                                if let Some(file_name) =
                                    file_path.file_name().and_then(|n| n.to_str())
                                    && file_name.ends_with(".xml")
                                {
                                    let content = tokio::fs::read_to_string(&file_path).await?;
                                    if let Ok(uid) = self.extract_uid_from_coolapk_xml(&content) {
                                        let result = GenericIdentificationResult::new(
                                            uid.clone(),
                                            "酷安".to_string(),
                                        );

                                        results.push(Box::new(result) as Box<dyn IdentificationResult>);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// 从XML内容中提取UID
    fn extract_uid_from_xml(
        &self,
        xml_content: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cursor = Cursor::new(xml_content);
        let mut reader = Reader::from_reader(cursor);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"string" {
                        let mut is_au_u = false;
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| format!("XML属性错误: {}", e))?;
                            if attr.key.as_ref() == b"name" && attr.value.as_ref() == b"au_u" {
                                is_au_u = true;
                            }
                        }

                        if is_au_u && let Ok(Event::Text(ref t)) = reader.read_event_into(&mut buf)
                        {
                            let value = t
                                .unescape()
                                .map_err(|e| format!("XML转义错误: {}", e))?
                                .to_string();
                            return Ok(value);
                        }
                    }
                }
                Err(e) => return Err(format!("XML解析错误: {}", e).into()),
                _ => (),
            }
            buf.clear();
        }

        Err("未找到酷安UID字段".into())
    }

    /// 从酷安测试数据XML中提取UID（仅测试使用）
    #[cfg(test)]
    fn extract_uid_from_coolapk_xml(
        &self,
        xml_content: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cursor = Cursor::new(xml_content);
        let mut reader = Reader::from_reader(cursor);
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() == b"string" {
                        let mut is_uid = false;
                        for attr in e.attributes() {
                            let attr = attr.map_err(|e| format!("XML属性错误: {}", e))?;
                            if attr.key.as_ref() == b"name" && attr.value.as_ref() == b"uid" {
                                is_uid = true;
                            }
                        }

                        if is_uid && let Ok(Event::Text(ref t)) = reader.read_event_into(&mut buf) {
                            let value = t
                                .unescape()
                                .map_err(|e| format!("XML转义错误: {}", e))?
                                .to_string();
                            return Ok(value);
                        }
                    }
                }
                Err(e) => return Err(format!("XML解析错误: {}", e).into()),
                _ => (),
            }
            buf.clear();
        }

        Err("未找到酷安UID字段".into())
    }
}

impl Default for CoolapkIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Identifier for CoolapkIdentifier {
    fn name(&self) -> &'static str {
        "酷安识别器"
    }

    async fn identify(&self) -> Vec<Box<dyn IdentificationResult>> {
        match self.get_coolapk_uid_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("酷安识别错误: {}", e);
                Vec::new()
            }
        }
    }
}
