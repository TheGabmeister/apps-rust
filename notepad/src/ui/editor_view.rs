use eframe::egui::{ScrollArea, TextEdit, TextStyle, Ui};

use crate::editor::EditorState;

pub fn render_editor(ui: &mut Ui, editor: &mut EditorState) -> bool {
    let available_size = ui.available_size();

    ScrollArea::vertical()
        .show(ui, |ui| {
            let output = TextEdit::multiline(&mut editor.content)
                .font(TextStyle::Monospace)
                .desired_width(available_size.x)
                .lock_focus(true)
                .show(ui);

            if let Some(cursor_range) = output.cursor_range {
                let char_index = cursor_range.primary.index;
                editor.update_cursor_position(char_index);
            }

            output.response.changed()
        })
        .inner
}
