# egui Showcase — Milestones

## Status Legend
- [ ] Not started
- [~] In progress
- [x] Complete

---

## M1 — Foundation
**Goal**: A running shell with sidebar navigation, animation system, and theme toggle. No section content yet — just the scaffolding every other milestone depends on.

**Must be completed before any parallel work begins.**

### Deliverables
- [ ] `main.rs` — `eframe::run_native` with 1400×900 window, "egui Showcase" title
- [ ] `app.rs` — `FancyShowcaseApp` struct, `eframe::App::update()` routing
- [ ] `animation.rs` — `Animation` struct + 6 easing curves (linear, ease-in-out cubic, ease-out cubic, ease-out elastic, ease-out bounce, ease-in quad)
- [ ] `theme.rs` — light/dark `Visuals` configuration, theme toggle
- [ ] `sections/mod.rs` — `Section` enum with all 6 variants
- [ ] Collapsible overlay sidebar — icon+label expanded, icon-only collapsed, animated with ease-out-cubic
- [ ] Placeholder content area — each section shows a "Coming soon" panel until implemented
- [ ] Clean `cargo run` showing navigable shell

### Interface Contract (locked after M1)
All parallel agents in M2/M4 must work against these signatures without changing them:

```rust
// animation.rs
pub enum Easing { Linear, EaseInOutCubic, EaseOutCubic, EaseOutElastic, EaseOutBounce, EaseInQuad }
pub struct Animation { ... }
impl Animation {
    pub fn new(duration: f32, easing: Easing) -> Self;
    pub fn start(&mut self, ctx: &egui::Context);
    pub fn reverse(&mut self, ctx: &egui::Context);
    pub fn progress(&self, ctx: &egui::Context) -> f32;
    pub fn is_active(&self, ctx: &egui::Context) -> bool;
}

// sections/mod.rs
pub enum Section { Buttons, Sliders, DataViz, Dashboard, Panels, Transitions }
pub trait SectionView {
    fn title(&self) -> &str;
    fn icon(&self) -> &str;
    fn show(&mut self, ui: &mut egui::Ui);
}

// app.rs
pub struct AppState {
    pub active_section: Section,
    pub is_dark_mode: bool,
    pub sidebar_expanded: bool,
    pub sidebar_animation: Animation,
    pub buttons: sections::buttons::ButtonsSection,
    pub sliders: sections::sliders::SlidersSection,
    pub data_viz: sections::data_viz::DataVizSection,
    pub dashboard: sections::dashboard::DashboardSection,
    pub panels: sections::panels::PanelsSection,
    pub transitions: sections::transitions::TransitionsSection,
}
```

---

## M2 — Sections 1–3

- [ ] **Buttons & Interactions** (`sections/buttons.rs`)
  - Stock widgets: button, small_button, toggle_value, selectable_label, radio_value, hyperlink
  - Custom: hover glow effect, click ripple, animated toggle switch, sliding button group indicator
  - Two-column layout: stock left, custom right
- [ ] **Sliders & Inputs** (`sections/sliders.rs`)
  - Stock widgets: Slider (H+V), DragValue, TextEdit (single + multi), color_edit_button_rgba, ComboBox, checkbox, spinner
  - Custom: two-thumb range slider, rotary knob, gradient progress bar, focus-glow text input
  - Grid layout: labeled rows, stock and custom side by side
- [ ] **Data Visualization** (`sections/data_viz.rs`)
  - egui_plot charts: line (sine waves), bar (random-walk histogram), area (stacked sines)
  - Slider controls frequency/amplitude in real time
  - Painter-drawn: radial gauge (animated needle), sparklines, animated donut chart
- [ ] Wire all three sections into `AppState` in `app.rs`
- [ ] `cargo run` — sections 1–3 fully navigable

---

## M3 — Dashboard Grid (Sequential)
**Goal**: The most complex section — fully interactive resizable and reorderable tile grid.

**Single agent. Do not parallelize.**

- [ ] `TileId`, `Tile { id, grid_rect, content_type }` types
- [ ] Initial layout: 3-column grid, mixed 1×1 and 2×1 tiles
- [ ] Tile content variants: sparkline, gauge, stat card, mini controls, text log
- [ ] **Resize**: drag handles on tile edges/corners, adjacent tiles adjust, minimum size enforced
- [ ] **Reorder**: drag by title bar, placeholder outline at drop position, tiles animate to make room
- [ ] Tile painting: rounded borders, drop shadows, elevated shadow on drag
- [ ] `cargo run` — dashboard section fully interactive

---

## M4 — Sections 5–6

- [ ] **Panels & Navigation** (`sections/panels.rs`)
  - Idiomatic: TopBottomPanel, SidePanel, CentralPanel, nested ScrollArea, CollapsingHeader accordion
  - Navigation patterns: tab bar with animated sliding underline, breadcrumb trail, collapsible file tree
  - Exotic: split pane with draggable divider via `allocate_rect()` + pointer tracking, labeled "Custom"
- [ ] **Animated Transitions** (`sections/transitions.rs`)
  - Transition demos: fade, slide (H+V), scale, fade+slide combo — each with Play + Reverse buttons
  - Easing curve visualizer: line graph per curve, animated dot, moving square showing motion feel
  - Before/after comparison: instant vs animated state change
- [ ] Wire both sections into `AppState`
- [ ] `cargo run` — all 6 sections navigable and complete

---

## M5 — Polish
**Goal**: Final visual pass, consistency check, any rough edges smoothed.

- [ ] Consistent spacing and padding across all sections
- [ ] Sidebar icons reviewed (legible at narrow width)
- [ ] Light/dark theme verified across all sections (no hardcoded colors)
- [ ] Window min-size (800×600) tested — layout degrades gracefully
- [ ] `cargo clippy` clean
- [ ] Final `cargo run` end-to-end review
