use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;
use zed_extension_api::{self as zed, i18n::I18NExtension};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TranslationData {
    #[serde(flatten)]
    translations: HashMap<String, String>,  // 使用 String 而不是 Value
}

struct I18nExtension {
    translations: RwLock<Option<TranslationData>>,
    work_dir: PathBuf,
}

impl I18nExtension {
    fn new() -> Self {
        let extension = Self {
            translations: RwLock::new(None),
            work_dir: std::env::current_dir().unwrap_or_default(),
        };

        // 初始化时加载翻译数据
        if let Ok(data) = extension.load_translations() {
            *extension.translations.write().unwrap() = Some(data);
        }

        extension
    }

    fn get_translation(&self, full_key: &str) -> String {
        let guard = match self.translations.read() {
            Ok(guard) => guard,
            Err(_) => return full_key.to_string(),
        };
        
        let data = match &*guard {
            Some(data) => data,
            None => return full_key.to_string(),
        };

        data.translations.get(full_key)
            .cloned()
            .unwrap_or_else(|| full_key.to_string())
    }

    fn load_translations(&self) -> Result<TranslationData> {
        let translation_file = self.work_dir.join("resources").join("translation.json");
        
        // 加载翻译文件
        if let Ok(content) = std::fs::read_to_string(translation_file) {
            let data: TranslationData = serde_json::from_str(&content)?;
            Ok(data)
        } else {
            Ok(TranslationData::default())
        }
    }
}

impl zed::Extension for I18nExtension {
    fn new() -> Self {
        Self::new()
    }
}

impl I18NExtension for I18nExtension {
    fn get_id(&self) -> &str {
        "zh-CN"
    }
    
    fn get_display_name(&self) -> &str {
        "简体中文 (Simplified Chinese)"
    }
    
    fn get_version(&self) -> &str {
        "0.1.0"
    }

    fn load_translations(&self) -> std::io::Result<HashMap<String, String>> {
        let mut translations = HashMap::new();
        let translation_file = self.work_dir.join("resources").join("translation.json");

        // 从单个translation.json文件加载所有翻译
        if let Ok(content) = std::fs::read_to_string(translation_file) {
            if let Ok(data) = serde_json::from_str::<TranslationData>(&content) {
                translations.extend(data.translations);
            }
        }

        Ok(translations)
    }
}

// 注册扩展
zed::register_extension!(I18nExtension);