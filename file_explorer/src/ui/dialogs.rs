use eframe::egui;

use crate::app::{DialogKind, FileExplorerApp};

pub fn show(ctx: &egui::Context, app: &mut FileExplorerApp) {
    let dialog = match app.dialog.take() {
        Some(d) => d,
        None => return,
    };

    match dialog {
        DialogKind::NewFolder { mut name } => {
            let mut keep_open = true;
            egui::Window::new("New Folder")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui: &mut egui::Ui| {
                    ui.label("Folder name:");
                    let response = ui.text_edit_singleline(&mut name);

                    if name.is_empty() {
                        response.request_focus();
                    }

                    ui.horizontal(|ui: &mut egui::Ui| {
                        let can_create = !name.is_empty();
                        if ui
                            .add_enabled(can_create, egui::Button::new("Create"))
                            .clicked()
                            || (response.lost_focus()
                                && ui.input(|i: &egui::InputState| {
                                    i.key_pressed(egui::Key::Enter)
                                })
                                && can_create)
                        {
                            match crate::fs::operations::create_folder(
                                &app.nav.current_dir,
                                &name,
                            ) {
                                Ok(_) => {
                                    app.refresh();
                                    keep_open = false;
                                }
                                Err(e) => {
                                    app.error_message = Some(e.to_string());
                                    keep_open = false;
                                }
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            keep_open = false;
                        }
                    });
                });
            if keep_open {
                app.dialog = Some(DialogKind::NewFolder { name });
            }
        }

        DialogKind::Rename {
            original,
            mut new_name,
        } => {
            let mut keep_open = true;
            egui::Window::new("Rename")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui: &mut egui::Ui| {
                    ui.label(format!("Rename \"{original}\" to:"));
                    let response = ui.text_edit_singleline(&mut new_name);

                    ui.horizontal(|ui: &mut egui::Ui| {
                        let can_rename = !new_name.is_empty() && new_name != original;
                        if ui
                            .add_enabled(can_rename, egui::Button::new("Rename"))
                            .clicked()
                            || (response.lost_focus()
                                && ui.input(|i: &egui::InputState| {
                                    i.key_pressed(egui::Key::Enter)
                                })
                                && can_rename)
                        {
                            let from = app.nav.current_dir.join(&original);
                            match crate::fs::operations::rename_entry(&from, &new_name) {
                                Ok(_) => {
                                    app.refresh();
                                    keep_open = false;
                                }
                                Err(e) => {
                                    app.error_message = Some(e.to_string());
                                    keep_open = false;
                                }
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            keep_open = false;
                        }
                    });
                });
            if keep_open {
                app.dialog = Some(DialogKind::Rename {
                    original,
                    new_name,
                });
            }
        }

        DialogKind::DeleteConfirm { target, path } => {
            let mut keep_open = true;
            egui::Window::new("Confirm Delete")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui: &mut egui::Ui| {
                    ui.label(format!(
                        "Are you sure you want to delete \"{target}\"?"
                    ));
                    ui.label("This action cannot be undone.");
                    ui.add_space(8.0);
                    ui.horizontal(|ui: &mut egui::Ui| {
                        if ui.button("Delete").clicked() {
                            match crate::fs::operations::delete_entry(&path) {
                                Ok(()) => {
                                    app.refresh();
                                    keep_open = false;
                                }
                                Err(e) => {
                                    app.error_message = Some(e.to_string());
                                    keep_open = false;
                                }
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            keep_open = false;
                        }
                    });
                });
            if keep_open {
                app.dialog = Some(DialogKind::DeleteConfirm { target, path });
            }
        }
    }
}
