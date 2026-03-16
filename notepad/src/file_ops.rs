use std::fs;
use std::path::PathBuf;

pub fn open_file_dialog() -> Option<(PathBuf, String)> {
    let path = rfd::FileDialog::new()
        .add_filter("Text Files", &["txt", "md", "rs", "toml", "json", "xml", "log"])
        .add_filter("All Files", &["*"])
        .pick_file()?;

    match fs::read_to_string(&path) {
        Ok(content) => Some((path, content)),
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            None
        }
    }
}

pub fn save_file(path: &PathBuf, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|e| format!("Failed to save: {}", e))
}

pub fn save_file_as_dialog(content: &str) -> Option<PathBuf> {
    let path = rfd::FileDialog::new()
        .add_filter("Text Files", &["txt"])
        .add_filter("All Files", &["*"])
        .save_file()?;

    match fs::write(&path, content) {
        Ok(()) => Some(path),
        Err(e) => {
            eprintln!("Failed to save file: {}", e);
            None
        }
    }
}
