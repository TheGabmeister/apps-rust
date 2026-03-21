# egui Showcase — Claude Context

## Project Summary
A portfolio-quality egui/eframe widget gallery. Desktop-only native app (1400x900, min 800x600) with a 6-section gallery and collapsible overlay sidebar. All milestones (M1–M5) are COMPLETE.

## Build & Run
```bash
cargo run          # launch the app
cargo clippy       # lint — must be clean
```

## Dependencies
- `eframe` 0.33.3, `egui` 0.33.3, `egui_extras` 0.33.3, `egui_plot` 0.34.1
- No other third-party crates allowed
- Rust edition 2024

## Code Structure
```
src/
  main.rs                 - eframe::run_native entry point only
  app.rs                  - FancyShowcaseApp struct, sidebar, section routing
  animation.rs            - Animation struct + 6 easing curves (shared by all sections)
  theme.rs                - Light/dark Visuals toggle
  sections/
    mod.rs                - Section enum (Buttons, Sliders, DataViz, Dashboard, Panels, Transitions)
    buttons.rs            - Section 1: stock + custom buttons (hover glow, ripple, toggle, button group)
    sliders.rs            - Section 2: stock + custom inputs (range slider, knob, progress bar, focus glow)
    data_viz.rs           - Section 3: egui_plot charts + painter-drawn gauge/sparklines/donut
    dashboard.rs          - Section 4: resizable/reorderable tile grid (sparkline, gauge, stat card, controls, log)
    panels.rs             - Section 5: idiomatic panels, tab bar, breadcrumbs, file tree, split pane
    transitions.rs        - Section 6: transition demos, easing visualizer, before/after comparison
```

## Section Details
1. **Buttons & Interactions** — Two-column: stock egui widgets (button, toggle, selectable, radio, hyperlink) left, custom-painted (hover glow, click ripple, animated toggle switch, sliding button group) right
2. **Sliders & Inputs** — Two-column: stock (slider H/V, DragValue, TextEdit, color picker, ComboBox, checkbox, spinner) left, custom (range slider, rotary knob, gradient progress bar, focus-glow input) right
3. **Data Visualization** — Top row: egui_plot line/bar/area charts with frequency/amplitude sliders. Bottom row: Painter-drawn radial gauge, sparklines, animated donut chart
4. **Dashboard Grid** — 3-column resizable/reorderable tile grid. 5 content types: sparkline, gauge, stat card, mini controls, text log. Drag title bars to reorder, drag edges/corners to resize
5. **Panels & Navigation** — 3 tabs: Idiomatic (simulated TopBottom/Side/CentralPanel, ScrollArea, CollapsingHeader accordion), Navigation (tab bar with animated underline, breadcrumbs, file tree), Custom (split pane with draggable divider)
6. **Animated Transitions** — Transition demos (fade, slide H/V, scale, fade+slide) with Play/Reverse. Easing visualizer (3x2 grid of all curves with animated dot + moving square). Before/after comparison (instant vs animated)

## Key Conventions
- Sections own their state as struct fields with `Default` impl; state is held in `FancyShowcaseApp`
- Sidebar is `egui::Area`-based overlay (NOT `SidePanel`)
- Animation API: `Animation::new(duration, easing)`, `.start(ctx)`, `.reverse(ctx)`, `.progress(ctx)`, `.is_active(ctx)`
- Easing enum variants: `Linear`, `EaseInOutCubic`, `EaseOutCubic`, `EaseOutElastic`, `EaseOutBounce`, `EaseInQuad`
- `Easing::ALL` and `Easing::label()` for iterating/displaying all easing variants
- `lerp_color()` helper is in `sections::buttons` (pub)
- egui_plot 0.34 API: `Line::new(name, points)` and `BarChart::new(name, bars)` — name is first arg
- `Painter::rect_stroke()` requires 4th arg `egui::StrokeKind::Outside`
- `egui::Margin::same()` and `Margin::symmetric()` take `i8`, not `f32` — use integer literals (e.g., `Margin::same(8)`)
- All custom-painted colors must adapt to `ui.visuals().dark_mode` — no hardcoded grays
- Consistent section layout: heading → 8px space → description label → 12px space → content

## Interface Contract (locked)
The `FancyShowcaseApp` struct fields and `Animation` API signatures are frozen:
```rust
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
```
