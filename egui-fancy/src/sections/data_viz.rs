use crate::animation::{Animation, Easing};
use std::f32::consts::PI;

pub struct DataVizSection {
    // Chart controls
    pub frequency: f64,
    pub amplitude: f64,

    // Bar chart
    pub bar_data: Vec<f64>,
    pub bar_seed: u32,

    // Gauge
    pub gauge_target: f32,
    pub gauge_animation: Animation,
    pub gauge_current: f32,

    // Donut chart
    pub donut_values: [f32; 4],
    pub donut_animation: Animation,
}

impl Default for DataVizSection {
    fn default() -> Self {
        let mut section = Self {
            frequency: 2.0,
            amplitude: 1.0,
            bar_data: Vec::new(),
            bar_seed: 42,
            gauge_target: 0.65,
            gauge_animation: Animation::new(0.8, Easing::EaseOutElastic),
            gauge_current: 0.65,
            donut_values: [30.0, 25.0, 20.0, 25.0],
            donut_animation: Animation::new(0.6, Easing::EaseOutCubic),
        };
        section.regenerate_bars();
        section
    }
}

impl DataVizSection {
    fn regenerate_bars(&mut self) {
        let mut val = 0.0_f64;
        self.bar_data.clear();
        let mut seed = self.bar_seed;
        for _ in 0..20 {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let r = ((seed >> 16) & 0x7FFF) as f64 / 32768.0 - 0.5;
            val += r * 3.0;
            self.bar_data.push(val);
        }
        self.bar_seed = self.bar_seed.wrapping_add(1);
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Data Visualization");
        ui.add_space(8.0);
        ui.label("egui_plot charts with interactive controls, plus custom Painter-drawn visualizations.");
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui: &mut egui::Ui| {
            // Controls
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Frequency:");
                ui.add(egui::Slider::new(&mut self.frequency, 0.5..=8.0));
                ui.label("Amplitude:");
                ui.add(egui::Slider::new(&mut self.amplitude, 0.1..=3.0));
            });
            ui.add_space(8.0);

            // Top row: egui_plot charts
            ui.label(egui::RichText::new("egui_plot Charts").strong());
            ui.add_space(4.0);

            let chart_height = 200.0;
            ui.columns(3, |cols| {
                self.line_chart(&mut cols[0], chart_height);
                self.bar_chart(&mut cols[1], chart_height);
                self.area_chart(&mut cols[2], chart_height);
            });

            ui.add_space(16.0);

            // Bottom row: Painter-drawn
            ui.label(egui::RichText::new("Custom Painter Visualizations").strong());
            ui.add_space(4.0);

