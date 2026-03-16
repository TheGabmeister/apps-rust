use std::path::PathBuf;

use eframe::egui;

use crate::fs::entries::{self, FileEntry};
use crate::state::{NavState, SearchState, SelectionState};
use crate::ui;

pub enum DialogKind {
    NewFolder { name: String },
    Rename { original: String, new_name: String },
    DeleteConfirm { target: String, path: PathBuf },
}

pub struct FileExplorerApp {
    pub nav: NavState,
    pub selection: SelectionState,
    pub search: SearchState,
    pub entries: Vec<FileEntry>,
    pub dialog: Option<DialogKind>,
    pub error_message: Option<String>,
}

impl FileExplorerApp {
    pub fn new() -> Self {
        let start = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        let entries = entries::read_directory(&start).unwrap_or_default();
        Self {
            nav: NavState::new(start),
            selection: SelectionState::new(),
            search: SearchState::new(),
            entries,
            dialog: None,
            error_message: None,
        }
    }

    pub fn refresh(&mut self) {
        match entries::read_directory(&self.nav.current_dir) {
            Ok(mut e) => {
                self.selection.sort_entries(&mut e);
                self.entries = e;
                self.error_message = None;
            }
            Err(err) => {
                self.error_message = Some(format!("Error reading directory: {err}"));
                self.entries.clear();
            }
        }
        self.selection.clear();
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.nav.navigate(path);
        self.search.clear();
        self.refresh();
    }
}

impl eframe::App for FileExplorerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::toolbar::show(ctx, self);
        ui::side_panel::show(ctx, self);
        ui::status_bar::show(ctx, self);
        ui::file_list::show(ctx, self);
        ui::dialogs::show(ctx, self);
    }
}
