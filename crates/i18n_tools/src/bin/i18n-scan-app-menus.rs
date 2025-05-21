use anyhow::Result;
use linked_hash_map::LinkedHashMap;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use regex;

#[derive(Debug, Clone)]
struct MenuPath {
    path: Vec<String>,  // 完整的菜单路径，例如 ["zed", "settings"]
}

impl MenuPath {
    fn new() -> Self {
        MenuPath { path: Vec::new() }
    }
    
    fn push(&mut self, name: &str) {
        self.path.push(normalize_key(name));
    }
    
    fn pop(&mut self) -> Option<String> {
        self.path.pop()
    }
    
    fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
    
    fn to_key(&self, item_name: &str) -> String {
        if self.is_empty() {
            // 如果路径为空，返回顶级菜单键
            format!("i18n.menu.{}", normalize_key(item_name))
        } else {
            // 否则返回完整路径键
            let path_str = self.path.join(".");
            format!("i18n.menu.{}.{}", path_str, normalize_key(item_name))
        }
    }
    
    fn to_menu_key(&self) -> String {
        if self.is_empty() {
            return "i18n.menu".to_string();
        }
        let path_str = self.path.join(".");
        format!("i18n.menu.{}", path_str)
    }
    
    fn last(&self) -> Option<&String> {
        self.path.last()
    }
}

#[derive(Debug)]
struct ExtractedText {
    text: String,         // 提取的文本
    is_menu: bool,       // 是否是菜单名称
}

fn normalize_key(text: &str) -> String {
    text.to_lowercase()
        .replace("…", "")
        .replace("...", "")
        .replace(" ", "_")
        .replace("-", "_")
        .replace("'", "")
        .replace("&", "")
}

/// 提取菜单文本
fn extract_menu_text(line: &str) -> Option<ExtractedText> {
    // 如果行包含 t!(cx,") 格式宏调用，跳过它
    if line.contains("t!(cx,") {
        return None;
    }

    // 检查是否是菜单名定义
    if line.contains("name:") && line.contains('"') {
        if let Some(start) = line.find('"') {
            if let Some(end) = line[start + 1..].find('"') {
                let text = line[start + 1..start + 1 + end].to_string();
                return Some(ExtractedText {
                    text,
                    is_menu: true,
                });
            }
        }
    }
    
    // 检查是否是菜单项定义
    if line.contains("MenuItem::action") && line.contains('"') {
        if let Some(start) = line.find('"') {
            if let Some(end) = line[start + 1..].find('"') {
                let text = line[start + 1..start + 1 + end].to_string();
                return Some(ExtractedText {
                    text,
                    is_menu: false,
                });
            }
        }
    }
    
    None
}

/// 处理提取的文本，生成 i18n 键值对
fn process_menu_text(line: &str, menu_path: &MenuPath) -> Option<(String, String)> {
    if line.trim().is_empty() || line.contains("#[cfg") || line.contains("separator") {
        return None;
    }

    if let Some(extracted) = extract_menu_text(line) {
        let key = if extracted.is_menu {
            // 菜单名称
            menu_path.to_menu_key()
        } else {
            // 菜单项
            menu_path.to_key(&extracted.text)
        };

        Some((key, extracted.text))
    } else {
        None
    }
}

