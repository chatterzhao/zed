use serde::{Deserialize, Serialize};
use gpui::{App, BorrowAppContext, Global};
use settings::{Settings, SettingsSources, VsCodeSettings};
use std::collections::HashMap;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct I18nLangMeta {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub extension_id: Option<String>,
    pub rtl: bool,
}

#[derive(Default, Deserialize, Serialize, Clone, JsonSchema)]
pub struct I18nSettings {
    /// 当前选择的i18n语言ID
    pub i18n_lang: Option<String>,
    
    /// 是否自动检测系统i18n语言
    #[serde(default = "default_true")]
    pub auto_detect_system_i18n_lang: bool,

    /// 备用i18n语言(当主i18n语言缺少翻译时使用)
    pub fallback_i18n_lang: Option<String>,

    /// 已安装的i18n语言包信息
    #[serde(skip)]
    pub available_i18n_langs: HashMap<String, I18nLangMeta>,
}

impl Global for I18nSettings {}

fn default_true() -> bool {
    true
}

impl Settings for I18nSettings {
    const KEY: Option<&'static str> = Some("i18n");
    type FileContent = Self;

    fn load(sources: SettingsSources<'_, Self::FileContent>, cx: &mut App) -> anyhow::Result<Self> {
        sources.json_merge()
    }

    fn import_from_vscode(_vscode_settings: &VsCodeSettings, _: &mut Self::FileContent) {
        // TODO: 实现从 VSCode 设置导入
    }
}

impl I18nSettings {
    /// 获取当前激活的i18n语言设置
    pub fn get_active_i18n_lang(cx: &App) -> Option<String> {
        Self::get_global(cx).i18n_lang.clone()
    }

    /// 设置当前i18n语言
    pub fn set_i18n_lang(lang: Option<String>, cx: &mut App) {
        cx.update_default_global::<Self, ()>(|settings, _| {
            settings.i18n_lang = lang;
        });
    }

    /// 获取是否自动检测系统i18n语言
    pub fn get_auto_detect_system_i18n_lang(cx: &App) -> bool {
        Self::get_global(cx).auto_detect_system_i18n_lang
    }

    /// 设置是否自动检测系统i18n语言
    pub fn set_auto_detect_system_i18n_lang(enable: bool, cx: &mut App) {
        cx.update_default_global::<Self, ()>(|settings, _| {
            settings.auto_detect_system_i18n_lang = enable;
        });
    }

    /// 获取备用i18n语言
    pub fn get_fallback_i18n_lang(cx: &App) -> Option<String> {
        Self::get_global(cx).fallback_i18n_lang.clone()
    }

    /// 设置备用i18n语言
    pub fn set_fallback_i18n_lang(lang: Option<String>, cx: &mut App) {
        cx.update_default_global::<Self, ()>(|settings, _| {
            settings.fallback_i18n_lang = lang;
        });
    }

    /// 获取所有可用的i18n语言
    pub fn get_available_i18n_langs(cx: &App) -> HashMap<String, I18nLangMeta> {
        Self::get_global(cx).available_i18n_langs.clone()
    }

    /// 添加可用的i18n语言
    pub fn add_available_i18n_lang(meta: I18nLangMeta, cx: &mut App) {
        cx.update_default_global::<Self, ()>(|settings, _| {
            settings.available_i18n_langs.insert(meta.id.clone(), meta);
        });
    }

    /// 移除可用的i18n语言
    pub fn remove_available_i18n_lang(lang_id: &str, cx: &mut App) {
        cx.update_default_global::<Self, ()>(|settings, _| {
            settings.available_i18n_langs.remove(lang_id);
        });
    }
}