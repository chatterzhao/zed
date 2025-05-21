//! 国际化翻译管理工具
//! 
//! # 主要功能
//! 
//! - 验证国际化扩展的 translation.json：检查翻译文件的格式、完整性和正确性
//! - 生成国际化扩展的 translation.json：快速创建新语言的翻译模板（初始值为英文）
//! - 同步翻译：根据默认键自动更新现有翻译
//! - 检查硬编码字符串：扫描整个项目中还没有加翻译键的的硬编码字符串，并生成一个报告，用于检查是否需要国际化
//! 
//! # 快速开始
//! 
//! ```rust
//! use zed_i18n::validation::I18NValidator;
//! 
//! // 1. 验证语言包
//! let mut validator = I18NValidator::new("/path/to/i18n".into());
//! validator.load_reference_keys()?;
//! let report = validator.validate("/path/to/lang_pack".as_ref())?;
//! 
//! // 2. 创建新语言包
//! use zed_i18n::validation::I18NTemplate;
//! let template = I18NTemplate::new(
//!     "/path/to/output".into(),
//!     "fr".to_string(),      // 语言代码
//!     "French".to_string(),  // 语言名称
//! );
//! template.generate()?;
//! 
//! // 3. 同步翻译文件
//! validator.reorganize_translations("/path/to/translation.json".as_ref())?;
//! 
//! // 4. 检查硬编码文本
//! let findings = validator.scan_hardcoded(vec!["/path/to/src".into()])?;
//! ```
//! 
//! # 常用命令
//! 
//! ```bash
//! # 验证语言包
//! cargo run --bin zed-i18n-validate /path/to/lang-pack
//! 
//! # 创建新语言包
//! cargo run --bin zed-i18n-new fr French
//! ```
//! 
//! # 注意事项
//! 
//! - 翻译键必须以 i18n. 开头
//! - 必须使用 UTF-8 编码
//! - 提交前请运行验证检查

