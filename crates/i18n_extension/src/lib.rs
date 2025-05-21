use anyhow::{Result, Context};
use async_trait::async_trait;
use gpui::{AppContext, BackgroundExecutor};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::Arc,
};
use crate::core::{I18nManager, I18nSettings, I18nLangMeta};

#[async_trait]
pub trait ExtensionI18nProxy: Send + Sync {
    fn register_i18n_lang(&self, i18n_lang_id: String, i18n_lang_name: String);
    fn provide_translation(&self, i18n_lang_id: String, key: String, text: String);
    fn get_current_i18n_lang(&self) -> Option<String>;
}

pub struct I18nExtension {
    translations: RwLock<Option<TranslationData>>,
    work_dir: PathBuf,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TranslationData {
    #[serde(flatten)]
    translations: HashMap<String, String>,
}

impl I18nExtension {
    pub fn new(work_dir: PathBuf) -> Self {
        Self {
            translations: RwLock::new(None),
            work_dir,
        }
    }

    pub async fn load_translations(&self) -> Result<()> {
        let translation_file = self.work_dir
            .join("resources")
            .join("translations")
            .join("translation.json");

        let content = tokio::fs::read_to_string(&translation_file)
            .await
            .context("Failed to read translation file")?;

        let data: TranslationData = serde_json::from_str(&content)
            .context("Failed to parse translation file")?;

        let mut translations = self.translations.write().unwrap();
        *translations = Some(data);

        Ok(())
    }

    pub fn get_translation(&self, key: &str) -> Option<String> {
        self.translations.read().unwrap()
            .as_ref()
            .and_then(|data| data.translations.get(key).cloned())
    }
}

pub struct ExtensionHostProxy {
    executor: BackgroundExecutor,
    i18n_manager: Arc<I18nManager>,
}

impl ExtensionHostProxy {
    pub fn new(executor: BackgroundExecutor, i18n_manager: Arc<I18nManager>) -> Self {
        Self {
            executor,
            i18n_manager,
        }
    }
}

#[async_trait]
impl ExtensionI18nProxy for ExtensionHostProxy {
    fn register_i18n_lang(&self, i18n_lang_id: String, i18n_lang_name: String) {
        let meta = I18nLangMeta {
            id: i18n_lang_id.clone(),
            name: i18n_lang_name.clone(),
            display_name: format!("{} ({})", i18n_lang_name, i18n_lang_id),
            extension_id: None,
            rtl: false,
        };

        // 更新设置
        I18nSettings::add_available_i18n_lang(meta, &mut AppContext::global());
    }

    fn provide_translation(&self, i18n_lang_id: String, key: String, text: String) {
        // 更新翻译资源
        self.i18n_manager.add_translation(i18n_lang_id, key, text);
    }

    fn get_current_i18n_lang(&self) -> Option<String> {
        I18nSettings::get_active_i18n_lang(&AppContext::global())
    }
}

pub fn register_i18n_extensions(cx: &mut AppContext) {
    let i18n_manager = I18nManager::global(cx);
    
    for extension in cx.installed_extensions() {
        // 检查是否是i18n语言扩展
        if extension.manifest.categories.contains(&"i18n".to_string()) {
            let i18n_lang_id = extension.manifest.i18n
                .as_ref()
                .map(|l| l.locale.clone())
                .unwrap_or_else(|| "unknown".to_string());
            
            // 加载翻译资源
            i18n_manager.register_i18n_lang_extension(
                i18n_lang_id.clone(),
                extension.path.clone()
            );
            
            // 如果是当前选择的i18n语言，应用它
            let settings = I18nSettings::get_global(cx);
            if settings.i18n_lang.as_ref() == Some(&i18n_lang_id) {
                i18n_manager.switch_i18n_lang(&i18n_lang_id).ok();
            }
        }
    }
    
    // 通知i18n语言扩展加载完成
    i18n_manager.set_extensions_loaded();
} 