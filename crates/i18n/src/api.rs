use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

/// 国际化文本键的前缀
pub const I18N_PREFIX: &str = "i18n.";

/// 宏定义，用于创建i18n文本键
#[macro_export]
macro_rules! i18n_key {
    ($path:expr) => {
        format!("{}{}", $crate::I18N_PREFIX, $path)
    };
}

/// 宏定义，用于创建i18n文本
#[macro_export]
macro_rules! i18n {
    ($key:expr) => {
        $crate::I18nManager::global(cx).translate($key, cx)
    };
}

/// 翻译资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationResource {
    /// 语言标识
    pub lang_id: String,
    /// 翻译文本映射
    pub translations: HashMap<String, String>,
}

/// 翻译错误
#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("翻译资源未找到")]
    ResourceNotFound,
    #[error("翻译键未找到: {0}")]
    KeyNotFound(String),
    #[error("无效的翻译键格式: {0}")]
    InvalidKeyFormat(String),
}

/// 国际化管理器的公共API
pub trait I18nManagerAPI {
    /// 获取当前激活的语言
    fn get_active_lang(&self) -> String;
    
    /// 设置当前语言
    fn set_lang(&mut self, lang_id: &str) -> Result<()>;
    
    /// 翻译文本
    fn translate(&self, key: &str) -> String;
    
    /// 添加翻译资源
    fn add_translation_resource(&mut self, resource: TranslationResource) -> Result<()>;
    
    /// 删除翻译资源
    fn remove_translation_resource(&mut self, lang_id: &str) -> Result<()>;
    
    /// 获取所有可用语言
    fn get_available_langs(&self) -> Vec<String>;
    
    /// 获取语言名称
    fn get_lang_name(&self, lang_id: &str) -> Option<String>;
}
