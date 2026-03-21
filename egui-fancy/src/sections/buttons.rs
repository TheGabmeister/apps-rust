use crate::animation::{Animation, Easing};

pub struct ButtonsSection {
    // Stock widget state
    pub toggle_on: bool,
    pub selectable_selected: bool,
    pub radio_value: usize,
    pub click_count: u32,

    // Custom widget state
    pub custom_toggle: bool,
    pub toggle_animation: Animation,
    pub button_group_active: usize,
    pub button_group_animation: Animation,
    pub button_group_prev: usize,

    // Ripple state
    pub ripple_center: egui::Pos2,
    pub ripple_animation: Animation,

    // Hover glow
    pub hover_glow_animation: Animation,
}

impl Default for ButtonsSection {
    fn default() -> Self {
        Self {
            toggle_on: false,
            selectable_selected: false,
            radio_value: 0,
            click_count: 0,
            custom_toggle: false,
            toggle_animation: Animation::new(0.5, Easing::EaseOutElastic),
            button_group_active: 0,
            button_group_animation: Animation::new(0.35, Easing::EaseOutCubic),
            button_group_prev: 0,
            ripple_center: egui::pos2(0.0, 0.0),
            ripple_animation: Animation::new(0.6, Easing::EaseOutCubic),
            hover_glow_animation: Animation::new(0.3, Easing::EaseOutCubic),
        }
    }
}

