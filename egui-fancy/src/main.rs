mod animation;
mod app;
mod sections;
mod theme;

use app::FancyShowcaseApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 900.0])
            .with_title("egui Showcase")
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "egui Showcase",
        options,
        Box::new(|_cc| Ok(Box::new(FancyShowcaseApp::default()))),
    )
}
