mod app;
mod page_cache;
mod pdf_document;
mod recent_files;
mod toolbar;
mod viewer;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("PDF Reader"),
        ..Default::default()
    };

    eframe::run_native(
        "PDF Reader",
        options,
        Box::new(|cc| Ok(Box::new(app::PdfReaderApp::new(cc)))),
    )
}
