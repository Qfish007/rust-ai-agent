use std::{fs, io, path::{Path, PathBuf}};

pub fn unzip(zip_path: &str, extract_to: Option<&str>) -> anyhow::Result<String> {
    let zip_path = Path::new(zip_path);

    // 检查文件是否存在
    if !zip_path.exists() {
        anyhow::bail!("File not found: {}", zip_path.display());
    }
    // 检查目标目录是否存在, 如果不存在则创建
    let extract_to: PathBuf = match extract_to {
        Some(dir) => PathBuf::from(dir),
        None => zip_path.with_extension(""),// 移除扩展名
    };
    fs::create_dir_all(&extract_to)?;

    // 解压文件
    let file = fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    // 解压文件
    let mut names = Vec::with_capacity(archive.len());
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let out_path = extract_to.join(entry.name());
        names.push(entry.name().to_string());

        if entry.is_dir() {
            // 如果是目录, 则创建目录
            fs::create_dir_all(&out_path)?;
        } else {
            // 如果是文件, 则创建文件
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = fs::File::create(&out_path)?;
            io::copy(&mut entry, &mut out_file)?;
        }
    }

    let mut summary = format!("Extracted {} files to {}/\n\nContents:\n", names.len(), extract_to.display());
    for name in names.iter().take(20) {
        summary.push_str(&format!(". - {name}\n"));
    }
    if names.len() > 20 {
        summary.push_str(&format!("  ... and {} more files\n", names.len() - 20));
    }

    Ok(summary)
}