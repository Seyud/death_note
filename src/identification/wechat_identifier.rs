use crate::blacklist::manager::DeathNote;
use crate::identification::lifespan_calculator::LifespanCalculator;
use crate::identification::traits::{GenericShinigamiEyeResult, ShinigamiEye, ShinigamiEyeResult};
use async_trait::async_trait;
use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Cursor;
use std::path::Path;
use tokio::fs;

pub struct WeChatShinigamiEye {
    lifespan_calculator: LifespanCalculator,
    death_note: DeathNote,
}

impl WeChatShinigamiEye {
    pub fn new() -> Self {
        Self {
            lifespan_calculator: LifespanCalculator::new(),
            death_note: DeathNote::new(),
        }
    }

    async fn perceive_wechat_users_async(
        &self,
    ) -> Result<Vec<Box<dyn ShinigamiEyeResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let mut results: Vec<Box<dyn ShinigamiEyeResult>> = Vec::new();

        let file_path = "/data/user/0/com.tencent.mm/shared_prefs/com.tencent.mm_preferences.xml";
        if Path::new(file_path).exists() {
            let content = fs::read_to_string(file_path).await?;
            results.extend(self.extract_results_from_xml(&content)?);
        }

        #[cfg(any(test, debug_assertions))]
        {
            let test_file_path = Path::new(
                "test_data/wechat/com.tencent.mm/shared_prefs/com.tencent.mm_preferences.xml",
            );
            if test_file_path.exists() {
                let content = fs::read_to_string(test_file_path).await?;
                results.extend(self.extract_results_from_xml(&content)?);
            }
        }

        Ok(results)
    }

    fn extract_results_from_xml(
        &self,
        xml_content: &str,
    ) -> Result<Vec<Box<dyn ShinigamiEyeResult>>, Box<dyn std::error::Error + Send + Sync>> {
        let extracted = self.extract_fields(xml_content)?;
        let mut results: Vec<Box<dyn ShinigamiEyeResult>> = Vec::new();

        if let Some(value) = extracted.last_login_bind_mobile {
            results.push(self.make_result(value, "微信-手机号", true)?);
        }
        if let Some(value) = extracted.last_login_alias {
            results.push(self.make_result(value, "微信-微信号", true)?);
        }
        if let Some(value) = extracted.login_weixin_username {
            results.push(self.make_result(value, "微信-wxid", false)?);
        }
        if let Some(value) = extracted.last_login_nick_name {
            results.push(self.make_result(value, "微信-昵称", false)?);
        }

        Ok(results)
    }

    fn make_result(
        &self,
        value: String,
        field_source: &str,
        should_check_blacklist: bool,
    ) -> Result<Box<dyn ShinigamiEyeResult>, Box<dyn std::error::Error + Send + Sync>> {
        let is_blacklisted = if should_check_blacklist {
            self.death_note.is_wechat_target_local_only(&value)
        } else {
            false
        };
        let lifespan = self
            .lifespan_calculator
            .calculate_lifespan(&value, is_blacklisted);
        Ok(Box::new(GenericShinigamiEyeResult::new(
            value,
            field_source.to_string(),
            lifespan,
            is_blacklisted,
        )) as Box<dyn ShinigamiEyeResult>)
    }

    fn extract_fields(
        &self,
        xml_content: &str,
    ) -> Result<WeChatPrefsFields, Box<dyn std::error::Error + Send + Sync>> {
        let cursor = Cursor::new(xml_content);
        let mut reader = Reader::from_reader(cursor);
        let mut buf = Vec::new();

        let mut fields = WeChatPrefsFields::default();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Ok(Event::Start(ref e)) | Ok(Event::Empty(ref e)) => {
                    if e.name().as_ref() != b"string" {
                        buf.clear();
                        continue;
                    }

                    let mut key: Option<String> = None;
                    for attr in e.attributes() {
                        let attr = attr?;
                        if attr.key.as_ref() == b"name" {
                            key = Some(String::from_utf8(attr.value.into_owned())?);
                            break;
                        }
                    }

                    let Some(key) = key else {
                        buf.clear();
                        continue;
                    };

                    if !fields.is_target_key(&key) {
                        buf.clear();
                        continue;
                    }

                    let value = match reader.read_event_into(&mut buf) {
                        Ok(Event::Text(ref t)) => t.decode()?.to_string(),
                        _ => {
                            buf.clear();
                            continue;
                        }
                    };

                    fields.set_value(key, value);
                }
                Err(e) => return Err(e.into()),
                _ => (),
            }
            buf.clear();
        }

        Ok(fields)
    }
}

impl Default for WeChatShinigamiEye {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ShinigamiEye for WeChatShinigamiEye {
    fn name(&self) -> &'static str {
        "WeChat死神之眼"
    }

    async fn identify(&self) -> Vec<Box<dyn ShinigamiEyeResult>> {
        match self.perceive_wechat_users_async().await {
            Ok(results) => results,
            Err(e) => {
                eprintln!("WeChat死神之眼感知错误: {}", e);
                Vec::new()
            }
        }
    }
}

#[derive(Default)]
struct WeChatPrefsFields {
    last_login_bind_mobile: Option<String>,
    last_login_alias: Option<String>,
    login_weixin_username: Option<String>,
    last_login_nick_name: Option<String>,
}

impl WeChatPrefsFields {
    fn is_target_key(&self, key: &str) -> bool {
        matches!(
            key,
            "last_login_bind_mobile"
                | "last_login_alias"
                | "login_weixin_username"
                | "last_login_nick_name"
        )
    }

    fn set_value(&mut self, key: String, value: String) {
        if value.is_empty() {
            return;
        }

        match key.as_str() {
            "last_login_bind_mobile" => self.last_login_bind_mobile = Some(value),
            "last_login_alias" => self.last_login_alias = Some(value),
            "login_weixin_username" => self.login_weixin_username = Some(value),
            "last_login_nick_name" => self.last_login_nick_name = Some(value),
            _ => {}
        }
    }
}
