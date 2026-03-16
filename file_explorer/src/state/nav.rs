use std::path::{Path, PathBuf};

pub struct NavState {
    pub current_dir: PathBuf,
    pub back_stack: Vec<PathBuf>,
    pub forward_stack: Vec<PathBuf>,
    pub address_bar_text: String,
    pub editing_address: bool,
}

impl NavState {
    pub fn new(start: PathBuf) -> Self {
        let address_bar_text = start.display().to_string();
        Self {
            current_dir: start,
            back_stack: Vec::new(),
            forward_stack: Vec::new(),
            address_bar_text,
            editing_address: false,
        }
    }

    pub fn navigate(&mut self, path: PathBuf) {
        self.back_stack.push(self.current_dir.clone());
        self.forward_stack.clear();
        self.current_dir = path;
        self.address_bar_text = self.current_dir.display().to_string();
    }

    pub fn go_back(&mut self) -> Option<PathBuf> {
        let prev = self.back_stack.pop()?;
        self.forward_stack.push(self.current_dir.clone());
        self.current_dir = prev.clone();
        self.address_bar_text = self.current_dir.display().to_string();
        Some(prev)
    }

    pub fn go_forward(&mut self) -> Option<PathBuf> {
        let next = self.forward_stack.pop()?;
        self.back_stack.push(self.current_dir.clone());
        self.current_dir = next.clone();
        self.address_bar_text = self.current_dir.display().to_string();
        Some(next)
    }

    pub fn go_up(&mut self) -> Option<PathBuf> {
        let parent = self.current_dir.parent()?.to_path_buf();
        self.navigate(parent.clone());
        Some(parent)
    }

    pub fn breadcrumbs(&self) -> Vec<(String, PathBuf)> {
        let mut crumbs = Vec::new();
        let mut current = Some(self.current_dir.as_path());

        while let Some(path) = current {
            let name = if path.parent().is_none() {
                path.display().to_string()
            } else {
                path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.display().to_string())
            };
            crumbs.push((name, path.to_path_buf()));
            current = path.parent();
        }

        crumbs.reverse();
        crumbs
    }

    pub fn has_parent(&self) -> bool {
        self.current_dir.parent().is_some()
            && self.current_dir.parent() != Some(Path::new(""))
    }
}
