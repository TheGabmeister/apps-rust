#[derive(Debug, Default)]
pub struct DashboardSection;

impl DashboardSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard Grid");
        ui.label("Coming soon...");
    }
}
