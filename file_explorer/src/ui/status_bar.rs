use eframe::egui;

use crate::app::FileExplorerApp;

pub fn show(ctx: &egui::Context, app: &mut FileExplorerApp) {
    egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            let total = app.entries.len();
            let selected_count = app.selection.selected_indices.len();

            ui.label(format!("{total} items"));

            if selected_count > 0 {
                ui.separator();
                ui.label(format!("{selected_count} selected"));

                let total_size: u64 = app
                    .selection
                    .selected_indices
                    .iter()
                    .filter_map(|&i| app.entries.get(i))
                    .filter(|e| !e.is_dir)
                    .map(|e| e.size)
                    .sum();

                if total_size > 0 {
                    ui.separator();
                    ui.label(format_size(total_size));
                }
            }

            if let Some(ref err) = app.error_message {
                ui.separator();
                ui.colored_label(egui::Color32::from_rgb(255, 100, 100), err);
            }
        });
    });
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    match bytes {
        s if s >= GB => format!("{:.1} GB", s as f64 / GB as f64),
        s if s >= MB => format!("{:.1} MB", s as f64 / MB as f64),
        s if s >= KB => format!("{:.1} KB", s as f64 / KB as f64),
        s => format!("{s} B"),
    }
}
