use anyhow::Result;
use std::{env, path::PathBuf};
use zed_i18n_tools::I18NTemplate;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("用法: cargo run --bin i18n-new i18n-xx");
        println!("例如: i18n-fr 将会是扩展的目录名");
        return Ok(());
    }

    let extension_id = &args[1];
    if !extension_id.starts_with("i18n-") {
        println!("错误: 扩展ID必须以i18n-开头");
        return Ok(());
    }

    // 从extension_id提取语言代码和信息
    let lang_id = extension_id.trim_start_matches("i18n-");
    let lang_info = get_language_info(lang_id);
    
    // 生成扩展目录路径
    let current_dir = env::current_dir()?;
    let target_dir = current_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("extensions")
        .join(extension_id);

    println!("正在生成 {} ({}) 语言包...", lang_info.name, lang_info.native_name);

    // 创建语言包模板
    let template = I18NTemplate::new(
        target_dir.clone(),
        lang_id.to_string(),
        lang_info.name.to_string(),
    );

    // 生成扩展
    template.generate()?;
    
    println!("✓ 成功创建语言包扩展: {}", extension_id);
    println!("  目录: {}", target_dir.display());
    println!("\n接下来的步骤:");
    println!("1. 编辑 resources/translations/translation.json 文件添加翻译");
    println!("2. 运行 cargo build 构建扩展");
    println!("3. 测试扩展功能");

    Ok(())
}

struct LangInfo {
    name: &'static str,
    native_name: &'static str,
}

fn get_language_info(lang_id: &str) -> LangInfo {
    match lang_id {
        "zh-cn" => LangInfo {
            name: "Chinese (Simplified)",
            native_name: "简体中文",
        },
        "zh-tw" => LangInfo {
            name: "Chinese (Traditional)", 
            native_name: "繁體中文",
        },
        "ja" => LangInfo {
            name: "Japanese",
            native_name: "日本語",
        },
        "ko" => LangInfo {
            name: "Korean",
            native_name: "한국어", 
        },
        "fr" => LangInfo {
            name: "French",
            native_name: "Français",
        },
        "de" => LangInfo {
            name: "German",
            native_name: "Deutsch",
        },
        "es" => LangInfo {
            name: "Spanish",
            native_name: "Español",
        },
        "it" => LangInfo {
            name: "Italian", 
            native_name: "Italiano",
        },
        "pt" => LangInfo {
            name: "Portuguese",
            native_name: "Português",
        },
        "ru" => LangInfo {
            name: "Russian",
            native_name: "Русский",
        },
        _ => LangInfo {
            name: "Unknown Language",
            native_name: "Unknown",
        },
    }
}
