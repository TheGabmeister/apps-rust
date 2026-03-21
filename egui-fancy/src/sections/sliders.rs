use crate::animation::{Animation, Easing};
use std::f32::consts::PI;

pub struct SlidersSection {
    // Stock widget state
    pub slider_h: f32,
    pub slider_v: f32,
    pub drag_value: f64,
    pub text_single: String,
    pub text_multi: String,
    pub color: egui::Color32,
    pub combo_selected: usize,
    pub checkbox_val: bool,

    // Custom widget state
    pub range_low: f32,
    pub range_high: f32,
    pub knob_value: f32,
    pub progress_value: f32,
    #[allow(dead_code)]
    pub progress_animation: Animation,
    pub focus_text: String,
    pub focus_animation: Animation,
    pub was_focused: bool,
}

impl Default for SlidersSection {
    fn default() -> Self {
        Self {
            slider_h: 50.0,
            slider_v: 0.5,
            drag_value: 42.0,
            text_single: "Hello".into(),
            text_multi: "Multi-line\ntext here".into(),
            color: egui::Color32::from_rgb(100, 150, 230),
            combo_selected: 0,
            checkbox_val: true,
            range_low: 20.0,
            range_high: 80.0,
            knob_value: 0.5,
            progress_value: 0.65,
            progress_animation: Animation::new(1.0, Easing::EaseOutCubic),
            focus_text: "Focus me".into(),
            focus_animation: Animation::new(0.3, Easing::EaseOutCubic),
            was_focused: false,
        }
    }
}

