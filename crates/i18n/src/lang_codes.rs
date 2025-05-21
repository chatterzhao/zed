use std::collections::HashMap;
use lazy_static::lazy_static;
use anyhow::{Result, anyhow};

//------------------------------------------------------------------------------
// 核心数据结构
//------------------------------------------------------------------------------

/// 表示一个语言及其相关信息
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language {
    /// 标准化的语言代码,如 "zh-cn", "en"
    pub code: String,
    /// 本地化显示的语言名称,如"简体中文", "English"
    pub display_name: String,
}

//------------------------------------------------------------------------------
// 静态映射表
//------------------------------------------------------------------------------

lazy_static! {
    /// 系统语言代码标准化映射
    /// key: 系统返回的语言代码(小写)
    /// value: 标准化的语言代码
    pub static ref SYSTEM_LANG_MAPPINGS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        
        // 简体中文映射
        m.insert("chinese-simplified", "zh-cn");
        m.insert("chinese (simplified)", "zh-cn");
        m.insert("chs", "zh-cn");
        m.insert("zh_cn", "zh-cn");
        m.insert("zh-cn", "zh-cn");
        m.insert("zh_hans", "zh-cn");
        m.insert("zh-hans", "zh-cn");
        m.insert("zh_cn.utf8", "zh-cn");
        m.insert("zh_cn.utf-8", "zh-cn");
        m.insert("zh.utf8", "zh-cn");
        m.insert("zh", "zh-cn"); // 默认zh映射到简中
        m.insert("chinese", "zh-cn");
        m.insert("zho", "zh-cn");
        m.insert("zhcn", "zh-cn");
        
        // 繁体中文映射
        m.insert("chinese-traditional", "zh-tw");
        m.insert("chinese (traditional)", "zh-tw");
        m.insert("cht", "zh-tw");
        m.insert("zh_tw", "zh-tw");
        m.insert("zh-tw", "zh-tw");
        m.insert("zh_hant", "zh-tw");
        m.insert("zh-hant", "zh-tw");
        m.insert("zh_tw.utf8", "zh-tw");
        m.insert("zh_tw.utf-8", "zh-tw");
        m.insert("zh_hk", "zh-tw");
        m.insert("zh-hk", "zh-tw");
        m.insert("zh_mo", "zh-tw");
        m.insert("zh-mo", "zh-tw");
        m.insert("zhtw", "zh-tw");
        m.insert("zhht", "zh-tw");
        
        // 日语映射
        m.insert("japanese", "ja");
        m.insert("ja", "ja");
        m.insert("ja_jp", "ja");
        m.insert("ja-jp", "ja");
        m.insert("jpn", "ja");
        m.insert("ja_ja", "ja");
        m.insert("ja-ja", "ja");
        m.insert("ja.utf8", "ja");
        m.insert("ja.utf-8", "ja");
        m.insert("ja_jp.utf8", "ja");
        m.insert("ja_jp.utf-8", "ja");
        
        // 韩语映射
        m.insert("korean", "ko");
        m.insert("ko", "ko");
        m.insert("ko_kr", "ko");
        m.insert("ko-kr", "ko");
        m.insert("kor", "ko");
        m.insert("ko_ko", "ko");
        m.insert("ko-ko", "ko");
        m.insert("ko.utf8", "ko");
        m.insert("ko.utf-8", "ko");
        m.insert("ko_kr.utf8", "ko");
        m.insert("ko_kr.utf-8", "ko");
        
        // 越南语映射
        m.insert("vietnamese", "vi");
        m.insert("vi", "vi");
        m.insert("vi_vn", "vi");
        m.insert("vi-vn", "vi");
        m.insert("vie", "vi");
        m.insert("vi.utf8", "vi");
        m.insert("vi.utf-8", "vi");
        m.insert("vi_vn.utf8", "vi");
        
        // 泰语映射
        m.insert("thai", "th");
        m.insert("th", "th");
        m.insert("th_th", "th"); 
        m.insert("th-th", "th");
        m.insert("tha", "th");
        m.insert("th.utf8", "th");
        m.insert("th_th.utf8", "th");
        
        // 印尼语映射
        m.insert("indonesian", "id");
        m.insert("id", "id");
        m.insert("id_id", "id");
        m.insert("id-id", "id");
        m.insert("ind", "id");
        m.insert("id.utf8", "id");
        m.insert("id_id.utf8", "id");
        
        // 马来语映射
        m.insert("malay", "ms");
        m.insert("ms", "ms");
        m.insert("ms_my", "ms");
        m.insert("ms-my", "ms");
        m.insert("msa", "ms");
        m.insert("ms.utf8", "ms");
        m.insert("ms_my.utf8", "ms");
        
        // 其他亚洲语言...
        
        // 西欧语言
        // 西班牙语
        m.insert("spanish", "es");
        m.insert("es", "es");
        m.insert("es_es", "es");
        m.insert("es-es", "es");
        m.insert("spa", "es");
        m.insert("es_419", "es");
        m.insert("es-419", "es");
        m.insert("es_mx", "es");
        m.insert("es-mx", "es");
        m.insert("es.utf8", "es");
        m.insert("es_es.utf8", "es");
        
        // 法语
        m.insert("french", "fr");
        m.insert("fr", "fr");
        m.insert("fr_fr", "fr");
        m.insert("fr-fr", "fr");
        m.insert("fra", "fr");
        m.insert("fr_ca", "fr");
        m.insert("fr-ca", "fr");
        m.insert("fr_be", "fr");
        m.insert("fr_ch", "fr");
        m.insert("fr.utf8", "fr");
        m.insert("fr_fr.utf8", "fr");
        
        // 德语
        m.insert("german", "de");
        m.insert("de", "de");
        m.insert("de_de", "de");
        m.insert("de-de", "de");
        m.insert("deu", "de");
        m.insert("de_at", "de");
        m.insert("de-at", "de");
        m.insert("de_ch", "de");
        m.insert("de.utf8", "de");
        m.insert("de_de.utf8", "de");
        
        // 意大利语
        m.insert("italian", "it");
        m.insert("it", "it");
        m.insert("it_it", "it");
        m.insert("it-it", "it");
        m.insert("ita", "it");
        m.insert("it_ch", "it");
        m.insert("it.utf8", "it");
        m.insert("it_it.utf8", "it");
        
        m
    };

    /// 扩展ID与标准语言代码的关联词列表
    /// 用于模糊匹配扩展ID中的语言信息
    pub static ref EXTENSION_LANG_KEYWORDS: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        
        // 简体中文相关关键词
        m.insert("zh-cn", vec![
            "zh-cn", "zh_cn", "zhcn", "zh-CN", "zh_CN",
            "simplified", "简体", "简体中文", "中文", "汉语",
        ]);
        
        // 繁体中文相关关键词
        m.insert("zh-tw", vec![
            "zh-tw", "zh_tw", "zhtw", "zh-TW", "zh_TW",
            "traditional", "繁體", "繁體中文", "正體", "正體中文",
            "zh-hk", "zh_hk", "香港", "台灣", "澳門",
        ]);
        
        // 日语相关关键词
        m.insert("ja", vec![
            "ja", "jp", "jpn", "japanese",
            "日本語", "日文", "にほんご",
        ]);
        
        // 韩语相关关键词
        m.insert("ko", vec![
            "ko", "kr", "kor", "korean",
            "한국어", "韓國語", "조선말",
        ]);
        
        // 越南语相关关键词
        m.insert("vi", vec![
            "vi", "vie", "vietnamese",
            "tieng-viet", "tiếng-việt",
        ]);
        
        // 其他东亚语言...
        
        // 西欧语言关键词
        m.insert("es", vec![
            "es", "spa", "spanish",
            "español", "castellano",
            "西班牙语", "西班牙文",
        ]);
        
        m.insert("fr", vec![
            "fr", "fra", "french",
            "français", "法语", "法文",
        ]);
        
        m.insert("de", vec![
            "de", "deu", "german", "deutsch",
            "德语", "德文",
        ]);

        m.insert("it", vec![
            "it", "ita", "italian", "italiano",
            "意大利语", "意大利文",
        ]);
        
        m
    };

    /// 语言的本地化显示名称
    pub static ref LANG_NATIVE_NAMES: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        // 东亚语言
        m.insert("zh-cn", "简体中文");
        m.insert("zh-tw", "繁體中文");
        m.insert("ja", "日本語");
        m.insert("ko", "한국어");
        
        // 东南亚语言
        m.insert("vi", "Tiếng Việt");
        m.insert("th", "ไทย");
        m.insert("id", "Bahasa Indonesia");
        m.insert("ms", "Bahasa Melayu");
        
        // 西欧语言
        m.insert("en", "English");
        m.insert("es", "Español");
        m.insert("fr", "Français");
        m.insert("de", "Deutsch");
        m.insert("it", "Italiano");
        m.insert("pt", "Português");
        m.insert("nl", "Nederlands");
        
        // 东欧语言
        m.insert("ru", "Русский");
        m.insert("pl", "Polski");
        m.insert("uk", "Українська");
        m.insert("cs", "Čeština");
        m.insert("hu", "Magyar");
        
        // 其他语言
        m.insert("ar", "العربية");
        m.insert("tr", "Türkçe");
        m.insert("hi", "हिन्दी");
        m.insert("he", "עברית");
        
        m
    };

    /// 扩展市场搜索关键词
    pub static ref LANG_SEARCH_KEYWORDS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("zh-cn", "Chinese Simplified 简体中文 中文");
        m.insert("zh-tw", "Chinese Traditional 繁體中文 正體中文"); 
        m.insert("ja", "Japanese 日本語 にほんご");
        m.insert("ko", "Korean 한국어 韓國語");
        m.insert("vi", "Vietnamese Tiếng Việt");
        m.insert("th", "Thai ไทย ภาษาไทย");
        m.insert("id", "Indonesian Bahasa Indonesia");
        m.insert("ms", "Malay Bahasa Melayu Melayu");
        m.insert("es", "Spanish Español Castellano");
        m.insert("fr", "French Français");
        m.insert("de", "German Deutsch");
        m.insert("it", "Italian Italiano");
        m.insert("pt", "Portuguese Português");
        m.insert("ru", "Russian Русский русский язык");
        m.insert("pl", "Polish Polski język polski");
        m.insert("ar", "Arabic العربية");
        m.insert("tr", "Turkish Türkçe");
        m
    };
}

