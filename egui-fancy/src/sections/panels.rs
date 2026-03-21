#[derive(Debug, Default)]
pub struct PanelsSection;

impl PanelsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Panels & Navigation");
        ui.label("Coming soon...");
    }
}
