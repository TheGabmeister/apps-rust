use std::f32::consts::PI;

const GRID_COLS: usize = 3;
const CELL_H: f32 = 180.0;
const GAP: f32 = 10.0;
const TITLE_H: f32 = 28.0;
const RESIZE_ZONE: f32 = 6.0;
const CORNER_ZONE: f32 = 16.0;
const ROUNDING: f32 = 8.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TileId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TileContent {
    Sparkline,
    Gauge,
    StatCard,
    MiniControls,
    TextLog,
}

impl TileContent {
    fn title(self) -> &'static str {
        match self {
            Self::Sparkline => "Sparklines",
            Self::Gauge => "Performance",
            Self::StatCard => "Active Users",
            Self::MiniControls => "Controls",
            Self::TextLog => "System Log",
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: TileId,
    col: usize,
    row: usize,
    col_span: usize,
    row_span: usize,
    content: TileContent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResizeEdge {
    Right,
    Bottom,
    BottomRight,
}

pub struct DashboardSection {
    tiles: Vec<Tile>,
    anim_rects: Vec<egui::Rect>,

    // Drag-to-reorder state
    drag_idx: Option<usize>,
    drag_offset: egui::Vec2,
    drop_target: Option<(usize, usize)>,

    // Resize state
    resize_idx: Option<usize>,
    resize_edge: ResizeEdge,
    resize_orig_span: (usize, usize),
    resize_start: egui::Pos2,

    // Tile content state
    gauge_value: f32,
    slider_val: f32,
    toggle_a: bool,
    toggle_b: bool,
}

impl Default for DashboardSection {
    fn default() -> Self {
        Self {
            tiles: vec![
                Tile { id: TileId(0), col: 0, row: 0, col_span: 2, row_span: 1, content: TileContent::Sparkline },
                Tile { id: TileId(1), col: 2, row: 0, col_span: 1, row_span: 1, content: TileContent::Gauge },
                Tile { id: TileId(2), col: 0, row: 1, col_span: 1, row_span: 1, content: TileContent::StatCard },
                Tile { id: TileId(3), col: 1, row: 1, col_span: 1, row_span: 1, content: TileContent::MiniControls },
                Tile { id: TileId(4), col: 2, row: 1, col_span: 1, row_span: 1, content: TileContent::TextLog },
            ],
            anim_rects: vec![],
            drag_idx: None,
            drag_offset: egui::Vec2::ZERO,
            drop_target: None,
            resize_idx: None,
            resize_edge: ResizeEdge::BottomRight,
            resize_orig_span: (1, 1),
            resize_start: egui::Pos2::ZERO,
            gauge_value: 0.72,
            slider_val: 42.0,
            toggle_a: true,
            toggle_b: false,
        }
    }
}

impl DashboardSection {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.heading("Dashboard Grid");
        ui.add_space(4.0);
        ui.horizontal(|ui: &mut egui::Ui| {
            ui.label(
                egui::RichText::new("Drag title bars to reorder \u{2022} Drag edges/corners to resize")
                    .weak(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui: &mut egui::Ui| {
                if ui.small_button("Reset Layout").clicked() {
                    *self = DashboardSection::default();
                }
            });
        });
        ui.add_space(8.0);

        let available = ui.available_rect_before_wrap();
        let cell_w =
            (available.width() - GAP * (GRID_COLS as f32 + 1.0)) / GRID_COLS as f32;
        let max_row = self
            .tiles
            .iter()
            .map(|t| t.row + t.row_span)
            .max()
            .unwrap_or(2);
        let grid_h = max_row as f32 * (CELL_H + GAP) + GAP;
        let (grid_rect, _) =
            ui.allocate_exact_size(egui::vec2(available.width(), grid_h), egui::Sense::hover());
        let origin = egui::pos2(grid_rect.left() + GAP, grid_rect.top());

        // Target pixel rects
        let target_rects: Vec<egui::Rect> = (0..self.tiles.len())
            .map(|i| tile_pixel_rect(&self.tiles[i], origin, cell_w))
            .collect();

        // Init / lerp animated rects
        if self.anim_rects.len() != self.tiles.len() {
            self.anim_rects = target_rects.clone();
        }
        let dt = ui.input(|i| i.stable_dt);
        let speed = 12.0;
        let mut needs_repaint = false;
        for (anim, target) in self.anim_rects.iter_mut().zip(target_rects.iter()) {
            let d = (target.min.x - anim.min.x).abs()
                + (target.min.y - anim.min.y).abs()
                + (target.max.x - anim.max.x).abs()
                + (target.max.y - anim.max.y).abs();
            if d > 0.5 {
                let f = (speed * dt).min(1.0);
                let min_x = anim.min.x + (target.min.x - anim.min.x) * f;
                let min_y = anim.min.y + (target.min.y - anim.min.y) * f;
                let max_x = anim.max.x + (target.max.x - anim.max.x) * f;
                let max_y = anim.max.y + (target.max.y - anim.max.y) * f;
                *anim = egui::Rect::from_min_max(egui::pos2(min_x, min_y), egui::pos2(max_x, max_y));
                needs_repaint = true;
            } else {
                *anim = *target;
            }
        }
        if needs_repaint {
            ui.ctx().request_repaint();
        }

        // Render order: dragged tile last (on top)
        let mut render_order: Vec<usize> = (0..self.tiles.len()).collect();
        if let Some(di) = self.drag_idx
            && let Some(pos) = render_order.iter().position(|&x| x == di)
        {
            render_order.remove(pos);
            render_order.push(di);
        }

        let pointer = ui.input(|i| i.pointer.interact_pos());
        let is_dark = ui.visuals().dark_mode;

        for &i in &render_order {
            let is_dragged = self.drag_idx == Some(i);

            let rect = if is_dragged {
                if let Some(pos) = pointer {
                    egui::Rect::from_min_size(pos - self.drag_offset, self.anim_rects[i].size())
                } else {
                    self.anim_rects[i]
                }
            } else {
                self.anim_rects[i]
            };

            // --- Paint tile ---
            paint_tile(ui, &self.tiles[i], rect, is_dragged, is_dark);

            // --- Render content (only for non-dragged tiles) ---
            if !is_dragged {
                let content_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.left() + 8.0, rect.top() + TITLE_H + 4.0),
                    egui::pos2(rect.right() - 8.0, rect.bottom() - 8.0),
                );
                if content_rect.width() > 10.0 && content_rect.height() > 10.0 {
                    self.render_content(ui, i, content_rect);
                }
            }

            // --- Title bar interaction (drag-to-reorder) ---
            let title_rect = egui::Rect::from_min_size(
                rect.left_top(),
                egui::vec2(rect.width(), TITLE_H),
            );
            let title_id = egui::Id::new(("dash_title", self.tiles[i].id.0));
            let title_resp =
                ui.interact(title_rect, title_id, egui::Sense::click_and_drag());

            if title_resp.drag_started() && self.drag_idx.is_none() && self.resize_idx.is_none() {
                self.drag_idx = Some(i);
                if let Some(pos) = title_resp.interact_pointer_pos() {
                    self.drag_offset = pos - rect.left_top();
                }
            }

            // --- Corner resize interaction ---
            if !is_dragged {
                let corner_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.right() - CORNER_ZONE, rect.bottom() - CORNER_ZONE),
                    rect.right_bottom(),
                );
                let corner_id = egui::Id::new(("dash_corner", self.tiles[i].id.0));
                let corner_resp =
                    ui.interact(corner_rect, corner_id, egui::Sense::click_and_drag());
                if corner_resp.drag_started()
                    && self.drag_idx.is_none()
                    && self.resize_idx.is_none()
                {
                    self.resize_idx = Some(i);
                    self.resize_edge = ResizeEdge::BottomRight;
                    self.resize_orig_span =
                        (self.tiles[i].col_span, self.tiles[i].row_span);
                    self.resize_start =
                        corner_resp.interact_pointer_pos().unwrap_or(rect.right_bottom());
                }

                // Right edge
                let right_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.right() - RESIZE_ZONE, rect.top() + TITLE_H),
                    egui::pos2(rect.right(), rect.bottom() - CORNER_ZONE),
                );
                let right_id = egui::Id::new(("dash_re", self.tiles[i].id.0));
                let right_resp =
                    ui.interact(right_rect, right_id, egui::Sense::click_and_drag());
                if right_resp.drag_started()
                    && self.drag_idx.is_none()
                    && self.resize_idx.is_none()
                {
                    self.resize_idx = Some(i);
                    self.resize_edge = ResizeEdge::Right;
                    self.resize_orig_span =
                        (self.tiles[i].col_span, self.tiles[i].row_span);
                    self.resize_start =
                        right_resp.interact_pointer_pos().unwrap_or(rect.right_bottom());
                }

                // Bottom edge
                let bottom_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.left(), rect.bottom() - RESIZE_ZONE),
                    egui::pos2(rect.right() - CORNER_ZONE, rect.bottom()),
                );
                let bottom_id = egui::Id::new(("dash_be", self.tiles[i].id.0));
                let bottom_resp =
                    ui.interact(bottom_rect, bottom_id, egui::Sense::click_and_drag());
                if bottom_resp.drag_started()
                    && self.drag_idx.is_none()
                    && self.resize_idx.is_none()
                {
                    self.resize_idx = Some(i);
                    self.resize_edge = ResizeEdge::Bottom;
                    self.resize_orig_span =
                        (self.tiles[i].col_span, self.tiles[i].row_span);
                    self.resize_start =
                        bottom_resp.interact_pointer_pos().unwrap_or(rect.right_bottom());
                }
            }
        }

        // --- Update drag ---
        if let Some(di) = self.drag_idx {
            ui.ctx().request_repaint();
            if let Some(pos) = pointer {
                let col = ((pos.x - origin.x) / (cell_w + GAP))
                    .floor()
                    .max(0.0) as usize;
                let row = ((pos.y - origin.y) / (CELL_H + GAP))
                    .floor()
                    .max(0.0) as usize;
                let max_col = GRID_COLS.saturating_sub(self.tiles[di].col_span);
                self.drop_target = Some((col.min(max_col), row));
            }
            if !ui.input(|i| i.pointer.primary_down()) {
                if let Some((col, row)) = self.drop_target {
                    // Set anim rect to current display position for smooth snap
                    if let Some(pos) = pointer {
                        self.anim_rects[di] = egui::Rect::from_min_size(
                            pos - self.drag_offset,
                            self.anim_rects[di].size(),
                        );
                    }
                    self.tiles[di].col = col;
                    self.tiles[di].row = row;
                    self.resolve_overlaps(di);
                }
                self.drag_idx = None;
                self.drop_target = None;
            }
        }

        // --- Update resize ---
        if let Some(ri) = self.resize_idx {
            ui.ctx().request_repaint();
            if let Some(pos) = pointer {
                let dx = pos.x - self.resize_start.x;
                let dy = pos.y - self.resize_start.y;
                let (orig_w, orig_h) = self.resize_orig_span;

                if matches!(self.resize_edge, ResizeEdge::Right | ResizeEdge::BottomRight) {
                    let new_w = (orig_w as f32 + dx / (cell_w + GAP))
                        .round()
                        .max(1.0) as usize;
                    let max_w = GRID_COLS - self.tiles[ri].col;
                    self.tiles[ri].col_span = new_w.min(max_w);
                }
                if matches!(self.resize_edge, ResizeEdge::Bottom | ResizeEdge::BottomRight) {
                    let new_h = (orig_h as f32 + dy / (CELL_H + GAP))
                        .round()
                        .max(1.0) as usize;
                    self.tiles[ri].row_span = new_h.min(4);
                }
                // Snap animated rect for the tile being resized
                self.anim_rects[ri] = tile_pixel_rect(&self.tiles[ri], origin, cell_w);
            }
            if !ui.input(|i| i.pointer.primary_down()) {
                self.resolve_overlaps(ri);
                self.resize_idx = None;
            }
        }

        // --- Drop placeholder ---
        if let (Some(di), Some((col, row))) = (self.drag_idx, self.drop_target) {
            let t = &self.tiles[di];
            let x = origin.x + col as f32 * (cell_w + GAP);
            let y = origin.y + row as f32 * (CELL_H + GAP);
            let w = t.col_span as f32 * cell_w + (t.col_span as f32 - 1.0) * GAP;
            let h = t.row_span as f32 * CELL_H + (t.row_span as f32 - 1.0) * GAP;
            let ph = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, h));
            ui.painter().rect_filled(
                ph,
                ROUNDING,
                egui::Color32::from_rgba_premultiplied(100, 149, 237, 25),
            );
            ui.painter().rect_stroke(
                ph,
                ROUNDING,
                egui::Stroke::new(
                    2.0,
                    egui::Color32::from_rgba_premultiplied(100, 149, 237, 160),
                ),
                egui::StrokeKind::Outside,
            );
        }
    }

    // --- Content rendering ---

    fn render_content(&mut self, ui: &mut egui::Ui, idx: usize, rect: egui::Rect) {
        match self.tiles[idx].content {
            TileContent::Sparkline => render_sparkline(ui, rect),
            TileContent::Gauge => render_gauge(ui, rect, self.gauge_value),
            TileContent::StatCard => render_stat_card(ui, rect),
            TileContent::MiniControls => {
                self.render_mini_controls(ui, idx, rect);
            }
            TileContent::TextLog => render_text_log(ui, idx, rect),
        }
    }

    fn render_mini_controls(&mut self, ui: &mut egui::Ui, _idx: usize, rect: egui::Rect) {
        let mut child =
            ui.new_child(egui::UiBuilder::new().max_rect(rect));
        child.vertical(|ui: &mut egui::Ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            ui.add(
                egui::Slider::new(&mut self.slider_val, 0.0..=100.0)
                    .text("Value"),
            );
            ui.checkbox(&mut self.toggle_a, "Option A");
            ui.checkbox(&mut self.toggle_b, "Option B");
            ui.horizontal(|ui: &mut egui::Ui| {
                ui.label("Gauge:");
                ui.add(
                    egui::Slider::new(&mut self.gauge_value, 0.0..=1.0),
                );
            });
        });
    }

    // --- Overlap resolution ---

    fn resolve_overlaps(&mut self, priority: usize) {
        // Push any tile overlapping `priority` downward
        for i in 0..self.tiles.len() {
            if i == priority {
                continue;
            }
            if tiles_overlap(&self.tiles[priority], &self.tiles[i]) {
                self.tiles[i].row =
                    self.tiles[priority].row + self.tiles[priority].row_span;
            }
        }
        // Iteratively resolve cascading overlaps
        for _ in 0..20 {
            let mut changed = false;
            for i in 0..self.tiles.len() {
                for j in (i + 1)..self.tiles.len() {
                    if tiles_overlap(&self.tiles[i], &self.tiles[j]) {
                        // Push the lower one further down
                        let upper = if self.tiles[i].row <= self.tiles[j].row {
                            i
                        } else {
                            j
                        };
                        let lower = if upper == i { j } else { i };
                        self.tiles[lower].row =
                            self.tiles[upper].row + self.tiles[upper].row_span;
                        changed = true;
                    }
                }
            }
            if !changed {
                break;
            }
        }
    }
}