//------------------------------------------------------------------------------
// Language 实现
//------------------------------------------------------------------------------

impl Language {
    /// 从语言代码创建 Language 实例
    ///
    /// # Arguments
    /// * `code` - 语言代码，如 "zh-cn", "ja" 等
    ///
    /// # Returns
    /// * `Ok(Language)` - 如果语言代码有效
    /// * `Err` - 如果语言代码不支持
    pub fn from_code(code: &str) -> anyhow::Result<Self> {
        let code = code.to_lowercase();
        let native_name = LANG_NATIVE_NAMES
            .get(code.as_str())
            .ok_or_else(|| anyhow::anyhow!("不支持的语言代码: {}", code))?;

        Ok(Self {
            code: code.to_string(),
            display_name: native_name.to_string(),
        })
    }

    /// 获取该语言对应的扩展搜索关键词
    pub fn search_keywords(&self) -> Option<&'static str> {
        LANG_SEARCH_KEYWORDS.get(self.code.as_str()).copied()
    }
    
    /// 检查指定的扩展ID是否支持该语言
    pub fn is_supported_by_extension(&self, extension_id: &str) -> bool {
        is_extension_match(extension_id, &self.code)
    }

    /// 验证语言代码是否有效
    pub fn validate(lang_code: &str) -> Result<()> {
        if lang_code.is_empty() {
            return Ok(());
        }
        if lang_code == "en" {
            return Ok(());
        }
        
        // 尝试创建 Language 实例来验证
        Language::from_code(lang_code)?;
        Ok(())
    }

    /// 查找匹配给定语言代码的扩展ID
    pub fn find_extension_id(lang_code: &str) -> Option<String> {
        let lang_code = lang_code.to_lowercase();
        
        if let Ok(lang) = Language::from_code(&lang_code) {
            // 使用关键词列表进行模糊匹配
            if let Some(keywords) = EXTENSION_LANG_KEYWORDS.get(lang.code.as_str()) {
                for keyword in keywords {
                    let extension_id = format!("i18n-{}", keyword);
                    if is_extension_match(&extension_id, &lang.code) {
                        return Some(extension_id);
                    }
                }
            }
            // 返回基于标准化语言代码的扩展ID
            return Some(format!("i18n-{}", lang.code));
        }
        None
    }

    /// 获取语言的显示名称
    pub fn get_display_name(lang_code: &str) -> Option<&'static str> {
        LANG_NATIVE_NAMES.get(lang_code).copied()
    }
}

