# egui Showcase - Reviewer Context

## File Access Rules
Do not read `CLAUDE.md` unless the user explicitly asks for it or the task specifically depends on it.

## Purpose
This repository is a desktop-only `egui` / `eframe` showcase app meant to look polished enough to serve as a portfolio piece. It is not a tutorial app, and it is not a benchmark or stress test.

## Read These First
When starting a new session, read these in this order:

1. `SPEC.md` if it exists.
2. `MILESTONES.md` if it exists.
3. `src/app.rs`
4. `src/animation.rs`
5. `src/sections/mod.rs`
6. The specific section file you are reviewing

## Build / Verify
Use:

```bash
cargo run
cargo clippy
```

If a review is focused on one section, also verify that section manually in the app.

## Project Constraints
- This is a native desktop app built with `egui` / `eframe`.
- The sidebar is intentionally an `egui::Area` overlay, not a `SidePanel`.
- The app repaints on demand, mainly while animations are active.
- Theme support is limited to light/dark with built-in `egui::Visuals`; `src/theme.rs` currently just swaps the stock visuals.
- Any new external dependency still requires user approval.

## Current Code Map
- `src/main.rs`: window setup and `eframe::run_native`
- `src/app.rs`: `FancyShowcaseApp`, sidebar, section routing, theme application
- `src/animation.rs`: shared animation helper and easing curves
- `src/theme.rs`: swaps the stock light/dark `egui::Visuals`
- `src/sections/buttons.rs`: implemented stock vs custom button demos
- `src/sections/sliders.rs`: implemented stock vs custom input demos
- `src/sections/data_viz.rs`: implemented plot + painter visualizations
- `src/sections/dashboard.rs`: implemented resizable/reorderable dashboard tile grid with custom-painted tile chrome
- `src/sections/panels.rs`: implemented simulated panel layout, navigation patterns, and a custom split pane
- `src/sections/transitions.rs`: implemented transition demos, easing visualizer, and before/after animation comparison

## Structural Notes
- Section state lives in section structs owned by `FancyShowcaseApp`.
- All section structs currently implement `Default`.
- `DashboardSection`, `PanelsSection`, and `TransitionsSection` own non-trivial interactive state.
- Most custom visuals are implemented directly with section-local `Painter` code.
- The codebase is still fairly local and lightweight rather than deeply abstracted.

## Important API Notes
- `egui_plot` is on `0.34.1`; constructors like `Line::new(name, points)` and `BarChart::new(name, bars)` take the name first.
- `Painter::rect_stroke()` needs `egui::StrokeKind::Outside`.
- `lerp_color()` already exists in `src/sections/buttons.rs` and is public.
- `Animation` currently stores `start_time: Option<f64>`, `duration`, `easing`, and a `forward` flag.

## What Is Actually Implemented
- `buttons.rs`: stock button widgets plus hover glow, click ripple, animated toggle, and sliding button group
- `sliders.rs`: stock inputs plus a custom range slider, rotary knob, gradient progress bar, and focus-glow text input
- `data_viz.rs`: line/bar/area charts with `egui_plot`, plus painter-drawn radial gauge, sparklines, and donut chart
- `dashboard.rs`: reorderable/resizable 3-column tile grid with sparkline, gauge, stat card, mini controls, and text log tiles
- `panels.rs`: idiomatic panel layout demo, animated navigation tab bar, breadcrumb trail, collapsible file tree, and draggable split pane
- `transitions.rs`: fade/slide/scale/combo transition demos, easing curve visualizer, and instant-vs-animated comparison

## Review Guidance
- If `SPEC.md` or `MILESTONES.md` exist and disagree with the code, call out the mismatch explicitly.

## Suggested Session Opening
If a new session starts without much context, first:

1. Read `SPEC.md` and `MILESTONES.md` if they exist. 
2. Inspect `src/app.rs` and the target section file
3. Run `cargo clippy` only if the task depends on current code health
4. Then compare the instructions or claims under review against the current code with minimal churn
