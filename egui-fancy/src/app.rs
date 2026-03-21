use crate::animation::{Animation, Easing};
use crate::sections::buttons::ButtonsSection;
use crate::sections::dashboard::DashboardSection;
use crate::sections::data_viz::DataVizSection;
use crate::sections::panels::PanelsSection;
use crate::sections::sliders::SlidersSection;
use crate::sections::transitions::TransitionsSection;
use crate::sections::Section;
use crate::theme;

const SIDEBAR_EXPANDED_WIDTH: f32 = 220.0;
const SIDEBAR_COLLAPSED_WIDTH: f32 = 52.0;

pub struct FancyShowcaseApp {
    pub active_section: Section,
    pub is_dark_mode: bool,
    pub sidebar_expanded: bool,
    pub sidebar_animation: Animation,
    pub buttons: ButtonsSection,
    pub sliders: SlidersSection,
    pub data_viz: DataVizSection,
    pub dashboard: DashboardSection,
    pub panels: PanelsSection,
    pub transitions: TransitionsSection,
}

impl Default for FancyShowcaseApp {
    fn default() -> Self {
        Self {
            active_section: Section::Buttons,
            is_dark_mode: true,
            sidebar_expanded: true,
            sidebar_animation: Animation::new(0.3, Easing::EaseOutCubic),
            buttons: ButtonsSection::default(),
            sliders: SlidersSection::default(),
            data_viz: DataVizSection::default(),
            dashboard: DashboardSection::default(),
            panels: PanelsSection::default(),
            transitions: TransitionsSection::default(),
        }
    }
}

impl FancyShowcaseApp {
    fn sidebar_width(&self, ctx: &egui::Context) -> f32 {
        let progress = self.sidebar_animation.progress(ctx);
        egui::lerp(SIDEBAR_COLLAPSED_WIDTH..=SIDEBAR_EXPANDED_WIDTH, progress)
    }
}

impl eframe::App for FancyShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        theme::apply_theme(ctx, self.is_dark_mode);

        let sidebar_width = self.sidebar_width(ctx);

        // Backdrop when sidebar is expanded
        if self.sidebar_expanded || self.sidebar_animation.is_active(ctx) {
            let progress = self.sidebar_animation.progress(ctx);
            if progress > 0.01 {
                let screen = ctx.content_rect();
                let backdrop_alpha = (progress * 80.0) as u8;
                egui::Area::new(egui::Id::new("sidebar_backdrop"))
                    .order(egui::Order::Middle)
                    .fixed_pos(egui::pos2(sidebar_width, 0.0))
                    .show(ctx, |ui: &mut egui::Ui| {
                        let rect = egui::Rect::from_min_size(
                            egui::pos2(0.0, 0.0),
                            egui::vec2(screen.width() - sidebar_width, screen.height()),
                        );
                        let response = ui.allocate_rect(rect, egui::Sense::click());
                        ui.painter().rect_filled(
                            rect,
                            0.0,
                            egui::Color32::from_black_alpha(backdrop_alpha),
                        );
                        // Click backdrop to collapse
                        if response.clicked() {
                            self.sidebar_expanded = false;
                            self.sidebar_animation.reverse(ctx);
                        }
                    });
            }
        }

        // Sidebar
        egui::Area::new(egui::Id::new("sidebar"))
            .order(egui::Order::Foreground)
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui: &mut egui::Ui| {
                let screen = ctx.content_rect();
                let sidebar_rect = egui::Rect::from_min_size(
                    egui::pos2(0.0, 0.0),
                    egui::vec2(sidebar_width, screen.height()),
                );

                // Background
                let bg_color = if self.is_dark_mode {
                    egui::Color32::from_rgb(30, 30, 38)
                } else {
                    egui::Color32::from_rgb(245, 245, 250)
                };
                ui.painter().rect_filled(sidebar_rect, 0.0, bg_color);

                // Shadow on the right edge
                let shadow_rect = egui::Rect::from_min_size(
                    egui::pos2(sidebar_width, 0.0),
                    egui::vec2(6.0, screen.height()),
                );
                ui.painter().rect_filled(
                    shadow_rect,
                    0.0,
                    egui::Color32::from_black_alpha(20),
                );

                ui.allocate_rect(sidebar_rect, egui::Sense::hover());

                let mut child_ui = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(sidebar_rect.shrink(8.0)),
                );

                child_ui.vertical(|ui: &mut egui::Ui| {
                    ui.spacing_mut().item_spacing.y = 4.0;

                    // Toggle button
                    let toggle_text = if self.sidebar_expanded { "\u{2190}" } else { "\u{2192}" };
                    if ui.add(egui::Button::new(toggle_text).frame(false)).clicked() {
                        self.sidebar_expanded = !self.sidebar_expanded;
                        if self.sidebar_expanded {
                            self.sidebar_animation.start(ctx);
                        } else {
                            self.sidebar_animation.reverse(ctx);
                        }
                    }

                    // Title (only when expanded enough)
                    if sidebar_width > 120.0 {
                        ui.add_space(4.0);
                        ui.heading("egui Showcase");
                        ui.separator();
                    } else {
                        ui.add_space(8.0);
                    }

                    ui.add_space(4.0);

                    // Section buttons
                    for &section in Section::ALL {
                        let is_active = self.active_section == section;
                        let text = if sidebar_width > 120.0 {
                            format!("{} {}", section.icon(), section.label())
                        } else {
                            section.icon().to_string()
                        };

                        let button = egui::Button::new(
                            egui::RichText::new(&text).size(14.0),
                        )
                        .fill(if is_active {
                            if self.is_dark_mode {
                                egui::Color32::from_rgb(55, 55, 75)
                            } else {
                                egui::Color32::from_rgb(220, 220, 235)
                            }
                        } else {
                            egui::Color32::TRANSPARENT
                        })
                        .min_size(egui::vec2(sidebar_width - 16.0, 32.0));

                        if ui.add(button).clicked() {
                            self.active_section = section;
                        }
                    }

                    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui: &mut egui::Ui| {
                        // Theme toggle at bottom
                        let theme_text = if sidebar_width > 120.0 {
                            if self.is_dark_mode { "\u{1f319} Dark" } else { "\u{2600} Light" }
                        } else if self.is_dark_mode {
                            "\u{1f319}"
                        } else {
                            "\u{2600}"
                        };
                        if ui.add(egui::Button::new(theme_text).frame(false)).clicked() {
                            self.is_dark_mode = !self.is_dark_mode;
                        }
                    });
                });
            });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            // Offset content to avoid sidebar overlap when collapsed
            let margin = SIDEBAR_COLLAPSED_WIDTH + 8.0;
            ui.add_space(8.0);
            ui.add_space(0.0);
            let available = ui.available_rect_before_wrap();
            let content_rect = egui::Rect::from_min_max(
                egui::pos2(available.left() + margin, available.top()),
                egui::pos2(available.right() - 16.0, available.bottom() - 16.0),
            );
            let mut content_ui = ui.new_child(
                egui::UiBuilder::new().max_rect(content_rect),
            );
            egui::Frame::NONE
                .show(&mut content_ui, |ui: &mut egui::Ui| {
                    match self.active_section {
                        Section::Buttons => self.buttons.show(ui),
                        Section::Sliders => self.sliders.show(ui),
                        Section::DataViz => self.data_viz.show(ui),
                        Section::Dashboard => self.dashboard.show(ui),
                        Section::Panels => self.panels.show(ui),
                        Section::Transitions => self.transitions.show(ui),
                    }
                });
        });
    }
}
