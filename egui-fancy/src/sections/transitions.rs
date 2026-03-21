#[derive(Debug, Default)]
pub struct TransitionsSection;

impl TransitionsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Animated Transitions");
        ui.label("Coming soon...");
    }
}
