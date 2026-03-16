use eframe::egui::{self, Ui};

use crate::editor::EditorState;

pub fn render_status_bar(ui: &mut Ui, editor: &EditorState) {
    ui.horizontal(|ui| {
        let path_text = editor
            .file_path
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "Untitled".to_string());
        ui.label(&path_text);

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(format!("Ln {}, Col {}", editor.cursor_line, editor.cursor_col));
            if editor.dirty {
                ui.separator();
                ui.label("Modified");
            }
        });
    });
}
