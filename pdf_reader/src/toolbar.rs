use crate::app::{PdfReaderApp, ViewMode, ZoomMode};

pub fn render_toolbar(app: &mut PdfReaderApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("toolbar").show(ctx, |ui: &mut egui::Ui| {
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.spacing_mut().item_spacing.x = 4.0;

            // --- File section ---
            if ui.button("Open").clicked() {
                app.open_file_dialog(ctx);
            }

            let recent_list: Vec<std::path::PathBuf> = app.recent_files.list().to_vec();
            if !recent_list.is_empty() {
                egui::ComboBox::from_id_salt("recent_files")
                    .selected_text("Recent")
                    .width(120.0)
                    .show_ui(ui, |ui: &mut egui::Ui| {
                        for path in &recent_list {
                            let label = path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            if ui
                                .selectable_label(false, &label)
                                .on_hover_text(path.display().to_string())
                                .clicked()
                            {
                                app.pending_open_path = Some(path.clone());
                            }
                        }
                    });
            }

            ui.separator();

            // --- Navigation section ---
            let has_doc = app.pdf_doc.is_some();

            ui.add_enabled_ui(has_doc, |ui: &mut egui::Ui| {
                if ui.button("|<").on_hover_text("First page (Home)").clicked() {
                    app.first_page();
                }
                if ui
                    .button("<")
                    .on_hover_text("Previous page (PageUp)")
                    .clicked()
                {
                    app.prev_page();
                }

                let response = ui.add_sized(
                    [50.0, 18.0],
                    egui::TextEdit::singleline(&mut app.page_jump_input)
                        .desired_width(40.0)
                        .horizontal_align(egui::Align::Center),
                );
                if response.lost_focus()
                    && ui.input(|i: &egui::InputState| i.key_pressed(egui::Key::Enter))
                {
                    if let Ok(page_num) = app.page_jump_input.parse::<usize>() {
                        app.go_to_page(page_num.saturating_sub(1));
                    }
                }

                ui.label(format!("/ {}", app.total_pages));

                if ui
                    .button(">")
                    .on_hover_text("Next page (PageDown)")
                    .clicked()
                {
                    app.next_page();
                }
                if ui.button(">|").on_hover_text("Last page (End)").clicked() {
                    app.last_page();
                }
            });

            ui.separator();

            // --- Zoom section ---
            ui.add_enabled_ui(has_doc, |ui: &mut egui::Ui| {
                if ui.button("-").on_hover_text("Zoom out (Ctrl+-)").clicked() {
                    app.zoom_out();
                }

                ui.label(format!("{:.0}%", app.zoom_percent));

                if ui.button("+").on_hover_text("Zoom in (Ctrl++)").clicked() {
                    app.zoom_in();
                }

                if ui
                    .selectable_label(app.zoom_mode == ZoomMode::FitWidth, "Fit Width")
                    .clicked()
                {
                    app.set_zoom(ZoomMode::FitWidth, None);
                }
                if ui
                    .selectable_label(app.zoom_mode == ZoomMode::FitPage, "Fit Page")
                    .clicked()
                {
                    app.set_zoom(ZoomMode::FitPage, None);
                }
            });

            ui.separator();

            // --- View mode section ---
            ui.add_enabled_ui(has_doc, |ui: &mut egui::Ui| {
                if ui
                    .selectable_label(app.view_mode == ViewMode::SinglePage, "Single")
                    .clicked()
                {
                    app.view_mode = ViewMode::SinglePage;
                }
                if ui
                    .selectable_label(app.view_mode == ViewMode::ContinuousScroll, "Scroll")
                    .clicked()
                {
                    app.view_mode = ViewMode::ContinuousScroll;
                }
            });
        });
    });
}