use anyhow::{anyhow, Result, Context};
use std::{
    collections::{HashMap, BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
    fmt::Write,
};
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use walkdir::WalkDir;
use regex::Regex;
use crate::core::{I18nManager, I18nLangMeta};

/// 语言包验证工具
pub struct I18NValidator {
    base_dir: PathBuf,
    reference_keys: HashSet<String>,
}

impl I18NValidator {
    pub fn new(base_dir: PathBuf) -> Self {
        Self {
            base_dir,
            reference_keys: HashSet::new(),
        }
    }

    /// 从 defaults.rs 加载默认翻译键
    pub fn load_reference_keys(&mut self) -> Result<()> {
        let defaults_path = self.base_dir.join("crates/i18n/core/defaults.rs");
        let content = fs::read_to_string(&defaults_path)?;
        
        // 解析 DEFAULT_TEXTS 中的键值对
        let mut keys = HashSet::new();
        for line in content.lines() {
            if line.contains("texts.insert") {
                if let Some(captures) = Regex::new(r#"texts\.insert\("([^"]+)",\s*"([^"]+)"\)"#)
                    .unwrap()
                    .captures(line) {
                    if let (Some(key), Some(value)) = (captures.get(1), captures.get(2)) {
                        keys.insert(key.as_str().to_string());
                    }
                }
            }
        }
        
        self.reference_keys = keys;
        Ok(())
    }

    /// 根据 defaults.rs 生成新的 translation.json
    pub fn generate_translation_json(&self, output_path: &Path) -> Result<()> {
        // 确保父目录存在
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // 读取现有的翻译（如果存在）
        let mut json = if output_path.exists() {
            let content = fs::read_to_string(output_path)?;
            serde_json::from_str(&content)?
        } else {
            serde_json::Map::new()
        };
        
        // 添加或更新键，使用defaults.rs中的英文作为默认值
        for key in &self.reference_keys {
            // 如果键不存在，则添加英文默认值
            if !json.contains_key(key) {
                json.insert(key.clone(), serde_json::Value::String(key.clone()));
            }
        }

        // 格式化并写入文件
        let formatted = serde_json::to_string_pretty(&json)?;
        fs::write(output_path, formatted)?;
        Ok(())
    }

    /// 验证翻译文件
    pub fn validate(&self) -> Result<ValidationReport> {
        // 加载默认文本键
        self.load_default_keys()?;
        
        // 加载翻译文件
        let translations = self.load_translations()?;
        
        // 验证翻译
        let mut report = ValidationReport {
            missing_keys: Vec::new(),
            extra_keys: Vec::new(),
            format_errors: Vec::new(),
            lang_id: translations.lang_id.clone(),
        };

        // 检查缺失的键
        for key in &self.reference_keys {
            if !translations.translations.contains_key(key) {
                report.missing_keys.push(key.clone());
            }
        }

        // 检查多余的键
        for key in translations.translations.keys() {
            if !self.reference_keys.contains(key) {
                report.extra_keys.push(key.clone());
            }
        }

        // 检查格式化错误
        for (key, value) in &translations.translations {
            if let Err(e) = self.validate_format(key, value) {
                report.format_errors.push(FormatError {
                    key: key.clone(),
                    error: e.to_string(),
                });
            }
        }

        Ok(report)
    }

    fn load_default_keys(&mut self) -> Result<()> {
        // 从 I18nManager 获取所有默认文本键
        let manager = I18nManager::global();
        for key in manager.get_all_default_text_keys() {
            self.reference_keys.insert(key.to_string());
        }
        Ok(())
    }

    fn load_translations(&self) -> Result<TranslationResource> {
        let content = std::fs::read_to_string(&self.base_dir.join("resources/translations/translation.json"))
            .context("Failed to read translation file")?;
        
        serde_json::from_str(&content)
            .context("Failed to parse translation file")
    }

    fn validate_format(&self, key: &str, value: &str) -> Result<()> {
        // 检查占位符格式
        let default_value = I18nManager::global()
            .get_default_text(key)
            .ok_or_else(|| anyhow!("No default text found for key: {}", key))?;

        // 检查占位符数量是否匹配
        let default_placeholders = self.count_placeholders(default_value);
        let value_placeholders = self.count_placeholders(value);

        if default_placeholders != value_placeholders {
            return Err(anyhow!(
                "Placeholder count mismatch: expected {}, got {}",
                default_placeholders,
                value_placeholders
            ));
        }

        Ok(())
    }

    fn count_placeholders(&self, text: &str) -> usize {
        text.matches("{").count()
    }

    /// 扫描项目中的硬编码字符串
    pub fn scan_hardcoded(&self, paths: Vec<PathBuf>) -> Result<Vec<HardcodedString>> {
        let mut scanner = CodeScanner::new(paths);
        scanner.scan()?;
        Ok(scanner.get_findings().to_vec())
    }

    /// 根据 defaults.rs 重组翻译文件
    /// 
    /// 改进功能:
    /// - 按翻译类别分组和排序(菜单、编辑器、扩展、面板等)  
    /// - 保持现有翻译,只添加缺失的键
    /// - 高效处理大量数据
    pub fn reorganize_translations(&self, translation_file: &Path) -> Result<()> {
        // 确保已加载参考键
        if self.reference_keys.is_empty() {
            return Err(anyhow!("未加载参考键,请先调用load_reference_keys"));
        }

        // 读取当前翻译文件
        let content = fs::read_to_string(translation_file)?;
        let current: HashMap<String, String> = serde_json::from_str(&content)?;

        // 初始化分类
        let mut categorized: BTreeMap<TranslationCategory, BTreeMap<String, String>> = BTreeMap::new();
        let mut others = BTreeMap::new(); // 未分类的键值对
        let mut extra_keys = BTreeMap::new(); // 在翻译中有但defaults.rs中没有的键

        // 1. 根据defaults.rs中的键进行分类
        for key in &self.reference_keys {
            if let Some(category) = self.get_translation_category(key) {
                let translations = categorized.entry(category).or_insert_with(BTreeMap::new);
                // 如果翻译文件中有该键就使用翻译值,否则使用英文默认值
                let translation = current.get(key).cloned().unwrap_or_else(|| key.clone());
                translations.insert(key.clone(), translation);
            } else {
                let translation = current.get(key).cloned().unwrap_or_else(|| key.clone());
                others.insert(key.clone(), translation);
            }
        }

        // 2. 处理额外的键(在翻译文件中有但defaults.rs中没有的)
        for (key, value) in current.iter() {
            if !self.reference_keys.contains_key(key) {
                extra_keys.insert(key.clone(), value.clone());
                println!("警告: 发现额外的翻译键: {}", key);
            }
        }

        // 3. 生成最终结果
        let mut result = String::new();
        result.push_str("{\n");

        // 添加顶部菜单翻译
        if let Some(menu_translations) = categorized.get(&TranslationCategory::TopMenuBar) {
            result.push_str("    // 顶部菜单翻译\n");
            self.write_json_section(&mut result, menu_translations, 1)?;
        }

        // 添加编辑器翻译
        if let Some(editor_translations) = categorized.get(&TranslationCategory::Editor) {
            result.push_str("\n    // 编辑器翻译\n");
            self.write_json_section(&mut result, editor_translations, 1)?;
        }

        // 添加扩展翻译
        if let Some(ext_translations) = categorized.get(&TranslationCategory::Extension) {
            result.push_str("\n    // 扩展翻译\n");
            self.write_json_section(&mut result, ext_translations, 1)?;
        }

        // 添加面板翻译
        let dock_panels: Vec<_> = categorized.iter()
            .filter(|(k, _)| matches!(k, TranslationCategory::DockPanels(_)))
            .collect();
        
        if !dock_panels.is_empty() {
            result.push_str("\n    // 面板翻译\n");
            for (_, translations) in dock_panels {
                self.write_json_section(&mut result, translations, 1)?;
            }
        }

        // 添加其他翻译
        if !others.is_empty() {
            result.push_str("\n    // 其他翻译\n");
            self.write_json_section(&mut result, &others, 1)?;
        }

        // 添加额外的键
        if !extra_keys.is_empty() {
            result.push_str("\n    // 未在defaults.rs中定义的额外键\n");
            self.write_json_section(&mut result, &extra_keys, 1)?;
        }

        result.push_str("}\n");

        // 写入文件
        fs::write(translation_file, result)?;
        Ok(())
    }

    /// 获取翻译键的类别
    fn get_translation_category(&self, key: &str) -> Option<TranslationCategory> {
        if key.starts_with("i18n.top_menu_bar.") {
            Some(TranslationCategory::TopMenuBar)
        } else if key.starts_with("i18n.editor.") {
            Some(TranslationCategory::Editor)
        } else if key.starts_with("i18n.extension.") {
            Some(TranslationCategory::Extension)
        } else if key.starts_with("i18n.dock_panels.") {
            if let Some(panel) = key.split('.').nth(2) {
                DockPanelType::from_str(panel).map(TranslationCategory::DockPanels)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 写入JSON部分
    fn write_json_section(&self, result: &mut String, translations: &BTreeMap<String, String>, depth: usize) -> Result<()> {
        let indent = "    ".repeat(depth);
        let last_index = translations.len() - 1;
        
        for (i, (key, value)) in translations.iter().enumerate() {
            writeln!(
                result,
                r#"{}"{key}": "{value}"{}"#,
                indent,
                if i == last_index { "" } else { "," }
            )?;
        }
        
        Ok(())
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ValidationReport {
    pub missing_keys: Vec<String>,
    pub extra_keys: Vec<String>,
    pub format_errors: Vec<FormatError>,
    pub lang_id: String,
}

impl ValidationReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn merge(&mut self, other: ValidationReport) {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn get_errors(&self) -> &[String] {
        &self.errors
    }

    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }
}

/// 扫描项目中的硬编码字符串
#[derive(Debug)]
pub struct CodeScanner {
    source_paths: Vec<PathBuf>,
    // 忽略的文件和目录
    ignore_patterns: Vec<Regex>,
    // 找到的硬编码字符串
    findings: Vec<HardcodedString>,
}

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum TranslationCategory {
    TopMenuBar,
    Editor,
    Extension,
    DockPanels(DockPanelType),
}

/// Dock面板类型
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum DockPanelType {
    Project,
    Git,
    Outline,
    Collab,
    ProjectSearch,
    ProjectDiagnostics,
    EditPredictions,
    Terminal,
    Agent,
    Notifications,
}

impl DockPanelType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "project" => Some(Self::Project),
            "git" => Some(Self::Git),
            "outline" => Some(Self::Outline),
            "collab" => Some(Self::Collab),
            "project_search" => Some(Self::ProjectSearch),
            "project_diagnostics" => Some(Self::ProjectDiagnostics),
            "edit_predictions" => Some(Self::EditPredictions),
            "terminal" => Some(Self::Terminal),
            "agent" => Some(Self::Agent),
            "notifications" => Some(Self::Notifications),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Git => "git", 
            Self::Outline => "outline",
            Self::Collab => "collab",
            Self::ProjectSearch => "project_search",
            Self::ProjectDiagnostics => "project_diagnostics",
            Self::EditPredictions => "edit_predictions",
            Self::Terminal => "terminal",
            Self::Agent => "agent",
            Self::Notifications => "notifications",
        }
    }
}

#[derive(Debug, Clone)]
pub struct HardcodedString {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub content: String,
    pub context: String,
}

impl CodeScanner {
    pub fn new(source_paths: Vec<PathBuf>) -> Self {
        Self {
            source_paths,
            ignore_patterns: vec![
                Regex::new(r"/target/").unwrap(),
                Regex::new(r"/\.git/").unwrap(),
                Regex::new(r".*\.(json|md|txt)$").unwrap(),
            ],
            findings: Vec::new(),
        }
    }

    /// 扫描项目中的硬编码字符串
    pub fn scan(&mut self) -> Result<()> {
        let paths = self.source_paths.clone();
        for path in &paths {
            self.scan_directory(path)?;
        }
        Ok(())
    }

    fn scan_directory(&mut self, path: &Path) -> Result<()> {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let path = entry.path();

            // 检查是否需要忽略
            if self.should_ignore(path) {
                continue;
            }

            if path.is_file() {
                if let Ok(content) = fs::read_to_string(path) {
                    self.scan_file(path.to_path_buf(), &content)?;
                }
            }
        }
        Ok(())
    }

    fn should_ignore(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.ignore_patterns.iter().any(|pattern| pattern.is_match(&path_str))
    }

    fn scan_file(&mut self, file_path: PathBuf, content: &str) -> Result<()> {
        for (i, line) in content.lines().enumerate() {
            if let Some(finding) = self.check_line(line) {
                self.findings.push(HardcodedString {
                    file_path: file_path.clone(),
                    line_number: i + 1,
                    content: finding,
                    context: self.extract_context(content, i),
                });
            }
        }
        Ok(())
    }

    fn check_line(&self, line: &str) -> Option<String> {
        // 跳过注释行
        if line.trim_start().starts_with("//") {
            return None;
        }

        // 1. 如果已经使用了t!宏，跳过
        if line.contains("t!(") {
            return None;
        }

        // 2. 检查可能需要国际化的情况:
        let should_check = 
            // 包含中文字符
            line.chars().any(|c| c.is_chinese()) ||
            // 包含引号包裹的UI文本(超过一个单词)
            (line.contains('"') && line.split_whitespace().count() > 1) ||
            // 包含常见UI文本模式
            line.contains("label:") ||
            line.contains("title:") ||
            line.contains("message:") ||
            line.contains("tooltip:") ||
            // 检查硬编码的错误消息
            line.contains("Error:") ||
            line.contains("Warning:") ||
            // 菜单相关
            line.contains("MenuItem::") ||
            line.contains("Menu {");

        if should_check {
            Some(line.trim().to_string())
        } else {
            None
        }
    }

    fn extract_context(&self, content: &str, line_number: usize) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let start = line_number.saturating_sub(2);
        let end = (line_number + 3).min(lines.len());
        lines[start..end].join("\n")
    }

    pub fn get_findings(&self) -> &[HardcodedString] {
        &self.findings
    }
}

trait IsChineseChar {
    fn is_chinese(&self) -> bool;
}

impl IsChineseChar for char {
    fn is_chinese(&self) -> bool {
        matches!(self, '\u{4E00}'..='\u{9FFF}')
    }
}

/// JSON 重组工具
#[derive(Debug)]
pub struct JsonReorganizer {
    reference: Map<String, Value>,
    current: Map<String, Value>,
}

impl JsonReorganizer {
    pub fn new(reference: Value, current: Value) -> Result<Self> {
        let reference = reference.as_object()
            .ok_or_else(|| anyhow!("Reference must be an object"))?
            .clone();
        let current = current.as_object()
            .ok_or_else(|| anyhow!("Current must be an object"))?
            .clone();

        Ok(Self { reference, current })
    }

    /// 重组 JSON，保持引用键的顺序
    pub fn reorganize(&self) -> Result<Value> {
        let mut result = BTreeMap::new();
        
        // 从引用文件中获取所有键，确保保持顺序
        for key in self.reference.keys() {
            if let Some(curr_value) = self.current.get(key) {
                result.insert(key.clone(), curr_value.clone());
            } else {
                result.insert(key.clone(), self.reference[key].clone());
            }
        }
        
        // 添加当前文件中的额外键（如果有）
        for key in self.current.keys() {
            if !self.reference.contains_key(key) {
                result.insert(key.clone(), self.current[key].clone());
            }
        }

        // 将有序映射转换为 JSON 对象
        Ok(Value::Object(result.into_iter().collect()))
    }
}

/// 语言包模板生成器
pub struct I18NTemplate {
    target_dir: PathBuf,
    lang_id: String,
    lang_name: String,
}

#[derive(Debug, Serialize)]
struct ExtensionManifest {
    id: String,
    name: String,
    description: String,
    version: String,
    schema_version: i32,
    authors: Vec<String>,
}

impl I18NTemplate {
    pub fn new(target_dir: PathBuf, lang_id: String, lang_name: String) -> Self {
        Self {
            target_dir,
            lang_id,
            lang_name,
        }
    }

    /// 生成语言包模板
    pub fn generate(&self) -> Result<()> {
        // 创建目录结构
        self.create_directories()?;
        
        // 生成配置文件
        self.generate_extension_toml()?;
        
        // 生成 Cargo.toml
        self.generate_cargo_toml()?;
        
        // 生成源代码
        self.generate_source_code()?;
        
        // 生成空的翻译文件
        self.generate_translation_files()?;
        
        // 生成 README.md
        self.generate_readme()?;

        Ok(())
    }

    fn create_directories(&self) -> Result<()> {
        let dirs = [
            "",
            "src",
            "resources",
            "resources/translations",
        ];

        for dir in dirs.iter() {
            fs::create_dir_all(self.target_dir.join(dir))?;
        }

        Ok(())
    }

    fn generate_extension_toml(&self) -> Result<()> {
        let manifest = ExtensionManifest {
            id: format!("i18n-{}", self.lang_id),
            name: format!("i18n: {} ({})", self.lang_name, self.lang_id.to_uppercase()),
            description: format!("Zed Editor {} language support", self.lang_name),
            version: "0.1.0".to_string(),
            schema_version: 1,
            authors: vec!["Zed Contributors".to_string()],
        };

        let content = toml::to_string_pretty(&manifest)?;
        fs::write(self.target_dir.join("extension.toml"), content)?;
        
        Ok(())
    }

    fn generate_cargo_toml(&self) -> Result<()> {
        let content = format!(
            r#"[package]
name = "zed-i18n-{}"
version = "0.1.0"
edition = "2021"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
anyhow = "1.0"
async-trait = "0.1"
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
zed-extension-api = {{ path = "../../crates/extension_api" }}"#,
            self.lang_id
        );

        fs::write(self.target_dir.join("Cargo.toml"), content)?;
        
        Ok(())
    }

    fn generate_source_code(&self) -> Result<()> {
        let content = format!(
            r#"use std::{{collections::HashMap, path::PathBuf, sync::{{Arc, RwLock}}}};
use anyhow::Result;
use async_trait::async_trait;
use serde::{{Deserialize, Serialize}};
use zed_extension_api as zed;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
struct TranslationData {{
    #[serde(flatten)]
    translations: HashMap<String, String>,  // 扁平化的键值对存储所有翻译
}}

struct I18nExtension {{
    translations: RwLock<Option<TranslationData>>,
    work_dir: PathBuf,
}}

impl I18nExtension {{
    fn new() -> Self {{
        Self {{
            translations: RwLock::new(None),
            work_dir: std::env::current_dir().unwrap_or_default(),
        }}
    }}
}}

#[no_mangle]
pub fn init_extension() -> Result<Box<dyn zed::Extension>> {{
    Ok(Box::new(I18nExtension::new()))
}}
"#
        );

        fs::write(self.target_dir.join("src/lib.rs"), content)?;
        
        Ok(())
    }

    fn generate_translation_files(&self) -> Result<()> {
        let translations_dir = self.target_dir.join("resources/translations");
        
        // 创建带有示例翻译的模板
        let template = serde_json::json!({
            "i18n.top_menu_bar.file": "",
            "i18n.top_menu_bar.file.new": "",
            "i18n.top_menu_bar.file.open": "",
            "i18n.editor.welcome": "",
            "i18n.editor.context_menu.cut": "",
            "i18n.extension.install": "",
            "i18n.extension.uninstall": "",
            "i18n.dock_panels.project.new_folder": "",
            "i18n.dock_panels.git.commit": "",
            "i18n.dock_panels.outline.title": "",
        });

        let content = serde_json::to_string_pretty(&template)?;
        fs::write(translations_dir.join("translation.json"), content)?;
        Ok(())
    }

    fn generate_readme(&self) -> Result<()> {
        let content = format!(
            r#"# Zed Editor {} Language Pack

This extension provides {} language support for Zed Editor.

## Development

1. Edit translation file `resources/translations/translation.json`.
   Translation keys follow this format:
   - Top Menu Bar: `i18n.top_menu_bar.[menu].[item]`
   - Editor: `i18n.editor.[feature].[element]`
   - Extension: `i18n.extension.[action]`
   - Dock Panels: `i18n.dock_panels.[panel].[action]`

2. Build the extension:
   ```
   cargo build
   ```

3. Test the extension:
   ```
   cargo test
   ```

## Contributing

Please feel free to submit issues and pull requests to improve the translations.

## License

This extension is released under the same license as Zed Editor.
"#,
            self.lang_name,
            self.lang_name
        );

        fs::write(self.target_dir.join("README.md"), content)?;
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatError {
    pub key: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TranslationResource {
    lang_id: String,
    translations: HashMap<String, String>,
}

impl I18NValidator {
    pub fn validate_language_pack(
        lang_id: &str,
        extension_path: PathBuf,
    ) -> Result<ValidationReport> {
        let translation_path = extension_path
            .join("resources")
            .join("translations")
            .join("translation.json");

        let validator = Self::new(translation_path);
        validator.validate()
    }
}
