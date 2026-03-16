use std::path::{Path, PathBuf};
use std::time::SystemTime;

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
    pub extension: String,
}

impl FileEntry {
    pub fn icon(&self) -> &str {
        if self.is_dir {
            "\u{1F4C1}"
        } else {
            match self.extension.as_str() {
                "rs" => "\u{1F9F0}",
                "txt" | "md" => "\u{1F4DD}",
                "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" => "\u{1F5BC}",
                "mp3" | "wav" | "flac" | "ogg" => "\u{1F3B5}",
                "mp4" | "avi" | "mkv" | "mov" => "\u{1F3AC}",
                "zip" | "tar" | "gz" | "7z" | "rar" => "\u{1F4E6}",
                "exe" | "msi" => "\u{2699}",
                "pdf" => "\u{1F4D5}",
                "toml" | "json" | "yaml" | "yml" | "xml" => "\u{2699}",
                _ => "\u{1F4C4}",
            }
        }
    }

    pub fn formatted_size(&self) -> String {
        if self.is_dir {
            return String::new();
        }
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        match self.size {
            s if s >= GB => format!("{:.1} GB", s as f64 / GB as f64),
            s if s >= MB => format!("{:.1} MB", s as f64 / MB as f64),
            s if s >= KB => format!("{:.1} KB", s as f64 / KB as f64),
            s => format!("{s} B"),
        }
    }

    pub fn formatted_date(&self) -> String {
        match self.modified {
            Some(time) => {
                let datetime: DateTime<Local> = time.into();
                datetime.format("%Y-%m-%d %H:%M").to_string()
            }
            None => String::new(),
        }
    }

    pub fn file_type_label(&self) -> String {
        if self.is_dir {
            "Folder".to_string()
        } else if self.extension.is_empty() {
            "File".to_string()
        } else {
            format!("{} File", self.extension.to_uppercase())
        }
    }
}

pub fn read_directory(path: &Path) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut entries = Vec::new();

    for dir_entry in std::fs::read_dir(path)? {
        let dir_entry = dir_entry?;
        let metadata = dir_entry.metadata()?;
        let name = dir_entry.file_name().to_string_lossy().to_string();
        let path = dir_entry.path();
        let extension = path
            .extension()
            .map(|e| e.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        entries.push(FileEntry {
            name,
            path,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().ok(),
            extension,
        });
    }

    // Directories first, then files; alphabetical within each group
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}
