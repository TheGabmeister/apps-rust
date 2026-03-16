use std::path::{Path, PathBuf};

pub fn create_folder(parent: &Path, name: &str) -> Result<PathBuf, std::io::Error> {
    let path = parent.join(name);
    std::fs::create_dir(&path)?;
    Ok(path)
}

pub fn delete_entry(path: &Path) -> Result<(), std::io::Error> {
    if path.is_dir() {
        std::fs::remove_dir_all(path)
    } else {
        std::fs::remove_file(path)
    }
}

pub fn rename_entry(from: &Path, new_name: &str) -> Result<PathBuf, std::io::Error> {
    let to = from
        .parent()
        .expect("entry must have a parent directory")
        .join(new_name);
    std::fs::rename(from, &to)?;
    Ok(to)
}
