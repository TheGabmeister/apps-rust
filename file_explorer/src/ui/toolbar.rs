use std::path::PathBuf;

use eframe::egui;

use crate::app::FileExplorerApp;

pub fn show(ctx: &egui::Context, app: &mut FileExplorerApp) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            // Back button
            if ui
                .add_enabled(!app.nav.back_stack.is_empty(), egui::Button::new("\u{2190}"))
                .on_hover_text("Back")
                .clicked()
            {
                let _ = app.nav.go_back();
                app.search.clear();
                app.refresh();
            }

            // Forward button
            if ui
                .add_enabled(
                    !app.nav.forward_stack.is_empty(),
                    egui::Button::new("\u{2192}"),
                )
                .on_hover_text("Forward")
                .clicked()
            {
                let _ = app.nav.go_forward();
                app.search.clear();
                app.refresh();
            }

            // Up button
            if ui
                .add_enabled(app.nav.has_parent(), egui::Button::new("\u{2191}"))
                .on_hover_text("Up")
                .clicked()
            {
                if let Some(_parent) = app.nav.go_up() {
                    app.search.clear();
                    app.refresh();
                }
            }

            // Refresh button
            if ui.button("\u{21BB}").on_hover_text("Refresh").clicked() {
                app.refresh();
            }

            ui.separator();

            // Breadcrumbs / address bar
            if app.nav.editing_address {
                let response = ui.text_edit_singleline(&mut app.nav.address_bar_text);
                if response.lost_focus() {
                    if ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter)) {
                        let path = PathBuf::from(&app.nav.address_bar_text);
                        if path.is_dir() {
                            app.navigate_to(path);
                        } else {
                            app.error_message = Some("Invalid directory path".to_string());
                            app.nav.address_bar_text =
                                app.nav.current_dir.display().to_string();
                        }
                    }
                    app.nav.editing_address = false;
                }
            } else {
                let crumbs = app.nav.breadcrumbs();
                let mut nav_target = None;
                for (i, (name, path)) in crumbs.iter().enumerate() {
                    if i > 0 {
                        ui.label("/");
                    }
                    if ui.selectable_label(false, name).clicked() {
                        nav_target = Some(path.clone());
                    }
                }
                if ui
                    .small_button("\u{270F}")
                    .on_hover_text("Edit path")
                    .clicked()
                {
                    app.nav.editing_address = true;
                    app.nav.address_bar_text = app.nav.current_dir.display().to_string();
                }
                if let Some(path) = nav_target {
                    app.navigate_to(path);
                }
            }

            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                |ui: &mut egui::Ui| {
                    ui.label("\u{1F50D}");
                    let search_response = ui.add(
                        egui::TextEdit::singleline(&mut app.search.query)
                            .hint_text("Search..."),
                    );
                    if search_response.changed() {
                        app.search.is_active = !app.search.query.is_empty();
                    }
                },
            );
        });
    });
}
