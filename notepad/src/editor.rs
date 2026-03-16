use std::path::PathBuf;

pub struct EditorState {
    pub content: String,
    pub file_path: Option<PathBuf>,
    pub dirty: bool,
    pub cursor_line: usize,
    pub cursor_col: usize,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            file_path: None,
            dirty: false,
            cursor_line: 1,
            cursor_col: 1,
        }
    }

    pub fn reset(&mut self) {
        self.content.clear();
        self.file_path = None;
        self.dirty = false;
        self.cursor_line = 1;
        self.cursor_col = 1;
    }

    pub fn load(&mut self, path: PathBuf, content: String) {
        self.content = content;
        self.file_path = Some(path);
        self.dirty = false;
    }

    pub fn mark_saved(&mut self, path: Option<PathBuf>) {
        if let Some(p) = path {
            self.file_path = Some(p);
        }
        self.dirty = false;
    }

    pub fn window_title(&self) -> String {
        let name = self
            .file_path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string());

        if self.dirty {
            format!("*{} - Notepad", name)
        } else {
            format!("{} - Notepad", name)
        }
    }

    pub fn update_cursor_position(&mut self, char_index: usize) {
        let byte_offset = self
            .content
            .char_indices()
            .nth(char_index)
            .map(|(i, _)| i)
            .unwrap_or(self.content.len());

        let text_before = &self.content[..byte_offset];
        self.cursor_line = text_before.matches('\n').count() + 1;
        self.cursor_col = match text_before.rfind('\n') {
            Some(pos) => byte_offset - pos,
            None => byte_offset + 1,
        };
    }
}