// --- Free functions ---

fn tile_pixel_rect(t: &Tile, origin: egui::Pos2, cell_w: f32) -> egui::Rect {
    let x = origin.x + t.col as f32 * (cell_w + GAP);
    let y = origin.y + t.row as f32 * (CELL_H + GAP);
    let w = t.col_span as f32 * cell_w + (t.col_span as f32 - 1.0) * GAP;
    let h = t.row_span as f32 * CELL_H + (t.row_span as f32 - 1.0) * GAP;
    egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(w, h))
}

fn tiles_overlap(a: &Tile, b: &Tile) -> bool {
    a.col < b.col + b.col_span
        && a.col + a.col_span > b.col
        && a.row < b.row + b.row_span
        && a.row + a.row_span > b.row
}

fn paint_tile(
    ui: &egui::Ui,
    tile: &Tile,
    rect: egui::Rect,
    is_dragged: bool,
    is_dark: bool,
) {
    let painter = ui.painter();

    // Drop shadow
    let shadow_off = if is_dragged { 6.0 } else { 2.0 };
    let shadow_alpha = if is_dragged { 50 } else { 20 };
    let shadow_rect = rect.translate(egui::vec2(shadow_off, shadow_off));
    painter.rect_filled(
        shadow_rect,
        ROUNDING,
        egui::Color32::from_black_alpha(shadow_alpha),
    );

    // Background
    let bg = if is_dark {
        egui::Color32::from_rgb(38, 38, 48)
    } else {
        egui::Color32::WHITE
    };
    painter.rect_filled(rect, ROUNDING, bg);

    // Border
    let border_color = if is_dragged {
        egui::Color32::from_rgb(100, 149, 237)
    } else if is_dark {
        egui::Color32::from_rgb(55, 55, 70)
    } else {
        egui::Color32::from_rgb(220, 220, 230)
    };
    painter.rect_stroke(
        rect,
        ROUNDING,
        egui::Stroke::new(1.0, border_color),
        egui::StrokeKind::Outside,
    );

    // Title bar background
    let title_rect =
        egui::Rect::from_min_size(rect.left_top(), egui::vec2(rect.width(), TITLE_H));
    let title_bg = if is_dark {
        egui::Color32::from_rgb(45, 45, 58)
    } else {
        egui::Color32::from_rgb(245, 245, 250)
    };
    painter.rect_filled(
        title_rect,
        egui::CornerRadius {
            nw: ROUNDING as u8,
            ne: ROUNDING as u8,
            sw: 0,
            se: 0,
        },
        title_bg,
    );

    // Title bar separator
    painter.line_segment(
        [
            egui::pos2(title_rect.left(), title_rect.bottom()),
            egui::pos2(title_rect.right(), title_rect.bottom()),
        ],
        egui::Stroke::new(1.0, border_color),
    );

    // Title text
    let text_color = if is_dark {
        egui::Color32::from_gray(200)
    } else {
        egui::Color32::from_gray(50)
    };
    painter.text(
        egui::pos2(title_rect.left() + 10.0, title_rect.center().y),
        egui::Align2::LEFT_CENTER,
        tile.content.title(),
        egui::FontId::proportional(13.0),
        text_color,
    );

    // Drag-handle grip dots on title bar
    let dot_color = if is_dark {
        egui::Color32::from_gray(90)
    } else {
        egui::Color32::from_gray(180)
    };
    let dx_base = title_rect.right() - 20.0;
    let dy_base = title_rect.center().y;
    for r in 0..3 {
        for c in 0..2 {
            painter.circle_filled(
                egui::pos2(dx_base + c as f32 * 5.0, dy_base + (r as f32 - 1.0) * 5.0),
                1.3,
                dot_color,
            );
        }
    }

    // Resize handle (diagonal lines in bottom-right corner)
    let handle_color = if is_dark {
        egui::Color32::from_gray(75)
    } else {
        egui::Color32::from_gray(195)
    };
    let br = egui::pos2(rect.right() - 5.0, rect.bottom() - 5.0);
    for k in 0..3 {
        let d = (k + 1) as f32 * 4.0;
        painter.line_segment(
            [egui::pos2(br.x - d, br.y), egui::pos2(br.x, br.y - d)],
            egui::Stroke::new(1.0, handle_color),
        );
    }
}

