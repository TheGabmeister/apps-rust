#[derive(Debug, Default)]
pub struct DataVizSection;

impl DataVizSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Visualization");
        ui.label("Coming soon...");
    }
}
