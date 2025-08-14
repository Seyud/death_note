//! 异步酷安识别器
//! 将原有的同步酷安识别改造为异步版本

use crate::identification::traits::{
    GenericIdentificationResult, IdentificationResult, Identifier,
};
use async_trait::async_trait;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::fs;
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
        let file_path =
            "/data/data/com.coolapk.market/shared_prefs/mobclick_agent_user_com.coolapk.market.xml";

        // 检查文件是否存在
        if !Path::new(file_path).exists() {
            return Err(format!("酷安配置文件 {} 不存在", file_path).into());
        }

        // 异步读取文件内容
        let content = tokio::fs::read_to_string(file_path).await?;

        // 解析XML并提取UID
        let uid = self.extract_uid_from_xml(&content)?;

        let result = GenericIdentificationResult::new(uid.clone(), "酷安".to_string())
            .with_package_name("com.coolapk.market".to_string())
            .with_config_path(file_path.to_string())
            .with_additional_info("应用名称".to_string(), "酷安市场".to_string());

        Ok(vec![Box::new(result)])
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

                        if is_au_u {
                            if let Ok(Event::Text(ref t)) = reader.read_event_into(&mut buf) {
                                let value = t
                                    .unescape()
                                    .map_err(|e| format!("XML转义错误: {}", e))?
                                    .to_string();
                                return Ok(value);
                            }
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