fn render_sparkline(ui: &egui::Ui, rect: egui::Rect) {
    let datasets: &[&[f32]] = &[
        &[3.0, 5.0, 2.0, 8.0, 4.0, 7.0, 3.0, 6.0, 5.0, 9.0, 4.0, 6.0],
        &[9.0, 7.0, 5.0, 6.0, 4.0, 3.0, 5.0, 2.0, 4.0, 6.0, 3.0, 5.0],
        &[2.0, 4.0, 3.0, 5.0, 8.0, 6.0, 7.0, 9.0, 8.0, 7.0, 8.0, 9.0],
    ];
    let colors = [
        egui::Color32::from_rgb(66, 133, 244),
        egui::Color32::from_rgb(234, 67, 53),
        egui::Color32::from_rgb(52, 168, 83),
    ];
    let labels = ["Revenue", "Users", "Growth"];
    let painter = ui.painter();
    let text_color = ui.visuals().text_color();
    let line_h = rect.height() / 3.0;

    for (i, (data, color)) in datasets.iter().zip(colors.iter()).enumerate() {
        let y_off = rect.top() + i as f32 * line_h;

        painter.text(
            egui::pos2(rect.left(), y_off + line_h * 0.5),
            egui::Align2::LEFT_CENTER,
            labels[i],
            egui::FontId::proportional(11.0),
            text_color,
        );

        let spark_rect = egui::Rect::from_min_max(
            egui::pos2(rect.left() + 58.0, y_off + 4.0),
            egui::pos2(rect.right(), y_off + line_h - 4.0),
        );
        draw_sparkline_line(painter, spark_rect, data, *color);
    }
}