impl TryFrom<&str> for Language {
    type Error = anyhow::Error;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        Self::from_code(code)
    }
}

//------------------------------------------------------------------------------
// 公共函数
//------------------------------------------------------------------------------

/// 检查扩展ID是否匹配指定的语言代码
///
/// # Arguments
/// * `extension_id` - 扩展ID，如 "i18n-zh-cn"
/// * `lang_code` - 语言代码，如 "zh-cn"
///
/// # Returns
/// * `true` - 如果扩展支持该语言
/// * `false` - 如果扩展不支持该语言
pub fn is_extension_match(extension_id: &str, lang_code: &str) -> bool {
    let extension_id = extension_id.to_lowercase();
    
    if !extension_id.starts_with("i18n-") {
        return false;
    }
    
    let lang_part = &extension_id[5..];

    // 1. 直接匹配标准代码
    if lang_part.starts_with(lang_code) {
        return true;
    }
    
    // 2. 检查关键词匹配
    if let Some(keywords) = EXTENSION_LANG_KEYWORDS.get(lang_code) {
        if keywords.iter().any(|&keyword| lang_part.contains(&keyword.to_lowercase())) {
            return true;
        }
    }

    // 3. 检查本地化名称匹配
    if let Some(native_name) = LANG_NATIVE_NAMES.get(lang_code) {
        if lang_part.contains(&native_name.to_lowercase()) {
            return true;
        }
    }

    false
}

