use crate::defaults::{get_default_text, get_all_default_text_keys};
use anyhow::{Result, Context, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, RwLock},
    num::NonZeroUsize,
};
use lru::LruCache;
use futures::future::BoxFuture;
use gpui::{BackgroundExecutor, Subscription, Global};
use fs::Fs;
use parking_lot::RwLock as ParkingRwLock;

/// 翻译资源管理器
#[derive(Clone)]
pub struct I18nManager {
    state: Arc<ParkingRwLock<I18nState>>,
    fs: Arc<dyn Fs>,
    executor: BackgroundExecutor,
    _subscriptions: Vec<Arc<Subscription>>,
}

#[derive(Debug, Clone)]
pub struct I18nState {
    pub current_lang: String,
    pub resources: HashMap<String, HashMap<String, String>>,
    pub translation_cache: LruCache<String, String>,
}

impl Default for I18nState {
    fn default() -> Self {
        Self {
            current_lang: "en-US".to_string(),
            resources: HashMap::new(),
            translation_cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        }
    }
}

/// 单个语言的翻译资源
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct I18nLangResources {
    #[serde(flatten)]
    translations: HashMap<String, String>,  // 扁平化存储所有翻译
    #[serde(default)]
    rtl: bool, // 是否是从右到左的语言
}

impl Global for I18nManager {}

impl I18nManager {
    pub fn new(executor: BackgroundExecutor, fs: Arc<dyn Fs>) -> Self {
        Self {
            state: Arc::new(ParkingRwLock::new(I18nState::default())),
            fs,
            executor,
            _subscriptions: Vec::new(),
        }
    }

    /// 注册默认英文文本
    pub fn register_default_texts(&self) {
        let mut state = self.state.write();
        let mut default_resources = I18nLangResources::default();
        
        // 从 defaults.rs 加载所有默认文本
        for key in get_all_default_text_keys() {
            if let Some(text) = get_default_text(key) {
                default_resources.translations.insert(key.to_string(), text.to_string());
            }
        }
        
        state.resources.insert("en-US".to_string(), default_resources.translations);
    }

    /// 注册一个i18n语言扩展
    pub fn register_i18n_lang_extension(&self, lang_id: &str, extension_path: PathBuf) -> Result<()> {
        let translations_file = extension_path.join("translations.json");
        let content = self.fs.read_to_string(&translations_file)?;
        let translations: HashMap<String, String> = serde_json::from_str(&content)?;

        let mut state = self.state.write();
        state.resources.insert(lang_id.to_string(), translations);
        Ok(())
    }

    /// 获取翻译文本
    pub fn get_text(&self, key: &str) -> Option<String> {
        // 首先检查缓存
        {
            let state = self.state.read();
            if let Some(cached) = state.translation_cache.get(&key.to_string()) {
                return Some(cached.clone());
            }
        }

        // 获取当前语言的翻译
        let mut state = self.state.write();
        let current_lang = state.current_lang.clone();
        
        if let Some(resources) = state.resources.get(&current_lang) {
            if let Some(text) = resources.get(key) {
                // 更新缓存
                let mut state = self.state.write();
                let mut state = self.state.write();
                state.translation_cache.put(key.to_string(), text.clone());
                return Some(text.clone());
            }
        }

        // 如果当前语言没有翻译，尝试使用备用语言
        if let Some(fallback_lang) = state.resources.keys().find(|&lang| lang != &current_lang) {
            if let Some(resources) = state.resources.get(fallback_lang) {
                if let Some(text) = resources.get(key) {
                    // 更新缓存
                    let mut state = self.state.write();
                    state.translation_cache.put(key.to_string(), text.clone());
                    return Some(text.clone());
                }
            }
        }

        None
    }

    /// 格式化带参数的翻译文本
    pub fn format_text(&self, key: &str, params: &[(&str, &str)]) -> Option<String> {
        let text = self.get_text(key)?;
        let mut result = text;
        for (key, value) in params {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        Some(result)
    }

    /// 检查语言是否是RTL
    pub fn is_rtl(&self) -> bool {
        let state = self.state.read();
        state.current_lang == "ar" || state.current_lang == "he"
    }

    /// 通知UI需要刷新
    fn notify_i18n_lang_changed(&self) {
        // TODO: 实现具体的通知逻辑
        // 比如发送 LanguageChanged 事件
    }
}