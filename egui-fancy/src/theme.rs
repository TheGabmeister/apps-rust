pub fn apply_theme(ctx: &egui::Context, is_dark: bool) {
    let visuals = if is_dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    ctx.set_visuals(visuals);
}
