use anyhow::{anyhow, Result};
use std::env;
use std::path::{Path, PathBuf};
use zed_i18n_tools::I18NValidator;

/// 重组翻译文件，根据默认键自动更新现有翻译
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("用法: cargo run --bin i18n-reorganize <翻译文件路径>");
        println!("例如: cargo run --bin i18n-reorganize resources/translations/translation.json");
        return Ok(());
    }
    
    // 获取翻译文件路径
    let translation_file_path = PathBuf::from(&args[1]);
    if !translation_file_path.exists() || !translation_file_path.is_file() {
        return Err(anyhow!("翻译文件不存在或不是一个文件: {}", translation_file_path.display()));
    }
    
    // 创建验证器
    let mut validator = I18NValidator::new(env::current_dir()?
        .parent()
        .ok_or_else(|| anyhow!("找不到父目录"))?
        .parent()
        .ok_or_else(|| anyhow!("找不到父目录"))?
        .to_path_buf());
    
    // 加载参考键
    println!("正在加载参考键...");
    validator.load_reference_keys()?;
    
    // 重组翻译文件
    println!("正在重组翻译文件: {}", translation_file_path.display());
    validator.reorganize_translations(&translation_file_path)?;
    
    println!("✓ 翻译文件重组完成。");
    println!("  - 已添加缺失的键");
    println!("  - 已按照参考键顺序排序");
    println!("  - 已移除未使用的键");
    
    Ok(())
}