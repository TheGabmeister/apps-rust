use crate::app::{PdfReaderApp, ViewMode};
use egui::{Image, ScrollArea, TextureHandle, Vec2, load::SizedTexture};

pub fn render_viewer(app: &mut PdfReaderApp, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
        if let Some(ref err) = app.error_message {
            ui.colored_label(egui::Color32::RED, err);
            ui.separator();
        }

        if app.pdf_doc.is_none() {
            ui.centered_and_justified(|ui: &mut egui::Ui| {
                ui.heading("Open a PDF file to begin (Ctrl+O)");
            });
            return;
        }

        let available_width = ui.available_width();
        let available_height = ui.available_height();
        let render_width = app.compute_render_width(available_width, available_height);

        match app.view_mode {
            ViewMode::SinglePage => {
                render_single_page(app, ctx, ui, render_width, available_width);
            }
            ViewMode::ContinuousScroll => {
                render_continuous(app, ctx, ui, render_width, available_width);
            }
        }
    });
}

fn render_single_page(
    app: &mut PdfReaderApp,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    render_width: u32,
    available_width: f32,
) {
    let page_index = app.current_page;
    let texture = ensure_page_texture(app, ctx, page_index, render_width);

    if let Some(tex) = texture {
        let tex_size = tex.size_vec2();

        ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui: &mut egui::Ui| {
                let padding = ((available_width - tex_size.x) / 2.0).max(0.0);
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_space(padding);
                    ui.add(
                        Image::new(SizedTexture::new(tex.id(), tex_size))
                            .fit_to_exact_size(tex_size),
                    );
                });
            });
    }

    // Pre-render adjacent pages
    if page_index > 0 {
        let _ = ensure_page_texture(app, ctx, page_index - 1, render_width);
    }
    if page_index + 1 < app.total_pages {
        let _ = ensure_page_texture(app, ctx, page_index + 1, render_width);
    }

    app.page_cache.evict_distant(page_index, 3);
}

fn render_continuous(
    app: &mut PdfReaderApp,
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    render_width: u32,
    available_width: f32,
) {
    let page_spacing = 12.0;

    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui: &mut egui::Ui| {
            for page_index in 0..app.total_pages {
                let texture = ensure_page_texture(app, ctx, page_index, render_width);

                if let Some(tex) = texture {
                    let tex_size = tex.size_vec2();
                    let padding = ((available_width - tex_size.x) / 2.0).max(0.0);

                    ui.horizontal(|ui: &mut egui::Ui| {
                        ui.add_space(padding);
                        ui.add(
                            Image::new(SizedTexture::new(tex.id(), tex_size))
                                .fit_to_exact_size(tex_size),
                        );
                    });
                } else {
                    let aspect = app
                        .pdf_doc
                        .as_ref()
                        .and_then(|d| d.page_aspect_ratio(page_index).ok())
                        .unwrap_or(0.7727);
                    let height = render_width as f32 / aspect;
                    ui.allocate_space(Vec2::new(render_width as f32, height));
                }

                ui.add_space(page_spacing);
            }
        });
}

fn ensure_page_texture(
    app: &mut PdfReaderApp,
    ctx: &egui::Context,
    page_index: usize,
    render_width: u32,
) -> Option<TextureHandle> {
    if let Some(tex) = app.page_cache.get(page_index, render_width) {
        return Some(tex.clone());
    }

    if let Some(ref doc) = app.pdf_doc {
        match doc.render_page(page_index, render_width) {
            Ok((w, h, rgba)) => {
                let tex = app
                    .page_cache
                    .insert(page_index, render_width, ctx, w, h, rgba);
                Some(tex.clone())
            }
            Err(e) => {
                app.error_message = Some(format!("Page {}: {}", page_index + 1, e));
                None
            }
        }
    } else {
        None
    }
}