fn render_gauge(ui: &egui::Ui, rect: egui::Rect, value: f32) {
    let painter = ui.painter();
    let center = egui::pos2(rect.center().x, rect.center().y + 6.0);
    let radius = rect.width().min(rect.height() - 16.0) / 2.0 - 6.0;

    let start_a = PI * 0.8;
    let sweep = PI * 1.4;

    // Background arc
    draw_arc(
        painter,
        center,
        radius,
        start_a,
        start_a + sweep,
        5.0,
        egui::Color32::from_gray(80),
    );

    // Fill arc
    let fill_end = start_a + sweep * value;
    let fill_color = if value < 0.5 {
        egui::Color32::from_rgb(52, 168, 83)
    } else if value < 0.8 {
        egui::Color32::from_rgb(251, 188, 4)
    } else {
        egui::Color32::from_rgb(234, 67, 53)
    };
    draw_arc(painter, center, radius, start_a, fill_end, 5.0, fill_color);

    // Tick marks
    for k in 0..=10 {
        let t = k as f32 / 10.0;
        let a = start_a + sweep * t;
        let r_in = radius - 9.0;
        let r_out = radius - 3.0;
        painter.line_segment(
            [
                egui::pos2(center.x + a.cos() * r_in, center.y + a.sin() * r_in),
                egui::pos2(center.x + a.cos() * r_out, center.y + a.sin() * r_out),
            ],
            egui::Stroke::new(1.2, egui::Color32::from_gray(140)),
        );
    }

    // Needle
    let needle_a = start_a + sweep * value;
    let needle_len = radius - 16.0;
    let needle_end = egui::pos2(
        center.x + needle_a.cos() * needle_len,
        center.y + needle_a.sin() * needle_len,
    );
    let tc = ui.visuals().text_color();
    painter.line_segment([center, needle_end], egui::Stroke::new(2.0, tc));
    painter.circle_filled(center, 4.0, tc);

    // Value label
    painter.text(
        egui::pos2(center.x, center.y + 18.0),
        egui::Align2::CENTER_CENTER,
        format!("{}%", (value * 100.0) as u32),
        egui::FontId::proportional(14.0),
        tc,
    );
}

