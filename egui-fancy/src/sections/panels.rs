use crate::animation::{Animation, Easing};

/// Section 5: Panels & Navigation
///
/// Demonstrates egui's panel system and navigation patterns:
/// - Idiomatic: TopBottomPanel, SidePanel, CentralPanel, ScrollArea, CollapsingHeader
/// - Navigation: tab bar with animated underline, breadcrumb trail, collapsible file tree
/// - Exotic: split pane with draggable divider

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PanelTab {
    Idiomatic,
    Navigation,
    SplitPane,
}

impl PanelTab {
    const ALL: &[PanelTab] = &[PanelTab::Idiomatic, PanelTab::Navigation, PanelTab::SplitPane];

    fn label(self) -> &'static str {
        match self {
            PanelTab::Idiomatic => "Idiomatic Panels",
            PanelTab::Navigation => "Navigation Patterns",
            PanelTab::SplitPane => "Custom Split Pane",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NavTab {
    Overview,
    Details,
    Settings,
    Help,
}

impl NavTab {
    const ALL: &[NavTab] = &[NavTab::Overview, NavTab::Details, NavTab::Settings, NavTab::Help];

    fn label(self) -> &'static str {
        match self {
            NavTab::Overview => "Overview",
            NavTab::Details => "Details",
            NavTab::Settings => "Settings",
            NavTab::Help => "Help",
        }
    }
}

pub struct PanelsSection {
    active_tab: PanelTab,
    tab_underline_anim: Animation,
    prev_tab_index: usize,

    // Idiomatic panels state
    left_panel_open: bool,
    right_panel_open: bool,
    accordion_states: [bool; 4],
    scroll_content_lines: usize,

    // Navigation state
    nav_tab: NavTab,
    nav_tab_anim: Animation,
    prev_nav_tab_index: usize,
    breadcrumb_path: Vec<&'static str>,
    tree_expanded: [bool; 3],

    // Split pane state
    split_fraction: f32,
    split_dragging: bool,
}

impl Default for PanelsSection {
    fn default() -> Self {
        Self {
            active_tab: PanelTab::Idiomatic,
            tab_underline_anim: Animation::new(0.3, Easing::EaseOutCubic),
            prev_tab_index: 0,

            left_panel_open: true,
            right_panel_open: true,
            accordion_states: [true, false, false, false],
            scroll_content_lines: 30,

            nav_tab: NavTab::Overview,
            nav_tab_anim: Animation::new(0.3, Easing::EaseOutCubic),
            prev_nav_tab_index: 0,
            breadcrumb_path: vec!["Home", "Section", "Item"],
            tree_expanded: [true, false, false],

            split_fraction: 0.5,
            split_dragging: false,
        }
    }
}

impl std::fmt::Debug for PanelsSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PanelsSection").finish()
    }
}