impl ButtonsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Buttons & Interactions");
        ui.add_space(8.0);
        ui.label("Stock egui widgets on the left, custom-painted enhancements on the right.");
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui: &mut egui::Ui| {
            ui.columns(2, |cols| {
                self.show_stock(&mut cols[0]);
                self.show_custom(&mut cols[1]);
            });
        });
    }

    fn show_stock(&mut self, ui: &mut egui::Ui) {
        ui.strong("Stock Widgets");
        ui.add_space(8.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("button / small_button:");
            ui.horizontal(|ui: &mut egui::Ui| {
                if ui.button("Click me").clicked() {
                    self.click_count += 1;
                }
                let _ = ui.small_button("Small");
                ui.label(format!("Clicks: {}", self.click_count));
            });
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("toggle_value:");
            ui.toggle_value(&mut self.toggle_on, "Toggle");
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("selectable_label:");
            if ui
                .selectable_label(self.selectable_selected, "Select me")
                .clicked()
            {
                self.selectable_selected = !self.selectable_selected;
            }
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("radio_value:");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.radio_value(&mut self.radio_value, 0, "Option A");
                ui.radio_value(&mut self.radio_value, 1, "Option B");
                ui.radio_value(&mut self.radio_value, 2, "Option C");
            });
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("hyperlink / link:");
            ui.hyperlink_to("egui on GitHub", "https://github.com/emilk/egui");
            let _ = ui.link("A clickable link");
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("Button with icon:");
            let _ = ui.button("\u{2764} Like");
            let _ = ui.button("\u{2b50} Star");
        });
    }

    fn show_custom(&mut self, ui: &mut egui::Ui) {
        ui.strong("Custom Enhancements");
        ui.add_space(8.0);

        // Hover glow button
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Hover glow:");
            let desired = egui::vec2(120.0, 32.0);
            let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
            let painter = ui.painter();

            let hovered = response.hovered();
            let ctx = ui.ctx().clone();
            if hovered && !self.hover_glow_animation.is_active(&ctx) {
                self.hover_glow_animation.start(&ctx);
            }
            let glow_progress = if hovered {
                self.hover_glow_animation.progress(&ctx)
            } else {
                0.0
            };

            // Glow behind
            if glow_progress > 0.0 {
                let glow_rect = rect.expand(4.0 * glow_progress);
                let alpha = (60.0 * glow_progress) as u8;
                painter.rect_filled(
                    glow_rect,
                    8.0,
                    egui::Color32::from_rgba_premultiplied(100, 149, 237, alpha),
                );
            }

            // Button face
            let fill = if response.is_pointer_button_down_on() {
                egui::Color32::from_rgb(60, 100, 180)
            } else if hovered {
                egui::Color32::from_rgb(80, 130, 210)
            } else {
                egui::Color32::from_rgb(70, 120, 200)
            };
            painter.rect_filled(rect, 6.0, fill);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Hover me",
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );
        });

        ui.add_space(4.0);

        // Click ripple button
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Click ripple:");
            let desired = egui::vec2(120.0, 32.0);
            let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
            let painter = ui.painter();
            let ctx = ui.ctx().clone();

            if response.clicked() && let Some(pos) = response.interact_pointer_pos() {
                self.ripple_center = pos;
                self.ripple_animation.start(&ctx);
            }

            // Button face
            painter.rect_filled(rect, 6.0, egui::Color32::from_rgb(70, 120, 200));

            // Ripple
            let ripple_progress = self.ripple_animation.progress(&ctx);
            if self.ripple_animation.is_active(&ctx) || ripple_progress > 0.0 {
                let max_radius = rect.size().length() * 0.7;
                let radius = max_radius * ripple_progress;
                let alpha = ((1.0 - ripple_progress) * 120.0) as u8;
                painter.circle_filled(
                    self.ripple_center,
                    radius,
                    egui::Color32::from_rgba_premultiplied(255, 255, 255, alpha),
                );
            }

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Click me",
                egui::FontId::proportional(14.0),
                egui::Color32::WHITE,
            );
        });

        ui.add_space(4.0);

        // Animated toggle switch
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Animated toggle:");
            let desired = egui::vec2(50.0, 26.0);
            let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
            let painter = ui.painter();
            let ctx = ui.ctx().clone();

            if response.clicked() {
                self.custom_toggle = !self.custom_toggle;
                if self.custom_toggle {
                    self.toggle_animation.start(&ctx);
                } else {
                    self.toggle_animation.reverse(&ctx);
                }
            }

            let progress = self.toggle_animation.progress(&ctx);

            // Track
            let on_color = egui::Color32::from_rgb(76, 175, 80);
            let off_color = egui::Color32::from_rgb(158, 158, 158);
            let track_color = lerp_color(off_color, on_color, progress);
            painter.rect_filled(rect, rect.height() / 2.0, track_color);

            // Thumb
            let thumb_radius = rect.height() / 2.0 - 3.0;
            let left_x = rect.left() + rect.height() / 2.0;
            let right_x = rect.right() - rect.height() / 2.0;
            let thumb_x = egui::lerp(left_x..=right_x, progress);
            painter.circle_filled(
                egui::pos2(thumb_x, rect.center().y),
                thumb_radius,
                egui::Color32::WHITE,
            );

            ui.label(if self.custom_toggle { "ON" } else { "OFF" });
        });

        ui.add_space(4.0);

        // Sliding button group
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Button group:");
            let labels = ["Alpha", "Beta", "Gamma"];
            let button_width = 70.0;
            let button_height = 28.0;
            let total_width = button_width * labels.len() as f32;
            let (rect, _) =
                ui.allocate_exact_size(egui::vec2(total_width, button_height), egui::Sense::hover());
            let painter = ui.painter();
            let ctx = ui.ctx().clone();

            let anim_progress = self.button_group_animation.progress(&ctx);

            // Sliding indicator
            let indicator_x = egui::lerp(
                (rect.left() + self.button_group_prev as f32 * button_width)
                    ..=(rect.left() + self.button_group_active as f32 * button_width),
                anim_progress,
            );
            let indicator_rect = egui::Rect::from_min_size(
                egui::pos2(indicator_x, rect.top()),
                egui::vec2(button_width, button_height),
            );
            painter.rect_filled(indicator_rect, 6.0, egui::Color32::from_rgb(70, 120, 200));

            // Buttons
            for (i, label) in labels.iter().enumerate() {
                let btn_rect = egui::Rect::from_min_size(
                    egui::pos2(rect.left() + i as f32 * button_width, rect.top()),
                    egui::vec2(button_width, button_height),
                );
                let btn_response = ui.interact(btn_rect, ui.id().with(("btngrp", i)), egui::Sense::click());
                if btn_response.clicked() && self.button_group_active != i {
                    self.button_group_prev = self.button_group_active;
                    self.button_group_active = i;
                    self.button_group_animation.start(&ctx);
                }

                let text_color = if self.button_group_active == i {
                    egui::Color32::WHITE
                } else {
                    ui.visuals().text_color()
                };
                painter.text(
                    btn_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    *label,
                    egui::FontId::proportional(13.0),
                    text_color,
                );
            }
        });
    }
}

pub fn lerp_color(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    egui::Color32::from_rgba_premultiplied(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
        (a.a() as f32 + (b.a() as f32 - a.a() as f32) * t) as u8,
    )
}
