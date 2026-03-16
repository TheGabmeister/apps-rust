use crate::fs::entries::FileEntry;

pub struct SearchState {
    pub query: String,
    pub is_active: bool,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            is_active: false,
        }
    }

    pub fn matches(&self, entry: &FileEntry) -> bool {
        if self.query.is_empty() {
            return true;
        }
        entry
            .name
            .to_lowercase()
            .contains(&self.query.to_lowercase())
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.is_active = false;
    }
}