impl SlidersSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Sliders & Inputs");
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
            ui.label("Horizontal Slider:");
            ui.add(egui::Slider::new(&mut self.slider_h, 0.0..=100.0).text("value"));
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("Vertical Slider:");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.add(
                    egui::Slider::new(&mut self.slider_v, 0.0..=1.0)
                        .vertical()
                        .text("V"),
                );
            });
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("DragValue:");
            ui.add(egui::DragValue::new(&mut self.drag_value).speed(0.5));
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("TextEdit:");
            ui.add(egui::TextEdit::singleline(&mut self.text_single).hint_text("Single line"));
            ui.add(
                egui::TextEdit::multiline(&mut self.text_multi)
                    .hint_text("Multi line")
                    .desired_rows(3),
            );
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("Color picker:");
            let mut rgba = egui::Rgba::from(self.color);
            egui::color_picker::color_edit_button_rgba(
                ui,
                &mut rgba,
                egui::color_picker::Alpha::Opaque,
            );
            self.color = rgba.into();
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("ComboBox:");
            let items = ["Apple", "Banana", "Cherry", "Date"];
            egui::ComboBox::from_label("Pick fruit")
                .selected_text(items[self.combo_selected])
                .show_ui(ui, |ui: &mut egui::Ui| {
                    for (i, item) in items.iter().enumerate() {
                        ui.selectable_value(&mut self.combo_selected, i, *item);
                    }
                });
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("Checkbox:");
            ui.checkbox(&mut self.checkbox_val, "Check me");
        });

        ui.add_space(4.0);

        ui.group(|ui: &mut egui::Ui| {
            ui.label("Spinner:");
            ui.spinner();
        });
    }

    fn show_custom(&mut self, ui: &mut egui::Ui) {
        ui.strong("Custom Enhancements");
        ui.add_space(8.0);

        // Range slider
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Range slider:");
            self.range_slider(ui);
            ui.label(format!(
                "Range: {:.0} – {:.0}",
                self.range_low, self.range_high
            ));
        });

        ui.add_space(4.0);

        // Rotary knob
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Rotary knob:");
            self.rotary_knob(ui);
        });

        ui.add_space(4.0);

        // Gradient progress bar
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Gradient progress bar:");
            ui.add(egui::Slider::new(&mut self.progress_value, 0.0..=1.0).text("target"));
            self.gradient_progress_bar(ui, self.progress_value);
        });

        ui.add_space(4.0);

        // Focus glow text input
        ui.group(|ui: &mut egui::Ui| {
            ui.label("Focus-glow text input:");
            self.focus_glow_input(ui);
        });
    }

    fn range_slider(&mut self, ui: &mut egui::Ui) {
        let desired = egui::vec2(ui.available_width().min(300.0), 24.0);
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::drag());
        let painter = ui.painter();

        let track_y = rect.center().y;
        let track_left = rect.left() + 8.0;
        let track_right = rect.right() - 8.0;
        let track_width = track_right - track_left;

        // Track background
        painter.line_segment(
            [egui::pos2(track_left, track_y), egui::pos2(track_right, track_y)],
            egui::Stroke::new(4.0, egui::Color32::from_gray(100)),
        );

        // Filled track between thumbs
        let low_x = track_left + (self.range_low / 100.0) * track_width;
        let high_x = track_left + (self.range_high / 100.0) * track_width;
        painter.line_segment(
            [egui::pos2(low_x, track_y), egui::pos2(high_x, track_y)],
            egui::Stroke::new(4.0, egui::Color32::from_rgb(70, 150, 230)),
        );

        // Thumbs
        let thumb_radius = 8.0;
        painter.circle_filled(egui::pos2(low_x, track_y), thumb_radius, egui::Color32::from_rgb(50, 120, 200));
        painter.circle_filled(egui::pos2(high_x, track_y), thumb_radius, egui::Color32::from_rgb(50, 120, 200));
        painter.circle_stroke(egui::pos2(low_x, track_y), thumb_radius, egui::Stroke::new(1.5, egui::Color32::WHITE));
        painter.circle_stroke(egui::pos2(high_x, track_y), thumb_radius, egui::Stroke::new(1.5, egui::Color32::WHITE));

        // Drag logic
        if response.dragged() && let Some(pos) = response.interact_pointer_pos() {
            let val = ((pos.x - track_left) / track_width * 100.0).clamp(0.0, 100.0);
            let dist_low = (pos.x - low_x).abs();
            let dist_high = (pos.x - high_x).abs();
            if dist_low < dist_high {
                self.range_low = val.min(self.range_high - 1.0);
            } else {
                self.range_high = val.max(self.range_low + 1.0);
            }
        }
    }

    fn rotary_knob(&mut self, ui: &mut egui::Ui) {
        let size = 80.0;
        let (rect, response) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::drag());
        let painter = ui.painter();
        let center = rect.center();
        let radius = size / 2.0 - 4.0;

        // Outer circle
        painter.circle_stroke(center, radius, egui::Stroke::new(3.0, egui::Color32::from_gray(120)));

        // Arc indicator
        let start_angle = PI * 0.75;
        let end_angle = PI * 0.75 + PI * 1.5 * self.knob_value;
        let n_segments = 32;
        let points: Vec<egui::Pos2> = (0..=n_segments)
            .map(|i| {
                let t = i as f32 / n_segments as f32;
                let angle = start_angle + (end_angle - start_angle) * t;
                egui::pos2(
                    center.x + angle.cos() * (radius - 2.0),
                    center.y + angle.sin() * (radius - 2.0),
                )
            })
            .collect();
        if points.len() >= 2 {
            let stroke = egui::Stroke::new(5.0, egui::Color32::from_rgb(70, 150, 230));
            for i in 0..points.len() - 1 {
                painter.line_segment([points[i], points[i + 1]], stroke);
            }
        }

        // Needle line
        let needle_angle = start_angle + PI * 1.5 * self.knob_value;
        let needle_end = egui::pos2(
            center.x + needle_angle.cos() * (radius - 14.0),
            center.y + needle_angle.sin() * (radius - 14.0),
        );
        painter.line_segment(
            [center, needle_end],
            egui::Stroke::new(2.0, ui.visuals().text_color()),
        );

        // Center dot
        painter.circle_filled(center, 4.0, ui.visuals().text_color());

        // Value text
        painter.text(
            egui::pos2(center.x, center.y + radius + 12.0),
            egui::Align2::CENTER_CENTER,
            format!("{:.0}%", self.knob_value * 100.0),
            egui::FontId::proportional(12.0),
            ui.visuals().text_color(),
        );

        // Drag logic
        if response.dragged() {
            let delta = -response.drag_delta().y * 0.005;
            self.knob_value = (self.knob_value + delta).clamp(0.0, 1.0);
        }
    }

    fn gradient_progress_bar(&self, ui: &mut egui::Ui, value: f32) {
        let desired = egui::vec2(ui.available_width().min(300.0), 20.0);
        let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());
        let painter = ui.painter();

        // Background track
        painter.rect_filled(rect, rect.height() / 2.0, egui::Color32::from_gray(60));

        // Filled portion with gradient via mesh
        let fill_width = rect.width() * value.clamp(0.0, 1.0);
        if fill_width > 1.0 {
            let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(fill_width, rect.height()));
            let rounding = rect.height() / 2.0;

            let left_color = egui::Color32::from_rgb(66, 133, 244);
            let right_color = egui::Color32::from_rgb(139, 92, 246);

            // Use two rects to approximate gradient (left half, right half)
            let mid_x = fill_rect.center().x;
            let left_rect = egui::Rect::from_min_max(fill_rect.min, egui::pos2(mid_x, fill_rect.max.y));
            let right_rect = egui::Rect::from_min_max(egui::pos2(mid_x, fill_rect.min.y), fill_rect.max);

            painter.rect_filled(left_rect, rounding, left_color);
            if right_rect.width() > 0.0 {
                painter.rect_filled(right_rect, rounding, right_color);
            }
        }

        // Percentage text
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{:.0}%", value * 100.0),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }

    fn focus_glow_input(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        let response = ui.add(
            egui::TextEdit::singleline(&mut self.focus_text)
                .desired_width(200.0)
                .hint_text("Type here..."),
        );

        let focused = response.has_focus();
        if focused && !self.was_focused {
            self.focus_animation.start(&ctx);
        } else if !focused && self.was_focused {
            self.focus_animation.reverse(&ctx);
        }
        self.was_focused = focused;

        let progress = self.focus_animation.progress(&ctx);
        if progress > 0.01 {
            let glow_rect = response.rect.expand(3.0 * progress);
            let alpha = (80.0 * progress) as u8;
            ui.painter().rect_stroke(
                glow_rect,
                4.0,
                egui::Stroke::new(2.0, egui::Color32::from_rgba_premultiplied(100, 149, 237, alpha)),
                egui::StrokeKind::Outside,
            );
        }
    }
}
