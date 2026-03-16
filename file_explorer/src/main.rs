mod app;
mod fs;
mod state;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 680.0])
            .with_min_inner_size([640.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "File Explorer",
        options,
        Box::new(|_cc| Ok(Box::new(app::FileExplorerApp::new()))),
    )
}
