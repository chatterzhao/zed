use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use zed_i18n_tools::{I18NValidator, HardcodedString};

/// 从一行文本中提取双引号中的内容
fn extract_text(line: &str) -> Option<String> {
    // 1. 提取双引号中的文本
    if let Some(start) = line.find('"') {
        if let Some(end) = line[start + 1..].find('"') {
            let text = &line[start + 1..start + 1 + end];
            // 2. 只排除 URL
            if !text.is_empty() && !text.contains("http://") && !text.contains("https://") {
                return Some(text.to_string());
            }
        }
    }
    None
}

/// 规范化文本为键名
fn normalize_key(text: &str) -> String {
    let normalized = text.trim()
        .to_lowercase()
        .replace([' ', '-'], "_")
        .replace(['…', '.', ':', '?', '!', '"', ',', '/', '\\'], "");
    
    normalized.trim_matches('_').to_string()
}

/// 生成键名
fn generate_key(menu: &str, text: &str) -> String {
    let menu_key = normalize_key(menu);
    let item_key = normalize_key(text);
    format!("i18n.top_menu_bar.{}.{}", menu_key, item_key)
}

/// 处理菜单文本
fn process_menu_text(content: &str) -> Option<(String, String)> {
    if let Some(text) = extract_text(content) {
        if content.contains("Menu {") && content.contains("name:") {
            // 主菜单
            let key = format!("i18n.top_menu_bar.{}", normalize_key(&text));
            Some((key, text))
        } else {
            // 子菜单和菜单项
            Some((format!("i18n.top_menu_bar.unknown.{}", normalize_key(&text)), text))
        }
    } else {
        None
    }
}

/// 扫描硬编码字符串并生成defaults.rs文件
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("用法: cargo run --bin i18n-scan <源代码目录或文件> [输出文件路径]");
        println!("例如: cargo run --bin i18n-scan src/ crates/i18n/core/defaults.rs");
        println!("     cargo run --bin i18n-scan src/app_menus.rs crates/i18n/core/defaults.rs");
        return Ok(());
    }
    
    // 获取源代码路径
    let source_path = PathBuf::from(&args[1]);
    if !source_path.exists() {
        return Err(anyhow!("源代码路径不存在: {}", source_path.display()));
    }
    
    // 获取输出文件路径，默认为当前目录下的defaults.rs
    let output_path = if args.len() > 2 {
        PathBuf::from(&args[2])
    } else {
        let mut default_path = PathBuf::from(env::current_dir()?);
        default_path.push("defaults.rs");
        default_path
    };
    
    // 添加需要扫描的模块类别
    let scan_categories = vec![
        "top_menu_bar",
        "editor",
        "extension",
        "dock_panels"
    ];
    
    // 创建验证器
    let validator = I18NValidator::new(PathBuf::from("."));
    
    // 根据是文件还是目录进行不同的扫描
    let scan_paths = if source_path.is_file() {
        vec![source_path.clone()]
    } else {
        vec![source_path.clone()]
    };
    
    // 扫描硬编码字符串
    println!("正在扫描路径: {}", source_path.display());
    let findings = validator.scan_hardcoded(scan_paths)?;
    println!("找到 {} 个可能需要国际化的字符串", findings.len());
    
    // 生成defaults.rs文件
    generate_defaults_file(&findings, &output_path)?;
    
    Ok(())
}

/// 从硬编码字符串生成defaults.rs文件
fn generate_defaults_file(findings: &[HardcodedString], output_path: &Path) -> Result<()> {
    // 确保输出目录存在
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let mut texts = HashMap::new();
    // 当前处理的菜单名
    let mut current_menu = String::new();
    
    for finding in findings {
        if let Some((key, text)) = process_menu_text(&finding.content) {
            if finding.content.contains("Menu {") && finding.content.contains("name:") {
                current_menu = text.clone();
            } else if !current_menu.is_empty() {
                let key = generate_key(&current_menu, &text);
                texts.insert(key, text);
            } else {
                texts.insert(key, text);
            }
        }
    }
    
    // 生成 defaults.rs 文件内容
    let mut content = String::new();
    content.push_str("use std::collections::HashMap;\n");
    content.push_str("use once_cell::sync::Lazy;\n\n");
    
    content.push_str("// 全局静态默认文本映射\n");
    content.push_str("static DEFAULT_TEXTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {\n");
    content.push_str("    let mut texts = HashMap::new();\n\n");
    content.push_str("    // 所有键使用扁平化的命名方式\n");
    
    for (key, value) in &texts {
        content.push_str(&format!("    texts.insert(\"{}\", \"{}\");\n", key, value));
    }
    
    content.push_str("\n    texts\n");
    content.push_str("});\n\n");
    
    // 添加辅助函数
    content.push_str("/// 获取默认文本\n");
    content.push_str("pub fn get_default_text(key: &str) -> Option<&'static str> {\n");
    content.push_str("    DEFAULT_TEXTS.get(key).copied()\n");
    content.push_str("}\n\n");
    
    content.push_str("/// 获取所有默认文本键\n");
    content.push_str("pub fn get_all_default_text_keys() -> impl Iterator<Item = &'static str> {\n");
    content.push_str("    DEFAULT_TEXTS.keys().copied()\n");
    content.push_str("}\n");
    
    // 写入文件
    fs::write(output_path, content)?;
    
    Ok(())
}