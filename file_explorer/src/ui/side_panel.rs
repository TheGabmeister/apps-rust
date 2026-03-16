use eframe::egui;

use crate::app::FileExplorerApp;
use crate::fs::bookmarks;

pub fn show(ctx: &egui::Context, app: &mut FileExplorerApp) {
    egui::SidePanel::left("bookmarks")
        .default_width(160.0)
        .show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Quick Access");
            ui.separator();

            let mut nav_target = None;

            for bookmark in &bookmarks::default_bookmarks() {
                if let Some(ref path) = bookmark.path {
                    let selected = app.nav.current_dir == *path;
                    let label = format!("{} {}", bookmark.icon, bookmark.label);
                    if ui.selectable_label(selected, label).clicked() {
                        nav_target = Some(path.clone());
                    }
                }
            }

            if let Some(path) = nav_target {
                app.navigate_to(path);
            }

            // Drive roots on Windows
            #[cfg(target_os = "windows")]
            {
                ui.separator();
                ui.heading("Drives");
                let mut drive_target = None;
                for letter in b'A'..=b'Z' {
                    let drive = format!("{}:\\", letter as char);
                    let path = std::path::PathBuf::from(&drive);
                    if path.exists() {
                        let selected = app.nav.current_dir.starts_with(&path);
                        if ui
                            .selectable_label(selected, format!("\u{1F4BF} {drive}"))
                            .clicked()
                        {
                            drive_target = Some(path);
                        }
                    }
                }
                if let Some(path) = drive_target {
                    app.navigate_to(path);
                }
            }
        });
}
