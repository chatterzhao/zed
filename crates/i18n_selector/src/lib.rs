use gpui::{
    div, prelude::*, AppContext, Entity, Model, Picker, PickerDelegate, Render, SharedString,
    Subscription, View, Window,
};
use std::sync::Arc;
use crate::core::{I18nManager, I18nSettings, I18nLangMeta};

pub struct I18nLangSelector {
    picker: Entity<Picker<I18nLangSelectorDelegate>>,
}

struct I18nLangSelectorDelegate {
    i18n_langs: Vec<I18nLangMeta>,
    matches: Vec<StringMatch>,
    original_i18n_lang: String,
    selected_i18n_lang: Option<String>,
    selected_index: usize,
}

impl I18nLangSelector {
    pub fn new(cx: &mut AppContext) -> Self {
        let i18n_langs = I18nSettings::get_available_i18n_langs(cx);
        let current_lang = I18nSettings::get_active_i18n_lang(cx)
            .unwrap_or_else(|| "en-US".to_string());

        let delegate = I18nLangSelectorDelegate {
            i18n_langs: i18n_langs.into_values().collect(),
            matches: Vec::new(),
            original_i18n_lang: current_lang.clone(),
            selected_i18n_lang: Some(current_lang),
            selected_index: 0,
        };

        let picker = Picker::new(delegate, cx);
        Self { picker }
    }
}

impl Render for I18nLangSelector {
    fn render(&mut self, _window: &mut Window, cx: &mut AppContext) -> impl IntoElement {
        div()
            .child(self.picker.clone())
    }
}

impl PickerDelegate for I18nLangSelectorDelegate {
    fn placeholder_text(&self) -> SharedString {
        "选择语言...".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
        if let Some(match_) = self.matches.get(index) {
            self.selected_i18n_lang = Some(match_.text.clone());
        }
    }

    fn update_matches(&mut self, query: String, cx: &mut AppContext) {
        self.matches = self.i18n_langs
            .iter()
            .filter(|lang| {
                lang.name.to_lowercase().contains(&query.to_lowercase()) ||
                lang.display_name.to_lowercase().contains(&query.to_lowercase())
            })
            .map(|lang| StringMatch {
                text: lang.id.clone(),
                positions: Vec::new(),
            })
            .collect();

        if !self.matches.is_empty() {
            self.selected_index = 0;
            self.selected_i18n_lang = Some(self.matches[0].text.clone());
        }
    }

    fn confirm_selection(&mut self, cx: &mut AppContext) {
        if let Some(i18n_lang_id) = self.selected_i18n_lang.clone() {
            // 更新设置
            I18nSettings::set_i18n_lang(Some(i18n_lang_id.clone()), cx);
            I18nSettings::set_auto_detect_system_i18n_lang(false, cx);
            
            // 切换语言
            if let Some(manager) = I18nManager::global(cx) {
                manager.switch_i18n_lang(&i18n_lang_id).ok();
            }
        }
    }

    fn render_match(&self, index: usize) -> impl IntoElement {
        if let Some(match_) = self.matches.get(index) {
            if let Some(lang) = self.i18n_langs.iter().find(|l| l.id == match_.text) {
                div()
                    .child(lang.display_name.clone())
            } else {
                div().child(match_.text.clone())
            }
        } else {
            div()
        }
    }
}

#[derive(Clone)]
struct StringMatch {
    text: String,
    positions: Vec<usize>,
} 