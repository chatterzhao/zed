use anyhow::{anyhow, Result};
use std::env;
use std::path::PathBuf;
use zed_i18n_tools::I18NValidator;

/// 验证国际化扩展的翻译文件
fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("用法: cargo run --bin i18n-validate <语言包路径>");
        println!("例如: cargo run --bin i18n-validate extensions/i18n-fr");
        return Ok(());
    }
    
    // 获取语言包路径
    let lang_pack_path = PathBuf::from(&args[1]);
    if !lang_pack_path.exists() || !lang_pack_path.is_dir() {
        return Err(anyhow!("语言包目录不存在或不是一个目录: {}", lang_pack_path.display()));
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
    
    // 验证语言包
    println!("正在验证语言包: {}", lang_pack_path.display());
    let report = validator.validate(&lang_pack_path)?;
    
    // 输出验证结果
    let errors = report.get_errors();
    let warnings = report.get_warnings();
    
    if !errors.is_empty() {
        println!("\n❌ 验证失败，发现 {} 个错误:", errors.len());
        for error in errors {
            println!("  - {}", error);
        }
    }
    
    if !warnings.is_empty() {
        println!("\n⚠️ 发现 {} 个警告:", warnings.len());
        for warning in warnings {
            println!("  - {}", warning);
        }
    }
    
    if errors.is_empty() && warnings.is_empty() {
        println!("\n✓ 验证通过，没有发现问题。");
    } else if errors.is_empty() {
        println!("\n✓ 验证通过，但有警告需要注意。");
    }
    
    Ok(())
}