impl PanelsSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Panels & Navigation");
        ui.add_space(4.0);
        ui.label("Demonstrates egui's panel system, navigation patterns, and a custom split pane.");
        ui.add_space(12.0);

        // Top-level tab bar
        self.show_top_tabs(ui);
        ui.add_space(12.0);

        match self.active_tab {
            PanelTab::Idiomatic => self.show_idiomatic(ui),
            PanelTab::Navigation => self.show_navigation(ui),
            PanelTab::SplitPane => self.show_split_pane(ui),
        }
    }

    fn show_top_tabs(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        let tab_count = PanelTab::ALL.len();
        let current_index = PanelTab::ALL.iter().position(|&t| t == self.active_tab).unwrap_or(0);

        let available_width = ui.available_width().min(600.0);
        let tab_width = available_width / tab_count as f32;

        let (response, painter) = ui.allocate_painter(
            egui::vec2(available_width, 32.0),
            egui::Sense::click(),
        );
        let rect = response.rect;

        // Draw tab labels
        for (i, tab) in PanelTab::ALL.iter().enumerate() {
            let tab_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left() + i as f32 * tab_width, rect.top()),
                egui::vec2(tab_width, 28.0),
            );

            let is_active = *tab == self.active_tab;
            let color = if is_active {
                ui.visuals().strong_text_color()
            } else {
                ui.visuals().text_color()
            };

            painter.text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                tab.label(),
                egui::FontId::proportional(14.0),
                color,
            );

            // Click detection per tab
            if response.clicked()
                && let Some(pos) = response.interact_pointer_pos()
                    && tab_rect.contains(pos) && !is_active {
                        self.prev_tab_index = current_index;
                        self.active_tab = *tab;
                        self.tab_underline_anim.start(&ctx);
                    }
        }

        // Animated underline
        let anim_progress = self.tab_underline_anim.progress(&ctx);
        let underline_x = if self.tab_underline_anim.is_active(&ctx) {
            let from = self.prev_tab_index as f32 * tab_width;
            let to = current_index as f32 * tab_width;
            egui::lerp(from..=to, anim_progress)
        } else {
            current_index as f32 * tab_width
        };

        let underline_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + underline_x + 8.0, rect.bottom() - 3.0),
            egui::vec2(tab_width - 16.0, 3.0),
        );
        let accent = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(100, 149, 237)
        } else {
            egui::Color32::from_rgb(50, 100, 200)
        };
        painter.rect_filled(underline_rect, 1.5, accent);

        // Separator line
        painter.line_segment(
            [
                egui::pos2(rect.left(), rect.bottom() - 0.5),
                egui::pos2(rect.left() + available_width, rect.bottom() - 0.5),
            ],
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
        );
    }

    fn show_idiomatic(&mut self, ui: &mut egui::Ui) {
        ui.label(egui::RichText::new("Idiomatic").color(
            if ui.visuals().dark_mode {
                egui::Color32::from_rgb(100, 149, 237)
            } else {
                egui::Color32::from_rgb(50, 100, 200)
            },
        ));
        ui.add_space(8.0);

        // Simulated panel layout using frames
        let available = ui.available_rect_before_wrap();
        let panel_height = (available.height() - 80.0).max(300.0);

        // Top bar simulation
        egui::Frame::new()
            .fill(if ui.visuals().dark_mode {
                egui::Color32::from_rgb(40, 40, 50)
            } else {
                egui::Color32::from_rgb(230, 230, 240)
            })
            .inner_margin(egui::Margin::symmetric(12, 6))
            .corner_radius(4.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("TopBottomPanel::top()").strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("App Header Bar");
                    });
                });
            });

        ui.add_space(4.0);

        // Main body: left panel | center | right panel
        ui.horizontal(|ui| {
            let body_width = ui.available_width();
            let left_width = if self.left_panel_open { 160.0 } else { 0.0 };
            let right_width = if self.right_panel_open { 180.0 } else { 0.0 };
            let center_width = (body_width - left_width - right_width - 16.0).max(100.0);

            // Left SidePanel
            if self.left_panel_open {
                egui::Frame::new()
                    .fill(if ui.visuals().dark_mode {
                        egui::Color32::from_rgb(35, 35, 45)
                    } else {
                        egui::Color32::from_rgb(238, 238, 245)
                    })
                    .inner_margin(egui::Margin::same(8))
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(left_width - 16.0, panel_height));
                        ui.label(egui::RichText::new("SidePanel::left()").strong().size(11.0));
                        ui.separator();
                        ui.label("Navigation");
                        ui.add_space(4.0);
                        for item in &["Dashboard", "Analytics", "Reports", "Settings"] {
                            let _ = ui.selectable_label(false, *item);
                        }
                    });
            }

            // Center panel with scroll area and collapsing headers
            egui::Frame::new()
                .fill(if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(28, 28, 36)
                } else {
                    egui::Color32::from_rgb(248, 248, 252)
                })
                .inner_margin(egui::Margin::same(8))
                .corner_radius(4.0)
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2(center_width, panel_height));
                    ui.label(egui::RichText::new("CentralPanel").strong());
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.left_panel_open, "Left Panel");
                        ui.checkbox(&mut self.right_panel_open, "Right Panel");
                    });

                    ui.add_space(4.0);
                    ui.label("ScrollArea + CollapsingHeader accordion:");
                    ui.add_space(4.0);

                    egui::ScrollArea::vertical()
                        .max_height(panel_height - 100.0)
                        .show(ui, |ui| {
                            let headers = [
                                "Section A — Introduction",
                                "Section B — Configuration",
                                "Section C — Advanced Settings",
                                "Section D — About",
                            ];
                            for (i, header) in headers.iter().enumerate() {
                                let open = &mut self.accordion_states[i];
                                egui::CollapsingHeader::new(*header)
                                    .open(Some(*open))
                                    .show(ui, |ui| {
                                        for line in 0..self.scroll_content_lines.min(8) {
                                            ui.label(format!(
                                                "Content line {} of section {}",
                                                line + 1,
                                                (b'A' + i as u8) as char
                                            ));
                                        }
                                    });
                                // Toggle tracking
                                *open = ui
                                    .ctx()
                                    .data_mut(|d| {
                                        d.get_temp::<bool>(egui::Id::new(format!("accordion_{i}")))
                                    })
                                    .unwrap_or(*open);
                            }
                        });
                });

            // Right SidePanel
            if self.right_panel_open {
                egui::Frame::new()
                    .fill(if ui.visuals().dark_mode {
                        egui::Color32::from_rgb(35, 35, 45)
                    } else {
                        egui::Color32::from_rgb(238, 238, 245)
                    })
                    .inner_margin(egui::Margin::same(8))
                    .corner_radius(4.0)
                    .show(ui, |ui| {
                        ui.set_min_size(egui::vec2(right_width - 16.0, panel_height));
                        ui.label(egui::RichText::new("SidePanel::right()").strong().size(11.0));
                        ui.separator();
                        ui.label("Inspector / Properties");
                        ui.add_space(4.0);
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut String::from("Widget A"));
                        ui.label("Opacity:");
                        let mut val = 0.85_f32;
                        ui.add(egui::Slider::new(&mut val, 0.0..=1.0));
                        ui.label("Visible:");
                        let mut checked = true;
                        ui.checkbox(&mut checked, "Show");
                    });
            }
        });
    }

    fn show_navigation(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();

        // --- Tab bar with animated underline ---
        ui.label(egui::RichText::new("Tab Bar with Animated Underline").strong());
        ui.add_space(4.0);

        let tab_count = NavTab::ALL.len();
        let current_index = NavTab::ALL.iter().position(|&t| t == self.nav_tab).unwrap_or(0);
        let available_width = ui.available_width().min(500.0);
        let tab_width = available_width / tab_count as f32;

        let (response, painter) = ui.allocate_painter(
            egui::vec2(available_width, 30.0),
            egui::Sense::click(),
        );
        let rect = response.rect;

        for (i, tab) in NavTab::ALL.iter().enumerate() {
            let tab_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left() + i as f32 * tab_width, rect.top()),
                egui::vec2(tab_width, 26.0),
            );
            let is_active = *tab == self.nav_tab;
            let color = if is_active {
                ui.visuals().strong_text_color()
            } else {
                ui.visuals().text_color()
            };
            painter.text(
                tab_rect.center(),
                egui::Align2::CENTER_CENTER,
                tab.label(),
                egui::FontId::proportional(13.0),
                color,
            );
            if response.clicked()
                && let Some(pos) = response.interact_pointer_pos()
                    && tab_rect.contains(pos) && !is_active {
                        self.prev_nav_tab_index = current_index;
                        self.nav_tab = *tab;
                        self.nav_tab_anim.start(&ctx);
                    }
        }

        let anim_p = self.nav_tab_anim.progress(&ctx);
        let underline_x = if self.nav_tab_anim.is_active(&ctx) {
            let from = self.prev_nav_tab_index as f32 * tab_width;
            let to = current_index as f32 * tab_width;
            egui::lerp(from..=to, anim_p)
        } else {
            current_index as f32 * tab_width
        };

        let accent = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(100, 149, 237)
        } else {
            egui::Color32::from_rgb(50, 100, 200)
        };
        painter.rect_filled(
            egui::Rect::from_min_size(
                egui::pos2(rect.left() + underline_x + 4.0, rect.bottom() - 2.5),
                egui::vec2(tab_width - 8.0, 2.5),
            ),
            1.0,
            accent,
        );
        painter.line_segment(
            [
                egui::pos2(rect.left(), rect.bottom() - 0.5),
                egui::pos2(rect.left() + available_width, rect.bottom() - 0.5),
            ],
            egui::Stroke::new(1.0, ui.visuals().widgets.noninteractive.bg_stroke.color),
        );

        // Tab content
        ui.add_space(8.0);
        egui::Frame::new()
            .fill(if ui.visuals().dark_mode {
                egui::Color32::from_rgb(30, 30, 40)
            } else {
                egui::Color32::from_rgb(245, 245, 252)
            })
            .inner_margin(egui::Margin::same(12))
            .corner_radius(4.0)
            .show(ui, |ui| {
                match self.nav_tab {
                    NavTab::Overview => ui.label("This is the Overview tab content. It shows a summary of all available information."),
                    NavTab::Details => ui.label("The Details tab provides in-depth information about the selected item, including metadata and history."),
                    NavTab::Settings => ui.label("Settings tab: configure preferences, layout options, and notification rules."),
                    NavTab::Help => ui.label("Help tab: documentation links, keyboard shortcuts, and support contact information."),
                };
            });

        ui.add_space(16.0);

        // --- Breadcrumbs ---
        ui.label(egui::RichText::new("Breadcrumb Trail").strong());
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let path_len = self.breadcrumb_path.len();
            for (i, segment) in self.breadcrumb_path.clone().iter().enumerate() {
                if i > 0 {
                    ui.label(egui::RichText::new(" > ").weak());
                }
                if i < path_len - 1 {
                    if ui.link(*segment).clicked() {
                        self.breadcrumb_path.truncate(i + 1);
                    }
                } else {
                    ui.label(egui::RichText::new(*segment).strong());
                }
            }
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            let items: &[&str] = match self.breadcrumb_path.last() {
                Some(&"Home") => &["Section", "Gallery", "Docs"],
                Some(&"Section") => &["Item", "SubSection", "Details"],
                _ => &["Child A", "Child B"],
            };
            for &item in items {
                if ui.small_button(format!("Go to {item}")).clicked() {
                    self.breadcrumb_path.push(item);
                }
            }
            if self.breadcrumb_path.len() > 1
                && ui.small_button("Back").clicked() {
                    self.breadcrumb_path.pop();
                }
        });

        ui.add_space(16.0);

        // --- Collapsible file tree ---
        ui.label(egui::RichText::new("Collapsible File Tree").strong());
        ui.add_space(4.0);
        egui::Frame::new()
            .fill(if ui.visuals().dark_mode {
                egui::Color32::from_rgb(30, 30, 40)
            } else {
                egui::Color32::from_rgb(245, 245, 252)
            })
            .inner_margin(egui::Margin::same(8))
            .corner_radius(4.0)
            .show(ui, |ui| {
                egui::CollapsingHeader::new("\u{1f4c1} src/")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::CollapsingHeader::new("\u{1f4c1} sections/")
                            .default_open(self.tree_expanded[0])
                            .show(ui, |ui| {
                                ui.label("  \u{1f4c4} buttons.rs");
                                ui.label("  \u{1f4c4} sliders.rs");
                                ui.label("  \u{1f4c4} data_viz.rs");
                                ui.label("  \u{1f4c4} dashboard.rs");
                                ui.label("  \u{1f4c4} panels.rs");
                                ui.label("  \u{1f4c4} transitions.rs");
                                ui.label("  \u{1f4c4} mod.rs");
                            });
                        ui.label("  \u{1f4c4} main.rs");
                        ui.label("  \u{1f4c4} app.rs");
                        ui.label("  \u{1f4c4} animation.rs");
                        ui.label("  \u{1f4c4} theme.rs");
                    });
                egui::CollapsingHeader::new("\u{1f4c1} assets/")
                    .default_open(false)
                    .show(ui, |ui| {
                        ui.label("  \u{1f4c4} icon.png");
                        ui.label("  \u{1f4c4} style.css");
                    });
                ui.label("\u{1f4c4} Cargo.toml");
                ui.label("\u{1f4c4} README.md");
            });
    }

    fn show_split_pane(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Custom").color(
                if ui.visuals().dark_mode {
                    egui::Color32::from_rgb(237, 149, 100)
                } else {
                    egui::Color32::from_rgb(200, 100, 50)
                },
            ));
            ui.label(" — Draggable split pane built with allocate_rect() + pointer tracking");
        });
        ui.add_space(8.0);

        let available = ui.available_rect_before_wrap();
        let pane_height = (available.height() - 20.0).max(250.0);
        let total_width = available.width().min(900.0);
        let divider_width = 6.0;

        let left_width = (total_width - divider_width) * self.split_fraction;
        let right_width = total_width - divider_width - left_width;

        let start_x = available.left();
        let start_y = available.top();

        // Left pane
        let left_rect = egui::Rect::from_min_size(
            egui::pos2(start_x, start_y),
            egui::vec2(left_width, pane_height),
        );
        // Divider
        let divider_rect = egui::Rect::from_min_size(
            egui::pos2(start_x + left_width, start_y),
            egui::vec2(divider_width, pane_height),
        );
        // Right pane
        let right_rect = egui::Rect::from_min_size(
            egui::pos2(start_x + left_width + divider_width, start_y),
            egui::vec2(right_width, pane_height),
        );

        // Allocate the full area
        let full_rect = egui::Rect::from_min_size(
            egui::pos2(start_x, start_y),
            egui::vec2(total_width, pane_height),
        );
        ui.allocate_rect(full_rect, egui::Sense::hover());

        let painter = ui.painter();

        // Paint left pane background
        let pane_bg = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(30, 30, 40)
        } else {
            egui::Color32::from_rgb(245, 245, 252)
        };
        painter.rect_filled(left_rect, 4.0, pane_bg);
        painter.rect_filled(right_rect, 4.0, pane_bg);

        // Divider interaction
        let divider_response = ui.interact(divider_rect, egui::Id::new("split_divider"), egui::Sense::drag());

        if divider_response.dragged() {
            self.split_dragging = true;
            let delta = divider_response.drag_delta().x;
            let new_frac = self.split_fraction + delta / (total_width - divider_width);
            self.split_fraction = new_frac.clamp(0.15, 0.85);
        } else {
            self.split_dragging = false;
        }

        // Divider paint
        let divider_color = if self.split_dragging || divider_response.hovered() {
            if ui.visuals().dark_mode {
                egui::Color32::from_rgb(100, 149, 237)
            } else {
                egui::Color32::from_rgb(50, 100, 200)
            }
        } else if ui.visuals().dark_mode {
            egui::Color32::from_rgb(60, 60, 75)
        } else {
            egui::Color32::from_rgb(200, 200, 210)
        };
        painter.rect_filled(divider_rect, 2.0, divider_color);

        // Grab handle dots on divider
        let dot_color = if ui.visuals().dark_mode {
            egui::Color32::from_rgb(150, 150, 170)
        } else {
            egui::Color32::from_rgb(120, 120, 140)
        };
        let cx = divider_rect.center().x;
        let cy = divider_rect.center().y;
        for offset in [-8.0, 0.0, 8.0] {
            painter.circle_filled(egui::pos2(cx, cy + offset), 1.5, dot_color);
        }

        // Change cursor on hover
        if divider_response.hovered() || self.split_dragging {
            ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
        }

        // Left pane content
        let mut left_ui = ui.new_child(
            egui::UiBuilder::new().max_rect(left_rect.shrink(8.0)),
        );
        left_ui.label(egui::RichText::new("Left Pane").strong());
        left_ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("split_left_scroll")
            .show(&mut left_ui, |ui| {
                for i in 0..20 {
                    ui.label(format!("Left content line {}", i + 1));
                }
            });

        // Right pane content
        let mut right_ui = ui.new_child(
            egui::UiBuilder::new().max_rect(right_rect.shrink(8.0)),
        );
        right_ui.label(egui::RichText::new("Right Pane").strong());
        right_ui.separator();
        egui::ScrollArea::vertical()
            .id_salt("split_right_scroll")
            .show(&mut right_ui, |ui| {
                for i in 0..20 {
                    ui.label(format!("Right content line {}", i + 1));
                }
            });
    }
}
