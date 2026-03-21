#[derive(Debug, Default)]
pub struct ButtonsSection;

impl ButtonsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Buttons & Interactions");
        ui.label("Coming soon...");
    }
}
