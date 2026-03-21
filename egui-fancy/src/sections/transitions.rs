use crate::animation::{Animation, Easing};
use crate::sections::buttons::lerp_color;

/// Section 6: Animated Transitions
///
/// - Transition demos: fade, slide (H+V), scale, fade+slide combo with Play/Reverse
/// - Easing curve visualizer: line graph per curve, animated dot, moving square
/// - Before/after comparison: instant vs animated state change

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransitionKind {
    Fade,
    SlideH,
    SlideV,
    Scale,
    FadeSlide,
}

impl TransitionKind {
    const ALL: &[TransitionKind] = &[
        TransitionKind::Fade,
        TransitionKind::SlideH,
        TransitionKind::SlideV,
        TransitionKind::Scale,
        TransitionKind::FadeSlide,
    ];

    fn label(self) -> &'static str {
        match self {
            TransitionKind::Fade => "Fade",
            TransitionKind::SlideH => "Slide H",
            TransitionKind::SlideV => "Slide V",
            TransitionKind::Scale => "Scale",
            TransitionKind::FadeSlide => "Fade + Slide",
        }
    }
}

pub struct TransitionsSection {
    // Transition demos
    transition_anims: Vec<Animation>,

    // Easing visualizer
    selected_easing: usize,
    easing_anim: Animation, // Always Linear — we apply selected easing manually

    // Before/after
    before_after_animated: bool,
    before_after_anim: Animation,
    before_after_state: bool,
}

impl Default for TransitionsSection {
    fn default() -> Self {
        let transition_anims = TransitionKind::ALL
            .iter()
            .map(|_| Animation::new(0.8, Easing::EaseInOutCubic))
            .collect();

        Self {
            transition_anims,
            selected_easing: 0,
            easing_anim: Animation::new(1.5, Easing::Linear), // Always Linear for raw time
            before_after_animated: true,
            before_after_anim: Animation::new(0.6, Easing::EaseOutCubic),
            before_after_state: false,
        }
    }
}

impl std::fmt::Debug for TransitionsSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TransitionsSection").finish()
    }
}