/// 扫描菜单文件
fn scan_app_menus(file_path: &Path) -> Result<LinkedHashMap<String, String>> {
    let content = fs::read_to_string(file_path)?;
    let mut texts = LinkedHashMap::new();
    
    // 使用正则表达式提取所有引号中的字符串
    let re = regex::Regex::new(r#""([^"]+)""#).unwrap();
    let mut extracted_texts = Vec::new();
    
    for cap in re.captures_iter(&content) {
        let text = cap[1].to_string();
        // 排除空字符串、已国际化的字符串和 URL
        if !text.is_empty() && !text.contains("t!(cx,") && !text.starts_with("https://") {
            extracted_texts.push(text);
        }
    }

    // 解析菜单结构，使用更准确的方法
    let lines: Vec<&str> = content.lines().collect();
    
    // 跟踪当前菜单上下文
    let mut current_menu = String::new();
    let mut current_submenu = String::new();
    let mut in_editor_layout = false;
    
    for i in 0..lines.len() {
        let line = lines[i].trim();
        
        // 检测顶级菜单
        if line.contains("Menu {") {
            // 尝试找到菜单名称
            if i + 1 < lines.len() && lines[i+1].contains("name:") {
                let name_line = lines[i+1].trim();
                if name_line.contains("\"Zed\"") || name_line.contains("\"Zed\".into()") {
                    current_menu = "zed".to_string();
                    texts.insert("i18n.menu.zed".to_string(), "Zed".to_string());
                }
                else if name_line.contains("\"File\"") || name_line.contains("\"File\".into()") {
                    current_menu = "file".to_string();
                    texts.insert("i18n.menu.file".to_string(), "File".to_string());
                }
                else if name_line.contains("\"Edit\"") || name_line.contains("\"Edit\".into()") {
                    current_menu = "edit".to_string();
                    texts.insert("i18n.menu.edit".to_string(), "Edit".to_string());
                }
                else if name_line.contains("\"Selection\"") || name_line.contains("\"Selection\".into()") {
                    current_menu = "selection".to_string();
                    texts.insert("i18n.menu.selection".to_string(), "Selection".to_string());
                }
                else if name_line.contains("\"View\"") || name_line.contains("\"View\".into()") {
                    current_menu = "view".to_string();
                    texts.insert("i18n.menu.view".to_string(), "View".to_string());
                }
                else if name_line.contains("\"Go\"") || name_line.contains("\"Go\".into()") {
                    current_menu = "go".to_string();
                    texts.insert("i18n.menu.go".to_string(), "Go".to_string());
                }
                else if name_line.contains("\"Terminal\"") || name_line.contains("\"Terminal\".into()") {
                    current_menu = "terminal".to_string();
                    texts.insert("i18n.menu.terminal".to_string(), "Terminal".to_string());
                }
                else if name_line.contains("\"Window\"") || name_line.contains("\"Window\".into()") {
                    current_menu = "window".to_string();
                    texts.insert("i18n.menu.window".to_string(), "Window".to_string());
                }
                else if name_line.contains("\"Help\"") || name_line.contains("\"Help\".into()") {
                    current_menu = "help".to_string();
                    texts.insert("i18n.menu.help".to_string(), "Help".to_string());
                }
            }
        }
        
        // 检测子菜单
        else if line.contains("MenuItem::submenu") {
            if i + 2 < lines.len() && lines[i+2].contains("name:") {
                let name_line = lines[i+2].trim();
                if name_line.contains("\"Settings\"") || name_line.contains("\"Settings\".into()") {
                    current_submenu = "settings".to_string();
                    texts.insert("i18n.menu.zed.settings".to_string(), "Settings".to_string());
                }
                else if name_line.contains("\"Services\"") || name_line.contains("\"Services\".into()") {
                    current_submenu = "services".to_string();
                    texts.insert("i18n.menu.zed.services".to_string(), "Services".to_string());
                }
                else if name_line.contains("\"Editor Layout\"") || name_line.contains("\"Editor Layout\".into()") {
                    current_submenu = "editor_layout".to_string();
                    in_editor_layout = true;
                    texts.insert("i18n.menu.view.editor_layout".to_string(), "Editor Layout".to_string());
                }
            }
        }
        
        // 检测子菜单结束
        else if line.contains("}") && line.contains("],") {
            if in_editor_layout {
                in_editor_layout = false;
            }
            current_submenu = "".to_string();
        }
        
        // 检测菜单项
        else if line.contains("MenuItem::action") || line.contains("MenuItem::os_action") {
            // 提取菜单项文本
            let mut item_text = "";
            let mut j = i;
            
            // 处理跨行的菜单项
            while j < lines.len() {
                let current_line = lines[j].trim();
                if current_line.contains("\"") && !current_line.contains("t!(cx,") {
                    // 提取引号中的文本
                    if let Some(start) = current_line.find('"') {
                        if let Some(end) = current_line[start+1..].find('"') {
                            item_text = &current_line[start+1..start+1+end];
                            break;
                        }
                    }
                }
                j += 1;
            }
            
            if !item_text.is_empty() && !item_text.starts_with("https://") {
                let key: String;
                
                // 根据当前菜单上下文生成键名
                if current_menu == "zed" {
                    // 特殊处理 Settings 子菜单项
                    if item_text == "Open Settings" || 
                       item_text == "Open Key Bindings" || 
                       item_text == "Open Default Settings" || 
                       item_text == "Open Default Key Bindings" || 
                       item_text == "Open Project Settings" || 
                       item_text == "Select Theme..." {
                        key = format!("i18n.menu.zed.settings.{}", normalize_key(item_text));
                    } else {
                        key = format!("i18n.menu.zed.{}", normalize_key(item_text));
                    }
                }
                else if current_menu == "file" {
                    key = format!("i18n.menu.file.{}", normalize_key(item_text));
                }
                else if current_menu == "edit" {
                    key = format!("i18n.menu.edit.{}", normalize_key(item_text));
                }
                else if current_menu == "selection" {
                    key = format!("i18n.menu.selection.{}", normalize_key(item_text));
                }
                else if current_menu == "view" {
                    // 特殊处理 Editor Layout 子菜单项
                    if item_text == "Split Up" || 
                       item_text == "Split Down" || 
                       item_text == "Split Left" || 
                       item_text == "Split Right" {
                        key = format!("i18n.menu.view.editor_layout.{}", normalize_key(item_text));
                    } else {
                        key = format!("i18n.menu.view.{}", normalize_key(item_text));
                    }
                }
                else if current_menu == "go" {
                    key = format!("i18n.menu.go.{}", normalize_key(item_text));
                }
                else if current_menu == "terminal" {
                    key = format!("i18n.menu.terminal.{}", normalize_key(item_text));
                }
                else if current_menu == "window" {
                    key = format!("i18n.menu.window.{}", normalize_key(item_text));
                }
                else if current_menu == "help" {
                    key = format!("i18n.menu.help.{}", normalize_key(item_text));
                }
                else {
                    // 如果无法确定菜单上下文，使用通用前缀
                    key = format!("i18n.menu.other.{}", normalize_key(item_text));
                }
                
                texts.insert(key, item_text.to_string());
                
                // 从提取的字符串列表中移除已处理的字符串
                if let Some(pos) = extracted_texts.iter().position(|s| s == item_text) {
                    extracted_texts.remove(pos);
                }
            }
        }
    }
    
    // 处理特殊情况：About Zed 和 Check for Updates
    if !texts.values().any(|v| v == "About Zed…") {
        texts.insert("i18n.menu.zed.about_zed".to_string(), "About Zed…".to_string());
    }
    if !texts.values().any(|v| v == "Check for Updates") {
        texts.insert("i18n.menu.zed.check_for_updates".to_string(), "Check for Updates".to_string());
    }
    
    // 处理其他未匹配的菜单项
    for text in extracted_texts {
        if !texts.values().any(|v| v == &text) {
            // 跳过 URL
            if text.starts_with("https://") {
                continue;
            }
            
            // 检查是否已经是一个键名（以 i18n.menu 开头）
            if text.starts_with("i18n.menu") {
                // 如果已经是键名，直接使用原始文本作为键
                texts.insert(text.clone(), text);
            } else {
                // 对于未能匹配到菜单结构的字符串，使用一个通用前缀
                let key = format!("i18n.menu.other.{}", normalize_key(&text));
                texts.insert(key, text);
            }
        }
    }
    
    Ok(texts)
}

/// 生成 defaults-app-menus.rs 文件
fn generate_defaults_app_menus(texts: &LinkedHashMap<String, String>, output_path: &Path) -> Result<()> {
    let mut content = String::new();
    content.push_str("use std::collections::HashMap;\n");
    content.push_str("use once_cell::sync::Lazy;\n\n");
    
    content.push_str("// 全局静态默认文本映射\n");
    content.push_str("static DEFAULT_TEXTS: Lazy<HashMap<&str, &str>> = Lazy::new(|| {\n");
    content.push_str("    let mut texts = HashMap::new();\n\n");
    
    // 保持源码顺序输出，不进行排序，并排除 URL
    for (key, value) in texts.iter() {
        // 排除 URL
        if value.starts_with("https://") || key.contains("https://") {
            continue;
        }
        
        let formatted_key = key.replace("..", ".");  // 删除双点
        content.push_str(&format!("    texts.insert(\"{}\", \"{}\");\n", formatted_key, value));
    }
    
    content.push_str("\n    texts\n");
    content.push_str("});\n\n");
    
    content.push_str("/// 获取默认文本\n");
    content.push_str("pub fn get_default_text(key: &str) -> Option<&'static str> {\n");
    content.push_str("    DEFAULT_TEXTS.get(key).copied()\n");
    content.push_str("}\n\n");
    
    content.push_str("/// 获取所有默认文本键\n");
    content.push_str("pub fn get_all_default_text_keys() -> impl Iterator<Item = &'static str> {\n");
    content.push_str("    DEFAULT_TEXTS.keys().copied()\n");
    content.push_str("}\n");

    fs::write(output_path, content)?;
    Ok(())
}

/// 将硬编码字符串替换为 t! 宏调用
fn replace_hardcoded_strings(file_path: &Path, texts: &LinkedHashMap<String, String>) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let mut new_content = content.clone();
    
    // 反向映射：从值到键
    let mut value_to_key: HashMap<String, String> = HashMap::new();
    for (key, value) in texts {
        value_to_key.insert(value.clone(), key.clone());
    }
    
    // 遍历所有可能的字符串值
    for (value, key) in &value_to_key {
        // 带引号的完整模式，例如 "Zed"
        let pattern = format!("\"{}\"", value);
        // 替换为 t!(cx, "i18n.top_menu_bar.zed") 格式
        let replacement = format!("t!(cx, \"{}\")", key);
        
        // 全局替换
        new_content = new_content.replace(&pattern, &replacement);
    }
    
    // 添加必要的导入
    if !new_content.contains("use crate::i18n::t") {
        new_content = format!("use crate::i18n::t;\n\n{}", new_content);
    }
    
    fs::write(file_path, new_content)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("用法:");
        println!("  扫描并生成 defaults 文件:");
        println!("    i18n-scan-app-menus scan <app_menus.rs路径> <输出文件路径>");
        println!("  替换硬编码字符串为 t! 宏:");
        println!("    i18n-scan-app-menus replace <app_menus.rs路径> <defaults文件路径>");
        println!("\n示例:");
        println!("    i18n-scan-app-menus scan crates/zed/src/zed/app_menus.rs crates/i18n/core/defaults-app-menus.rs");
        println!("    i18n-scan-app-menus replace crates/zed/src/zed/app_menus.rs crates/i18n/core/defaults-app-menus.rs");
        return Ok(());
    }

    // 如果第一个参数不是 scan/replace,则使用默认命令 scan
    let (command, args_start) = if args[1] == "scan" || args[1] == "replace" {
        (args[1].as_str(), 2)
    } else {
        ("scan", 1)
    };

    match command {
        "scan" => {
            // 检查参数数量: 程序名 + 2个路径
            if args.len() != args_start + 2 {
                println!("扫描命令需要指定源文件和输出文件路径");
                return Ok(());
            }
            let input_path = Path::new(&args[args_start]);
            let output_path = Path::new(&args[args_start + 1]);

            println!("正在扫描菜单文件: {}", input_path.display());
            let texts = scan_app_menus(input_path)?;
            println!("找到 {} 个需要国际化的字符串", texts.len());

            println!("正在生成 defaults-app-menus.rs 文件: {}", output_path.display());
            generate_defaults_app_menus(&texts, output_path)?;
            println!("完成! 请检查生成的文件确保无误，然后运行 replace 命令进行替换。");
        }
        "replace" => {
            if args.len() != args_start + 2 {
                println!("替换命令需要指定源文件和 defaults 文件路径");
                return Ok(());
            }
            let source_path = Path::new(&args[args_start]);
            let defaults_path = Path::new(&args[args_start + 1]);

            println!("正在从 {} 读取已生成的键值对...", defaults_path.display());
            let texts = scan_defaults_file(defaults_path)?;
            println!("读取到 {} 个键值对", texts.len());
            
            println!("正在替换硬编码字符串为 t! 宏调用...");
            replace_hardcoded_strings(source_path, &texts)?;
            println!("替换完成!");
        }
        _ => {
            println!("未知命令: {}", command);
            return Ok(());
        }
    }

    Ok(())
}

/// 从已生成的 defaults 文件中读取键值对
fn scan_defaults_file(path: &Path) -> Result<LinkedHashMap<String, String>> {
    let content = fs::read_to_string(path)?;
    let mut texts = LinkedHashMap::new();
    
    for line in content.lines() {
        if line.contains("texts.insert") {
            if let (Some(key_start), Some(key_end)) = (line.find('"'), line.rfind('"')) {
                let parts: Vec<&str> = line[key_start..=key_end].split("\", \"").collect();
                if parts.len() == 2 {
                    let key = parts[0].trim_start_matches('"').to_string();
                    let value = parts[1].trim_end_matches('"').to_string();
                    texts.insert(key, value);
                }
            }
        }
    }
    
    Ok(texts)
}