/// 获取并标准化系统语言环境
///
/// # Returns
/// * `Some(String)` - 标准化的语言代码
/// * `None` - 如果系统语言不受支持
pub fn get_system_language() -> Option<String> {
    std::env::var("LANG")
        .ok()
        .and_then(|lang| {
            let system_lang = lang
                .split('.')
                .next()?
                .to_lowercase();
            
            SYSTEM_LANG_MAPPINGS
                .get(system_lang.as_str())
                .copied()
                .or_else(|| {
                    system_lang
                        .split(['_', '-'])
                        .next()
                        .and_then(|main_lang| SYSTEM_LANG_MAPPINGS.get(main_lang).copied())
                })
                .map(|s| s.to_string())
        })
}

//------------------------------------------------------------------------------
// 测试
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_creation() {
        // 有效的语言代码
        let zh_cn = Language::from_code("zh-cn").unwrap();
        assert_eq!(zh_cn.code, "zh-cn");
        assert_eq!(zh_cn.display_name, "简体中文");
        
        let ja = Language::from_code("ja").unwrap();
        assert_eq!(ja.code, "ja");
        assert_eq!(ja.display_name, "日本語");
        
        // 无效的语言代码
        assert!(Language::from_code("invalid").is_err());
    }

    #[test]
    fn test_system_language_detection() {
        // 这里我们模拟设置环境变量来测试
        std::env::set_var("LANG", "zh_CN.UTF-8");
        assert_eq!(get_system_language().unwrap(), "zh-cn");
        
        std::env::set_var("LANG", "ja_JP.utf8");
        assert_eq!(get_system_language().unwrap(), "ja");
        
        std::env::set_var("LANG", "en_US.UTF-8");
        assert_eq!(get_system_language(), None); // 英语暂不支持
    }

    #[test]
    fn test_extension_matching() {
        // 标准格式
        assert!(is_extension_match("i18n-zh-cn", "zh-cn"));
        assert!(is_extension_match("i18n-zh-CN", "zh-cn"));
        
        // 带后缀的格式
        assert!(is_extension_match("i18n-zh-cn-official", "zh-cn"));
        assert!(is_extension_match("i18n-zh-cn-community", "zh-cn"));
        assert!(is_extension_match("i18n-zh-cn-阿里", "zh-cn"));
        
        // 使用本地化名称
        assert!(is_extension_match("i18n-简体中文", "zh-cn"));
        assert!(is_extension_match("i18n-简体中文-社区版", "zh-cn"));
        
        // 使用关键词
        assert!(is_extension_match("i18n-zhcn-custom", "zh-cn"));
        assert!(is_extension_match("i18n-chinese-simplified", "zh-cn"));
        
        // 不匹配的情况
        assert!(!is_extension_match("zh-cn", "zh-cn")); // 不是i18n-开头
        assert!(!is_extension_match("i18n-en", "zh-cn")); // 语言不匹配
        assert!(!is_extension_match("i18n-japanese", "zh-cn")); // 语言不匹配
    }

    #[test]
    fn test_language_mappings() {
        // 测试标准化映射
        assert_eq!(SYSTEM_LANG_MAPPINGS.get("zh_cn"), Some(&"zh-cn"));
        assert_eq!(SYSTEM_LANG_MAPPINGS.get("chinese-simplified"), Some(&"zh-cn"));
        assert_eq!(SYSTEM_LANG_MAPPINGS.get("japanese"), Some(&"ja"));
        
        // 测试本地化名称
        assert_eq!(LANG_NATIVE_NAMES.get("zh-cn"), Some(&"简体中文"));
        assert_eq!(LANG_NATIVE_NAMES.get("ja"), Some(&"日本語"));
        
        // 测试搜索关键词
        assert!(LANG_SEARCH_KEYWORDS.get("zh-cn").unwrap().contains("中文"));
        assert!(LANG_SEARCH_KEYWORDS.get("ja").unwrap().contains("日本語"));
    }
}
