use eframe::egui;

use calculator::app::CalculatorApp;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 420.0])
            .with_min_inner_size([280.0, 380.0])
            .with_title("Calculator"),
        ..Default::default()
    };

    eframe::run_native(
        "Calculator",
        native_options,
        Box::new(|cc| Ok(Box::new(CalculatorApp::new(cc)))),
    )
}
