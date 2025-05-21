use anyhow::Result;
use gpui::App;
use fs::Fs;
use settings::{Settings, SettingsSources};
use crate::settings::I18nSettings;
use crate::manager::I18nManager;
use std::sync::Arc;

/// 初始化i18n系统
pub fn init_i18n_system(cx: &mut App) -> Result<()> {
    // 初始化 i18n 管理器
    let fs = cx.fs().clone();
    let executor = cx.background_executor().clone();
    let manager = I18nManager::new(executor, fs);
    cx.set_global(manager);

    // 加载设置
    let settings = SettingsSources::json_merge(cx)?;
    I18nSettings::load(settings, cx)?;

    // 初始化默认语言
    init_default_lang(cx)?;

    Ok(())
}

/// 初始化默认语言
pub fn init_default_lang(cx: &mut App) -> Result<()> {
    let settings = I18nSettings::get_global(cx);
    
    // 如果启用了自动检测系统语言
    if settings.auto_detect_system_i18n_lang {
        let system_lang = detect_system_lang();
        I18nSettings::set_i18n_lang(Some(system_lang), cx);
    }

    Ok(())
}

/// 检测系统语言
pub fn detect_system_lang() -> String {
    // 使用 sys-locale 获取系统语言
    match sys_locale::get_locale() {
        Some(lang) => lang,
        None => "en-US".to_string(),
    }
}
