use anyhow::{Result, Context};
use async_trait::async_trait;
use futures::future::BoxFuture;
use gpui::BackgroundExecutor;
use std::{
    path::PathBuf,
    sync::Arc,
};
use crate::core::{I18nManager, I18nLangMeta};
use crate::i18n_tools::I18nValidator;

pub struct I18nImporter {
    fs: Arc<dyn Fs>,
    executor: BackgroundExecutor,
}

impl I18nImporter {
    pub fn new(fs: Arc<dyn Fs>, executor: BackgroundExecutor) -> Self {
        Self { fs, executor }
    }

    pub fn import_from_file(&self, path: PathBuf) -> BoxFuture<'static, Result<I18nLangMeta>> {
        let fs = self.fs.clone();
        let executor = self.executor.clone();

        Box::pin(async move {
            // 读取文件内容
            let content = fs.read_to_string(&path)
                .await
                .context("Failed to read language pack file")?;

            // 解析语言包元数据
            let meta: I18nLangMeta = serde_json::from_str(&content)
                .context("Failed to parse language pack metadata")?;

            // 验证语言包
            let validator = I18nValidator::new(path.clone());
            let report = validator.validate()
                .context("Failed to validate language pack")?;

            if !report.missing_keys.is_empty() {
                log::warn!("Language pack is missing some translations: {:?}", report.missing_keys);
            }

            if !report.extra_keys.is_empty() {
                log::warn!("Language pack has extra translations: {:?}", report.extra_keys);
            }

            if !report.format_errors.is_empty() {
                log::warn!("Language pack has format errors: {:?}", report.format_errors);
            }

            // 复制语言包到扩展目录
            let extension_dir = get_extension_dir()?;
            let target_dir = extension_dir.join(format!("i18n-{}", meta.id));
            
            fs.create_dir_all(&target_dir)
                .await
                .context("Failed to create extension directory")?;

            // 复制所有文件
            copy_dir_recursive(&path, &target_dir, &fs)
                .await
                .context("Failed to copy language pack files")?;

            Ok(meta)
        })
    }

    pub fn import_from_url(&self, url: &str) -> BoxFuture<'static, Result<I18nLangMeta>> {
        let url = url.to_string();
        let fs = self.fs.clone();
        let executor = self.executor.clone();

        Box::pin(async move {
            // 下载语言包
            let response = reqwest::get(&url)
                .await
                .context("Failed to download language pack")?;

            let content = response.bytes()
                .await
                .context("Failed to read response body")?;

            // 创建临时目录
            let temp_dir = tempfile::tempdir()
                .context("Failed to create temporary directory")?;

            let temp_path = temp_dir.path().join("language_pack.zip");

            // 保存下载的文件
            fs.write(&temp_path, &content)
                .await
                .context("Failed to save downloaded file")?;

            // 解压文件
            let extract_dir = temp_dir.path().join("extract");
            fs.create_dir_all(&extract_dir)
                .await
                .context("Failed to create extract directory")?;

            // 解压 ZIP 文件
            let file = std::fs::File::open(&temp_path)
                .context("Failed to open downloaded file")?;
            let mut archive = zip::ZipArchive::new(file)
                .context("Failed to read ZIP archive")?;

            for i in 0..archive.len() {
                let mut file = archive.by_index(i)
                    .context("Failed to read file from archive")?;
                
                let outpath = extract_dir.join(file.name());
                
                if file.name().ends_with('/') {
                    fs.create_dir_all(&outpath)
                        .await
                        .context("Failed to create directory")?;
                } else {
                    let mut outfile = std::fs::File::create(&outpath)
                        .context("Failed to create output file")?;
                    std::io::copy(&mut file, &mut outfile)
                        .context("Failed to extract file")?;
                }
            }

            // 导入解压后的语言包
            let meta = self.import_from_file(extract_dir).await?;

            // 清理临时文件
            temp_dir.close()
                .context("Failed to clean up temporary files")?;

            Ok(meta)
        })
    }
}

async fn copy_dir_recursive(
    src: &PathBuf,
    dst: &PathBuf,
    fs: &Arc<dyn Fs>,
) -> Result<()> {
    let entries = fs.read_dir(src)
        .await
        .context("Failed to read source directory")?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if entry.file_type().await?.is_dir() {
            fs.create_dir_all(&dst_path)
                .await
                .context("Failed to create directory")?;
            copy_dir_recursive(&src_path, &dst_path, fs).await?;
        } else {
            fs.copy(&src_path, &dst_path)
                .await
                .context("Failed to copy file")?;
        }
    }

    Ok(())
}

fn get_extension_dir() -> Result<PathBuf> {
    let mut path = dirs::data_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get data directory"))?;
    
    path.push("Zed");
    path.push("extensions");
    
    Ok(path)
}

#[async_trait]
pub trait Fs: Send + Sync {
    async fn read_to_string(&self, path: &PathBuf) -> Result<String>;
    async fn write(&self, path: &PathBuf, content: &[u8]) -> Result<()>;
    async fn create_dir_all(&self, path: &PathBuf) -> Result<()>;
    async fn read_dir(&self, path: &PathBuf) -> Result<Vec<DirEntry>>;
    async fn copy(&self, src: &PathBuf, dst: &PathBuf) -> Result<()>;
}

pub struct DirEntry {
    pub path: PathBuf,
    pub file_name: String,
    pub file_type: FileType,
}

pub struct FileType {
    pub is_dir: bool,
} 