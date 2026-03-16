use eframe::egui;

use crate::app::FileExplorerApp;
use crate::state::{SortColumn, SortOrder};

pub fn show(ctx: &egui::Context, app: &mut FileExplorerApp) {
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
        // Column headers
        let mut sort_changed = false;
        ui.horizontal(|ui: &mut egui::Ui| {
            let columns: [(& str, SortColumn, f32); 4] = [
                ("Name", SortColumn::Name, 300.0),
                ("Size", SortColumn::Size, 100.0),
                ("Type", SortColumn::Type, 100.0),
                ("Modified", SortColumn::Modified, 150.0),
            ];

            for (label, column, width) in columns {
                let arrow = if app.selection.sort_column == column {
                    match app.selection.sort_order {
                        SortOrder::Ascending => " \u{25B2}",
                        SortOrder::Descending => " \u{25BC}",
                    }
                } else {
                    ""
                };
                let text = format!("{label}{arrow}");
                let response =
                    ui.add_sized([width, 20.0], egui::Button::new(text).selected(false));
                if response.clicked() {
                    app.selection.toggle_sort(column);
                    sort_changed = true;
                }
            }
        });

        if sort_changed {
            app.selection.sort_entries(&mut app.entries);
        }

        ui.separator();

        // File list with scroll area
        let mut action: Option<ListAction> = None;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui: &mut egui::Ui| {
                let filtered: Vec<(usize, &crate::fs::entries::FileEntry)> = app
                    .entries
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| app.search.matches(e))
                    .collect();

                if filtered.is_empty() {
                    ui.centered_and_justified(|ui: &mut egui::Ui| {
                        if app.search.is_active {
                            ui.label("No matching files found");
                        } else if app.entries.is_empty() {
                            ui.label("Empty directory");
                        }
                    });
                }

                for (idx, entry) in &filtered {
                    let selected = app.selection.selected_indices.contains(idx);

                    ui.horizontal(|ui: &mut egui::Ui| {
                        let name_text = format!("{} {}", entry.icon(), entry.name);
                        let response = ui.add_sized(
                            [300.0, 20.0],
                            egui::Button::new(name_text).selected(selected),
                        );

                        ui.add_sized(
                            [100.0, 20.0],
                            egui::Label::new(&entry.formatted_size()),
                        );
                        ui.add_sized(
                            [100.0, 20.0],
                            egui::Label::new(&entry.file_type_label()),
                        );
                        ui.add_sized(
                            [150.0, 20.0],
                            egui::Label::new(&entry.formatted_date()),
                        );

                        if response.clicked() {
                            if ui.input(|i: &egui::InputState| i.modifiers.ctrl) {
                                app.selection.toggle(*idx);
                            } else {
                                app.selection.select_single(*idx);
                            }
                        }

                        if response.double_clicked() {
                            if entry.is_dir {
                                action = Some(ListAction::Navigate(entry.path.clone()));
                            } else {
                                action = Some(ListAction::OpenFile(entry.path.clone()));
                            }
                        }

                        // Context menu
                        response.context_menu(|ui: &mut egui::Ui| {
                            if !selected {
                                app.selection.select_single(*idx);
                            }
                            crate::ui::context_menu::show_for_entry(ui, &mut action, entry);
                        });
                    });
                }

                // Background context menu
                let bg_response = ui.interact(
                    ui.available_rect_before_wrap(),
                    ui.id().with("bg_ctx"),
                    egui::Sense::click(),
                );
                if bg_response.clicked() && !ui.input(|i: &egui::InputState| i.modifiers.ctrl) {
                    app.selection.clear();
                }
                bg_response.context_menu(|ui: &mut egui::Ui| {
                    crate::ui::context_menu::show_for_background(ui, &mut action);
                });
            });

        // Handle Enter key
        if ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter))
            && !app.selection.selected_indices.is_empty()
        {
            if let Some(&idx) = app.selection.selected_indices.iter().next() {
                if let Some(entry) = app.entries.get(idx) {
                    if entry.is_dir {
                        action = Some(ListAction::Navigate(entry.path.clone()));
                    } else {
                        action = Some(ListAction::OpenFile(entry.path.clone()));
                    }
                }
            }
        }

        // Handle Backspace for go up
        if ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Backspace)) {
            if app.nav.has_parent() {
                if let Some(_parent) = app.nav.go_up() {
                    app.search.clear();
                    app.refresh();
                }
            }
        }

        // Handle Delete key
        if ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Delete)) {
            if let Some(&idx) = app.selection.selected_indices.iter().next() {
                if let Some(entry) = app.entries.get(idx) {
                    action = Some(ListAction::Delete(
                        entry.name.clone(),
                        entry.path.clone(),
                    ));
                }
            }
        }

        // Apply deferred action
        match action {
            Some(ListAction::Navigate(path)) => app.navigate_to(path),
            Some(ListAction::OpenFile(path)) => {
                if let Err(e) = crate::fs::open::open_with_default_app(&path) {
                    app.error_message = Some(format!("Failed to open file: {e}"));
                }
            }
            Some(ListAction::NewFolder) => {
                app.dialog = Some(crate::app::DialogKind::NewFolder {
                    name: String::new(),
                });
            }
            Some(ListAction::Rename(name)) => {
                app.dialog = Some(crate::app::DialogKind::Rename {
                    original: name.clone(),
                    new_name: name,
                });
            }
            Some(ListAction::Delete(name, path)) => {
                app.dialog = Some(crate::app::DialogKind::DeleteConfirm {
                    target: name,
                    path,
                });
            }
            None => {}
        }
    });
}

pub enum ListAction {
    Navigate(std::path::PathBuf),
    OpenFile(std::path::PathBuf),
    NewFolder,
    Rename(String),
    Delete(String, std::path::PathBuf),
}
