use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const MAX_RECENT: usize = 10;
const FILE_NAME: &str = "recent_files.json";

#[derive(Serialize, Deserialize, Default)]
pub struct RecentFiles {
    paths: Vec<PathBuf>,
}

impl RecentFiles {
    /// Load recent files from disk, or return empty if unavailable.
    pub fn load() -> Self {
        Self::config_path()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    /// Add a path to the front of the list (deduplicates and truncates).
    pub fn add(&mut self, path: &Path) {
        let canonical = path.to_path_buf();
        self.paths.retain(|p| p != &canonical);
        self.paths.insert(0, canonical);
        self.paths.truncate(MAX_RECENT);
        self.save();
    }

    /// Get the list of recent file paths.
    pub fn list(&self) -> &[PathBuf] {
        &self.paths
    }

    fn save(&self) {
        if let Some(dir) = Self::config_dir() {
            let _ = std::fs::create_dir_all(&dir);
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(dir.join(FILE_NAME), json);
            }
        }
    }

    fn config_dir() -> Option<PathBuf> {
        dirs::data_local_dir().map(|d| d.join("pdf_reader"))
    }

    fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|d| d.join(FILE_NAME))
    }
}