impl TransitionsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Animated Transitions");
        ui.add_space(4.0);
        ui.label("Showcases the animation system with transition patterns and easing curve visualizations.");
        ui.add_space(12.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.show_transition_demos(ui);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(12.0);
            self.show_easing_visualizer(ui);
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(12.0);
            self.show_before_after(ui);
            ui.add_space(20.0);
        });
    }

    fn show_transition_demos(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Transition Demos").strong().size(16.0));
        ui.add_space(4.0);
        ui.label("Each demo has Play and Reverse buttons to trigger the animation.");
        ui.add_space(8.0);

        let ctx = ui.ctx().clone();
        let accent = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(100, 149, 237)
        } else {
            egui::Color32::from_rgb(50, 100, 200)
        };
        let card_size = egui::vec2(80.0, 50.0);
        let demo_width = ui.available_width().min(900.0);
        let col_width = demo_width / TransitionKind::ALL.len() as f32;

        // Labels row
        ui.horizontal(|ui| {
            for kind in TransitionKind::ALL {
                ui.allocate_ui(egui::vec2(col_width, 18.0), |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.label(egui::RichText::new(kind.label()).strong());
                    });
                });
            }
        });

        ui.add_space(4.0);

        // Animation display row
        let (response, painter) = ui.allocate_painter(
            egui::vec2(demo_width, 80.0),
            egui::Sense::hover(),
        );
        let base_rect = response.rect;

        for (i, kind) in TransitionKind::ALL.iter().enumerate() {
            let progress = self.transition_anims[i].progress(&ctx);
            let center_x = base_rect.left() + (i as f32 + 0.5) * col_width;
            let center_y = base_rect.center().y;

            let card_center = egui::pos2(center_x, center_y);

            match kind {
                TransitionKind::Fade => {
                    let alpha = (progress * 255.0) as u8;
                    let color = egui::Color32::from_rgba_unmultiplied(
                        accent.r(), accent.g(), accent.b(), alpha,
                    );
                    let rect = egui::Rect::from_center_size(card_center, card_size);
                    painter.rect_filled(rect, 6.0, color);
                    if progress > 0.3 {
                        painter.text(
                            rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Fade",
                            egui::FontId::proportional(12.0),
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                        );
                    }
                }
                TransitionKind::SlideH => {
                    let offset_x = (1.0 - progress) * col_width * 0.4;
                    let rect = egui::Rect::from_center_size(
                        egui::pos2(center_x - offset_x, center_y),
                        card_size,
                    );
                    painter.rect_filled(rect, 6.0, accent);
                    painter.text(
                        rect.center(), egui::Align2::CENTER_CENTER, "Slide H",
                        egui::FontId::proportional(12.0), egui::Color32::WHITE,
                    );
                }
                TransitionKind::SlideV => {
                    let offset_y = (1.0 - progress) * 40.0;
                    let rect = egui::Rect::from_center_size(
                        egui::pos2(center_x, center_y + offset_y),
                        card_size,
                    );
                    painter.rect_filled(rect, 6.0, accent);
                    painter.text(
                        rect.center(), egui::Align2::CENTER_CENTER, "Slide V",
                        egui::FontId::proportional(12.0), egui::Color32::WHITE,
                    );
                }
                TransitionKind::Scale => {
                    let scale = progress;
                    let size = egui::vec2(card_size.x * scale, card_size.y * scale);
                    if scale > 0.01 {
                        let rect = egui::Rect::from_center_size(card_center, size);
                        painter.rect_filled(rect, 6.0 * scale, accent);
                        if scale > 0.4 {
                            painter.text(
                                rect.center(), egui::Align2::CENTER_CENTER, "Scale",
                                egui::FontId::proportional(12.0 * scale), egui::Color32::WHITE,
                            );
                        }
                    }
                }
                TransitionKind::FadeSlide => {
                    let alpha = (progress * 255.0) as u8;
                    let offset_y = (1.0 - progress) * 30.0;
                    let color = egui::Color32::from_rgba_unmultiplied(
                        accent.r(), accent.g(), accent.b(), alpha,
                    );
                    let rect = egui::Rect::from_center_size(
                        egui::pos2(center_x, center_y + offset_y),
                        card_size,
                    );
                    painter.rect_filled(rect, 6.0, color);
                    if progress > 0.3 {
                        painter.text(
                            rect.center(), egui::Align2::CENTER_CENTER, "Combo",
                            egui::FontId::proportional(12.0),
                            egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha),
                        );
                    }
                }
            }
        }

        ui.add_space(4.0);

        // Buttons row
        ui.horizontal(|ui| {
            for (i, _kind) in TransitionKind::ALL.iter().enumerate() {
                ui.allocate_ui(egui::vec2(col_width, 24.0), |ui| {
                    ui.horizontal(|ui| {
                        if ui.small_button("\u{25b6}").on_hover_text("Play").clicked() {
                            self.transition_anims[i].start(&ctx);
                        }
                        if ui.small_button("\u{25c0}").on_hover_text("Reverse").clicked() {
                            self.transition_anims[i].reverse(&ctx);
                        }
                    });
                });
            }
        });
    }

    fn show_easing_visualizer(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        ui.label(egui::RichText::new("Easing Curve Visualizer").strong().size(16.0));
        ui.add_space(4.0);

        ui.horizontal(|ui| {
            ui.label("Select easing:");
            for (i, easing) in Easing::ALL.iter().enumerate() {
                if ui.selectable_label(self.selected_easing == i, easing.label()).clicked() {
                    self.selected_easing = i;
                }
            }
        });

        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if ui.button("\u{25b6} Play").clicked() {
                self.easing_anim.start(&ctx);
            }
            if ui.button("\u{25c0} Reverse").clicked() {
                self.easing_anim.reverse(&ctx);
            }
        });

        ui.add_space(8.0);

        // Draw all 6 easing curves in a 3x2 grid
        let available_width = ui.available_width().min(900.0);
        let graph_width = (available_width - 24.0) / 3.0;
        let graph_height = 100.0;

        let selected_easing = Easing::ALL[self.selected_easing];
        let anim_progress_raw = self.get_raw_time_progress(&ctx);

        for row in 0..2 {
            ui.horizontal(|ui| {
                for col in 0..3 {
                    let idx = row * 3 + col;
                    if idx >= Easing::ALL.len() {
                        break;
                    }
                    let easing = Easing::ALL[idx];
                    let is_selected = easing == selected_easing;

                    ui.vertical(|ui| {
                        ui.set_min_width(graph_width);

                        // Label
                        let label_text = egui::RichText::new(easing.label()).size(11.0);
                        let label_text = if is_selected { label_text.strong() } else { label_text };
                        ui.label(label_text);

                        // Graph
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(graph_width, graph_height),
                            egui::Sense::hover(),
                        );
                        let rect = response.rect;

                        // Background
                        let bg = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(28, 28, 38)
                        } else {
                            egui::Color32::from_rgb(245, 245, 252)
                        };
                        painter.rect_filled(rect, 4.0, bg);

                        // Border highlight for selected
                        if is_selected {
                            let stroke_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(100, 149, 237)
                            } else {
                                egui::Color32::from_rgb(50, 100, 200)
                            };
                            painter.rect_stroke(
                                rect, 4.0,
                                egui::Stroke::new(1.5, stroke_color),
                                egui::StrokeKind::Outside,
                            );
                        }

                        // Draw curve
                        let curve_color = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(140, 180, 255)
                        } else {
                            egui::Color32::from_rgb(60, 100, 200)
                        };
                        let margin = 8.0;
                        let plot_rect = rect.shrink(margin);
                        let steps = 60;
                        let points: Vec<egui::Pos2> = (0..=steps)
                            .map(|s| {
                                let t = s as f32 / steps as f32;
                                let y = easing.apply(t);
                                egui::pos2(
                                    plot_rect.left() + t * plot_rect.width(),
                                    plot_rect.bottom() - y * plot_rect.height(),
                                )
                            })
                            .collect();

                        for pair in points.windows(2) {
                            painter.line_segment(
                                [pair[0], pair[1]],
                                egui::Stroke::new(1.5, curve_color),
                            );
                        }

                        // Animated dot (only for selected easing)
                        if is_selected && anim_progress_raw >= 0.0 {
                            let t = anim_progress_raw.clamp(0.0, 1.0);
                            let y = easing.apply(t);
                            let dot_pos = egui::pos2(
                                plot_rect.left() + t * plot_rect.width(),
                                plot_rect.bottom() - y * plot_rect.height(),
                            );
                            let dot_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(255, 120, 80)
                            } else {
                                egui::Color32::from_rgb(220, 60, 30)
                            };
                            painter.circle_filled(dot_pos, 4.0, dot_color);
                        }

                        // Moving square below graph
                        let square_size = 14.0;
                        let track_width = graph_width - square_size;

                        let (sq_resp, sq_painter) = ui.allocate_painter(
                            egui::vec2(graph_width, square_size + 4.0),
                            egui::Sense::hover(),
                        );
                        let sq_rect = sq_resp.rect;

                        // Track line
                        let track_color = if ui.visuals().dark_mode {
                            egui::Color32::from_rgb(50, 50, 65)
                        } else {
                            egui::Color32::from_rgb(210, 210, 220)
                        };
                        sq_painter.line_segment(
                            [
                                egui::pos2(sq_rect.left() + square_size / 2.0, sq_rect.center().y),
                                egui::pos2(sq_rect.right() - square_size / 2.0, sq_rect.center().y),
                            ],
                            egui::Stroke::new(2.0, track_color),
                        );

                        // Square position based on this easing's curve, driven by the shared anim time
                        if is_selected && anim_progress_raw >= 0.0 {
                            let t = anim_progress_raw.clamp(0.0, 1.0);
                            let eased = easing.apply(t);
                            let sq_x = sq_rect.left() + eased * track_width;
                            let sq = egui::Rect::from_min_size(
                                egui::pos2(sq_x, sq_rect.top() + 2.0),
                                egui::vec2(square_size, square_size),
                            );
                            let sq_color = if ui.visuals().dark_mode {
                                egui::Color32::from_rgb(100, 149, 237)
                            } else {
                                egui::Color32::from_rgb(50, 100, 200)
                            };
                            sq_painter.rect_filled(sq, 3.0, sq_color);
                        }

                        ui.add_space(2.0);
                    });
                }
            });
            ui.add_space(8.0);
        }
    }

    fn show_before_after(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        ui.label(egui::RichText::new("Before / After Comparison").strong().size(16.0));
        ui.add_space(4.0);
        ui.label("Compare instant state change vs animated transition.");
        ui.add_space(8.0);

        let progress = self.before_after_anim.progress(&ctx);

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.before_after_animated, "Use animation");
            if ui.button("Toggle State").clicked() {
                self.before_after_state = !self.before_after_state;
                if self.before_after_animated {
                    if self.before_after_state {
                        self.before_after_anim.start(&ctx);
                    } else {
                        self.before_after_anim.reverse(&ctx);
                    }
                }
            }
        });

        ui.add_space(8.0);

        let available_width = ui.available_width().min(700.0);
        let half_width = (available_width - 20.0) / 2.0;

        ui.horizontal(|ui| {
            // Instant (no animation)
            ui.vertical(|ui| {
                ui.set_min_width(half_width);
                ui.label(egui::RichText::new("Instant (No Animation)").strong());
                ui.add_space(4.0);

                let instant_progress = if self.before_after_state { 1.0 } else { 0.0 };
                self.draw_comparison_widget(ui, instant_progress, half_width);
            });

            ui.add_space(20.0);

            // Animated
            ui.vertical(|ui| {
                ui.set_min_width(half_width);
                ui.label(egui::RichText::new("Animated Transition").strong());
                ui.add_space(4.0);

                let anim_progress = if self.before_after_animated {
                    progress
                } else if self.before_after_state {
                    1.0
                } else {
                    0.0
                };
                self.draw_comparison_widget(ui, anim_progress, half_width);
            });
        });
    }

    fn draw_comparison_widget(&self, ui: &mut egui::Ui, progress: f32, width: f32) {
        let height = 120.0;
        let (response, painter) = ui.allocate_painter(
            egui::vec2(width, height),
            egui::Sense::hover(),
        );
        let rect = response.rect;

        // Background
        let bg_off = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(35, 35, 48)
        } else {
            egui::Color32::from_rgb(235, 235, 248)
        };
        let bg_on = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(30, 50, 70)
        } else {
            egui::Color32::from_rgb(220, 235, 252)
        };
        let bg = lerp_color(bg_off, bg_on, progress);
        painter.rect_filled(rect, 8.0, bg);

        // Sliding card
        let card_width = 60.0;
        let card_height = 40.0;
        let from_x = rect.left() + 15.0;
        let to_x = rect.right() - card_width - 15.0;
        let card_x = egui::lerp(from_x..=to_x, progress);
        let card_y = rect.center().y - card_height / 2.0;
        let card_rect = egui::Rect::from_min_size(
            egui::pos2(card_x, card_y),
            egui::vec2(card_width, card_height),
        );

        let card_color = if ui.visuals().dark_mode {
            lerp_color(
                egui::Color32::from_rgb(80, 80, 110),
                egui::Color32::from_rgb(100, 149, 237),
                progress,
            )
        } else {
            lerp_color(
                egui::Color32::from_rgb(180, 180, 200),
                egui::Color32::from_rgb(50, 100, 200),
                progress,
            )
        };
        painter.rect_filled(card_rect, 6.0, card_color);

        // Label on card
        let label = if progress > 0.5 { "ON" } else { "OFF" };
        painter.text(
            card_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(13.0),
            egui::Color32::WHITE,
        );

        // Progress bar at bottom
        let bar_height = 4.0;
        let bar_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 8.0, rect.bottom() - bar_height - 8.0),
            egui::vec2((rect.width() - 16.0) * progress, bar_height),
        );
        let bar_bg = egui::Rect::from_min_size(
            egui::pos2(rect.left() + 8.0, rect.bottom() - bar_height - 8.0),
            egui::vec2(rect.width() - 16.0, bar_height),
        );
        let track_col = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(50, 50, 65)
        } else {
            egui::Color32::from_rgb(210, 210, 220)
        };
        painter.rect_filled(bar_bg, 2.0, track_col);
        if progress > 0.01 {
            let accent = if ui.visuals().dark_mode {
                egui::Color32::from_rgb(100, 149, 237)
            } else {
                egui::Color32::from_rgb(50, 100, 200)
            };
            painter.rect_filled(bar_rect, 2.0, accent);
        }
    }

    /// Raw linear time (0..1) since easing_anim uses Linear easing.
    fn get_raw_time_progress(&self, ctx: &egui::Context) -> f32 {
        self.easing_anim.progress(ctx)
    }
}