            ui.columns(3, |cols| {
                self.radial_gauge(&mut cols[0]);
                self.sparklines(&mut cols[1]);
                self.donut_chart(&mut cols[2]);
            });
        });
    }

    fn line_chart(&self, ui: &mut egui::Ui, height: f32) {
        ui.label("Line chart (sine waves):");
        let freq = self.frequency;
        let amp = self.amplitude;
        let plot = egui_plot::Plot::new("line_chart")
            .height(height)
            .legend(egui_plot::Legend::default());
        plot.show(ui, |plot_ui| {
            for (i, (name, phase)) in [("Wave A", 0.0), ("Wave B", 1.0), ("Wave C", 2.0)]
                .iter()
                .enumerate()
            {
                let color = [
                    egui::Color32::from_rgb(66, 133, 244),
                    egui::Color32::from_rgb(234, 67, 53),
                    egui::Color32::from_rgb(52, 168, 83),
                ][i];
                let points: egui_plot::PlotPoints = (0..200)
                    .map(|j| {
                        let x = j as f64 * 0.05;
                        let y = amp * (x * freq + phase).sin();
                        [x, y]
                    })
                    .collect();
                plot_ui.line(
                    egui_plot::Line::new(*name, points)
                        .color(color)
                        .width(2.0),
                );
            }
        });
    }

    fn bar_chart(&mut self, ui: &mut egui::Ui, height: f32) {
        ui.label("Bar chart (random walk):");
        if ui.button("Regenerate").clicked() {
            self.regenerate_bars();
        }
        let plot = egui_plot::Plot::new("bar_chart").height(height);
        let bars: Vec<egui_plot::Bar> = self
            .bar_data
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                egui_plot::Bar::new(i as f64, v).width(0.8).fill(
                    if v >= 0.0 {
                        egui::Color32::from_rgb(66, 133, 244)
                    } else {
                        egui::Color32::from_rgb(234, 67, 53)
                    },
                )
            })
            .collect();
        plot.show(ui, |plot_ui| {
            plot_ui.bar_chart(egui_plot::BarChart::new("Random Walk", bars));
        });
    }

    fn area_chart(&self, ui: &mut egui::Ui, height: f32) {
        ui.label("Area chart (stacked sines):");
        let freq = self.frequency;
        let amp = self.amplitude;
        let plot = egui_plot::Plot::new("area_chart")
            .height(height)
            .legend(egui_plot::Legend::default());
        plot.show(ui, |plot_ui| {
            for (i, (name, offset)) in
                [("Layer 1", 0.0_f64), ("Layer 2", 0.5), ("Layer 3", 1.0)]
                    .iter()
                    .enumerate()
            {
                let color = [
                    egui::Color32::from_rgba_premultiplied(66, 133, 244, 100),
                    egui::Color32::from_rgba_premultiplied(52, 168, 83, 100),
                    egui::Color32::from_rgba_premultiplied(251, 188, 4, 100),
                ][i];
                let points: egui_plot::PlotPoints = (0..200)
                    .map(|j| {
                        let x = j as f64 * 0.05;
                        let base: f64 = (0..=i)
                            .map(|k| amp * (x * freq * (1.0 + k as f64 * 0.3) + offset).sin().abs())
                            .sum();
                        [x, base]
                    })
                    .collect();
                plot_ui.line(
                    egui_plot::Line::new(*name, points)
                        .color(color)
                        .fill(0.0)
                        .width(1.5),
                );
            }
        });
    }

    fn radial_gauge(&mut self, ui: &mut egui::Ui) {
        ui.label("Radial gauge:");

        let prev_target = self.gauge_target;
        ui.add(egui::Slider::new(&mut self.gauge_target, 0.0..=1.0).text("target"));
        let ctx = ui.ctx().clone();
        if (self.gauge_target - prev_target).abs() > 0.001 {
            self.gauge_current = prev_target;
            self.gauge_animation.start(&ctx);
        }

        let progress = self.gauge_animation.progress(&ctx);
        let display_val = self.gauge_current + (self.gauge_target - self.gauge_current) * progress;

        let size = 140.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        let painter = ui.painter();
        let center = egui::pos2(rect.center().x, rect.center().y + 10.0);
        let radius = size / 2.0 - 12.0;

        // Arc background
        let start = PI * 0.8;
        let sweep = PI * 1.4;
        let arc_bg = if ui.visuals().dark_mode {
            egui::Color32::from_gray(80)
        } else {
            egui::Color32::from_gray(200)
        };
        draw_arc(painter, center, radius, start, start + sweep, 6.0, arc_bg);

        // Filled arc
        let fill_end = start + sweep * display_val;
        let fill_color = if display_val < 0.5 {
            egui::Color32::from_rgb(52, 168, 83)
        } else if display_val < 0.8 {
            egui::Color32::from_rgb(251, 188, 4)
        } else {
            egui::Color32::from_rgb(234, 67, 53)
        };
        draw_arc(painter, center, radius, start, fill_end, 6.0, fill_color);

        // Tick marks
        let tick_color = if ui.visuals().dark_mode {
            egui::Color32::from_gray(160)
        } else {
            egui::Color32::from_gray(140)
        };
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let angle = start + sweep * t;
            let inner = radius - 10.0;
            let outer = radius - 4.0;
            painter.line_segment(
                [
                    egui::pos2(center.x + angle.cos() * inner, center.y + angle.sin() * inner),
                    egui::pos2(center.x + angle.cos() * outer, center.y + angle.sin() * outer),
                ],
                egui::Stroke::new(1.5, tick_color),
            );
        }

        // Needle
        let needle_angle = start + sweep * display_val;
        let needle_len = radius - 18.0;
        let needle_end = egui::pos2(
            center.x + needle_angle.cos() * needle_len,
            center.y + needle_angle.sin() * needle_len,
        );
        painter.line_segment(
            [center, needle_end],
            egui::Stroke::new(2.5, ui.visuals().text_color()),
        );
        painter.circle_filled(center, 5.0, ui.visuals().text_color());

        // Value text
        painter.text(
            egui::pos2(center.x, center.y + 24.0),
            egui::Align2::CENTER_CENTER,
            format!("{:.0}", display_val * 100.0),
            egui::FontId::proportional(18.0),
            ui.visuals().text_color(),
        );
    }

    fn sparklines(&self, ui: &mut egui::Ui) {
        ui.label("Sparklines:");
        ui.add_space(4.0);

        let datasets: [&[f32]; 4] = [
            &[3.0, 5.0, 2.0, 8.0, 4.0, 7.0, 3.0, 6.0, 5.0, 9.0, 4.0, 6.0],
            &[1.0, 3.0, 7.0, 4.0, 6.0, 2.0, 8.0, 5.0, 3.0, 7.0, 9.0, 5.0],
            &[9.0, 7.0, 5.0, 6.0, 4.0, 3.0, 5.0, 2.0, 4.0, 3.0, 2.0, 1.0],
            &[2.0, 4.0, 3.0, 5.0, 8.0, 6.0, 7.0, 9.0, 8.0, 7.0, 8.0, 9.0],
        ];
        let colors = [
            egui::Color32::from_rgb(66, 133, 244),
            egui::Color32::from_rgb(234, 67, 53),
            egui::Color32::from_rgb(52, 168, 83),
            egui::Color32::from_rgb(251, 188, 4),
        ];
        let labels = ["Revenue", "Users", "Errors", "Growth"];

        for (i, data) in datasets.iter().enumerate() {
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label(egui::RichText::new(labels[i]).small());
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(100.0, 24.0), egui::Sense::hover());
                let painter = ui.painter();

                let min_val = data.iter().copied().reduce(f32::min).unwrap_or(0.0);
                let max_val = data.iter().copied().reduce(f32::max).unwrap_or(1.0);
                let range = (max_val - min_val).max(0.01);

                let points: Vec<egui::Pos2> = data
                    .iter()
                    .enumerate()
                    .map(|(j, &v)| {
                        let x = rect.left() + (j as f32 / (data.len() - 1) as f32) * rect.width();
                        let y = rect.bottom()
                            - ((v - min_val) / range) * rect.height();
                        egui::pos2(x, y)
                    })
                    .collect();

                for w in points.windows(2) {
                    painter.line_segment([w[0], w[1]], egui::Stroke::new(1.5, colors[i]));
                }
            });
        }
    }

    fn donut_chart(&mut self, ui: &mut egui::Ui) {
        ui.label("Donut chart:");

        let labels = ["Sales", "Marketing", "R&D", "Ops"];
        let colors = [
            egui::Color32::from_rgb(66, 133, 244),
            egui::Color32::from_rgb(234, 67, 53),
            egui::Color32::from_rgb(52, 168, 83),
            egui::Color32::from_rgb(251, 188, 4),
        ];

        let mut changed = false;
        ui.horizontal_wrapped(|ui: &mut egui::Ui| {
            for (i, label) in labels.iter().enumerate() {
                ui.colored_label(colors[i], *label);
                let old = self.donut_values[i];
                ui.add(egui::DragValue::new(&mut self.donut_values[i]).speed(0.5).range(1.0..=100.0));
                if (self.donut_values[i] - old).abs() > 0.01 {
                    changed = true;
                }
            }
        });
        if changed {
            self.donut_animation.start(ui.ctx());
        }

        let size = 120.0;
        let (rect, _) = ui.allocate_exact_size(egui::vec2(size, size), egui::Sense::hover());
        let painter = ui.painter();
        let center = rect.center();
        let outer_r = size / 2.0 - 4.0;
        let inner_r = outer_r * 0.55;

        let total: f32 = self.donut_values.iter().sum();
        if total <= 0.0 {
            return;
        }

        let mut angle = -PI / 2.0;
        for (i, &val) in self.donut_values.iter().enumerate() {
            let sweep = (val / total) * 2.0 * PI;
            draw_arc_thick(painter, center, inner_r, outer_r, angle, angle + sweep, colors[i]);
            angle += sweep;
        }

        // Center hole — match the panel background
        let bg = ui.visuals().panel_fill;
        painter.circle_filled(center, inner_r - 1.0, bg);
    }
}

fn draw_arc(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    start: f32,
    end: f32,
    width: f32,
    color: egui::Color32,
) {
    let n = 40;
    for i in 0..n {
        let t0 = i as f32 / n as f32;
        let t1 = (i + 1) as f32 / n as f32;
        let a0 = start + (end - start) * t0;
        let a1 = start + (end - start) * t1;
        painter.line_segment(
            [
                egui::pos2(center.x + a0.cos() * radius, center.y + a0.sin() * radius),
                egui::pos2(center.x + a1.cos() * radius, center.y + a1.sin() * radius),
            ],
            egui::Stroke::new(width, color),
        );
    }
}

fn draw_arc_thick(
    painter: &egui::Painter,
    center: egui::Pos2,
    inner_r: f32,
    outer_r: f32,
    start: f32,
    end: f32,
    color: egui::Color32,
) {
    let mid_r = (inner_r + outer_r) / 2.0;
    let width = outer_r - inner_r;
    draw_arc(painter, center, mid_r, start, end, width, color);
}
