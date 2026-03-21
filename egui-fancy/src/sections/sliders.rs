#[derive(Debug, Default)]
pub struct SlidersSection;

impl SlidersSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sliders & Inputs");
        ui.label("Coming soon...");
    }
}