fn render_stat_card(ui: &egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let cx = rect.center().x;
    let cy = rect.center().y;
    let tc = ui.visuals().text_color();

    painter.text(
        egui::pos2(cx, cy - 18.0),
        egui::Align2::CENTER_CENTER,
        "1,284",
        egui::FontId::proportional(34.0),
        tc,
    );
    painter.text(
        egui::pos2(cx, cy + 10.0),
        egui::Align2::CENTER_CENTER,
        "Active Users",
        egui::FontId::proportional(13.0),
        egui::Color32::from_gray(130),
    );
    painter.text(
        egui::pos2(cx, cy + 30.0),
        egui::Align2::CENTER_CENTER,
        "\u{2191} 12.3%",
        egui::FontId::proportional(14.0),
        egui::Color32::from_rgb(52, 168, 83),
    );
}

fn render_text_log(ui: &mut egui::Ui, _idx: usize, rect: egui::Rect) {
    let log_lines: &[(&str, egui::Color32)] = &[
        ("[INFO]  System initialized", egui::Color32::PLACEHOLDER),
        ("[DEBUG] Loading config...", egui::Color32::from_gray(130)),
        ("[INFO]  Connected to DB", egui::Color32::PLACEHOLDER),
        ("[WARN]  High memory usage", egui::Color32::from_rgb(251, 188, 4)),
        ("[INFO]  Batch #1284 done", egui::Color32::PLACEHOLDER),
        ("[DEBUG] Cache hit: 94.2%", egui::Color32::from_gray(130)),
        ("[INFO]  API 200 OK", egui::Color32::PLACEHOLDER),
        ("[ERROR] Timeout /api/data", egui::Color32::from_rgb(234, 67, 53)),
        ("[INFO]  Retry succeeded", egui::Color32::PLACEHOLDER),
        ("[DEBUG] GC 12ms", egui::Color32::from_gray(130)),
        ("[INFO]  Metrics exported", egui::Color32::PLACEHOLDER),
        ("[WARN]  Rate limit near", egui::Color32::from_rgb(251, 188, 4)),
    ];
    let text_color = ui.visuals().text_color();
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
    egui::ScrollArea::vertical()
        .max_height(rect.height())
        .show(&mut child, |ui: &mut egui::Ui| {
            for &(line, color) in log_lines {
                let c = if color == egui::Color32::PLACEHOLDER {
                    text_color
                } else {
                    color
                };
                ui.label(egui::RichText::new(line).monospace().size(10.5).color(c));
            }
        });
}

// --- Drawing helpers ---

fn draw_arc(
    painter: &egui::Painter,
    center: egui::Pos2,
    radius: f32,
    start: f32,
    end: f32,
    width: f32,
    color: egui::Color32,
) {
    let n = 30;
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

fn draw_sparkline_line(
    painter: &egui::Painter,
    rect: egui::Rect,
    data: &[f32],
    color: egui::Color32,
) {
    if data.len() < 2 {
        return;
    }
    let min_v = data.iter().copied().reduce(f32::min).unwrap_or(0.0);
    let max_v = data.iter().copied().reduce(f32::max).unwrap_or(1.0);
    let range = (max_v - min_v).max(0.01);

    let pts: Vec<egui::Pos2> = data
        .iter()
        .enumerate()
        .map(|(j, &v)| {
            let x = rect.left() + (j as f32 / (data.len() - 1) as f32) * rect.width();
            let y = rect.bottom() - ((v - min_v) / range) * rect.height();
            egui::pos2(x, y)
        })
        .collect();

    for w in pts.windows(2) {
        painter.line_segment([w[0], w[1]], egui::Stroke::new(1.5, color));
    }
}
