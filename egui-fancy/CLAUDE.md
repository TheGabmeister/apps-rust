# egui Showcase — Claude Context

## Project Summary
A portfolio-quality egui/eframe widget gallery. Desktop-only native app (1400x900) with a 6-section gallery and collapsible overlay sidebar. See `SPEC.md` for full specification and `MILESTONES.md` for progress tracking.

## Milestone Status
- **M1 — Foundation**: COMPLETE (app shell, sidebar, animation system, theme toggle, section stubs)
- **M2 — Sections 1–3**: COMPLETE (Buttons & Interactions, Sliders & Inputs, Data Visualization)
- **M3 — Dashboard Grid**: COMPLETE (resizable/reorderable tile grid with 5 content types)
- **M4 — Sections 5–6**: COMPLETE (Panels & Navigation, Animated Transitions)
- **M5 — Polish**: NOT STARTED

## Build & Run
```bash
cargo run          # launch the app
cargo clippy       # lint — must be clean before marking milestones done
```

## Dependencies
- `eframe` 0.33.3, `egui` 0.33.3, `egui_extras` 0.33.3, `egui_plot` 0.34.1
- No other third-party crates allowed per spec
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

## Key Conventions
- Sections own their state as struct fields with `Default` impl; state is held in `FancyShowcaseApp`
- Sidebar is `egui::Area`-based overlay (NOT `SidePanel`) — this is per spec
- Animation API: `Animation::new(duration, easing)`, `.start(ctx)`, `.reverse(ctx)`, `.progress(ctx)`, `.is_active(ctx)`
- Easing enum variants: `Linear`, `EaseInOutCubic`, `EaseOutCubic`, `EaseOutElastic`, `EaseOutBounce`, `EaseInQuad`
- `lerp_color()` helper is in `sections::buttons` (pub)
- egui_plot 0.34 API: `Line::new(name, points)` and `BarChart::new(name, bars)` — name is first arg
- `Painter::rect_stroke()` requires 4th arg `egui::StrokeKind::Outside`
- `egui::Margin::same()` and `Margin::symmetric()` take `i8`, not `f32` — use integer literals (e.g., `Margin::same(8)`)
- `Easing::ALL` and `Easing::label()` are available for iterating/displaying all easing variants
- All sections now have fields and use `::default()` construction

## Interface Contract (from M1, locked)
The `FancyShowcaseApp` struct fields and `Animation` API signatures in `MILESTONES.md` are frozen. New sections must work against these without changing them.
