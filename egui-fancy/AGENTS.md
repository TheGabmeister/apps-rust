# egui Showcase - Agent Context

## File Access Rules
Do not read `CLAUDE.md` unless the user explicitly asks for it or the task specifically depends on it.
Treat `SPEC.md` and `MILESTONES.md` as the source of truth.

## Purpose
This repository is a desktop-only `egui` / `eframe` showcase app meant to look polished enough to serve as a portfolio piece. It is not a tutorial app, and it is not a benchmark or stress test.

The canonical product description lives in `SPEC.md`. The milestone breakdown and locked interfaces live in `MILESTONES.md`.

## Read These First
When starting a new session, read these in this order:

1. `SPEC.md`
2. `MILESTONES.md`
3. `src/app.rs`
4. `src/animation.rs`
5. `src/sections/mod.rs`
6. The specific section file you are modifying

`SPEC.md` and `MILESTONES.md` are the real source of truth.

## Build / Verify
Use:

```bash
cargo run
cargo clippy
```

If making a focused change, also verify the affected section manually in the app.

## Hard Constraints
- Do not add new third-party crates unless the user explicitly changes the spec.
- Keep this as a native desktop app.
- The sidebar is intentionally an `egui::Area` overlay, not a `SidePanel`.
- The app should repaint on demand, mainly while animations are active.
- Theme support is only light/dark using built-in `egui::Visuals` with small tweaks.
- Preserve the locked interface contract from `MILESTONES.md` unless the user explicitly approves changing it.

## Current Code Map
- `src/main.rs`: window setup and `eframe::run_native`
- `src/app.rs`: `FancyShowcaseApp`, sidebar, section routing, theme application
- `src/animation.rs`: shared animation helper and easing curves
- `src/theme.rs`: current light/dark theme application
- `src/sections/buttons.rs`: implemented stock vs custom button demos
- `src/sections/sliders.rs`: implemented stock vs custom input demos
- `src/sections/data_viz.rs`: implemented plot + painter visualizations
- `src/sections/dashboard.rs`: implemented resizable/reorderable dashboard tile grid with custom-painted tile chrome
- `src/sections/panels.rs`: implemented panel/navigation demos with animated tab underline, breadcrumbs, file tree, and custom split pane
- `src/sections/transitions.rs`: implemented transition demos, easing visualizer, and before/after animation comparison

## Existing Conventions
- Section state lives in section structs owned by `FancyShowcaseApp`.
- All section structs currently implement `Default`.
- `DashboardSection`, `PanelsSection`, and `TransitionsSection` now own non-trivial interactive state; do not treat them as placeholders or unit structs.
- Most custom visuals are painted directly with `Painter`; keep them section-local instead of inventing a global theme/chrome system.
- The project currently keeps logic simple and local rather than building a deep abstraction layer.

## Important API Notes
- `egui_plot` is on `0.34.1`; constructors like `Line::new(name, points)` and `BarChart::new(name, bars)` take the name first.
- `Painter::rect_stroke()` needs `egui::StrokeKind::Outside`.
- `lerp_color()` already exists in `src/sections/buttons.rs` and is public.
- `Animation` currently stores `start_time: Option<f64>`, `duration`, `easing`, and a `forward` flag. New animation work should reuse this helper instead of inventing another timing system.

## What Is Actually Implemented
- `buttons.rs`: stock button widgets plus hover glow, click ripple, animated toggle, and sliding button group
- `sliders.rs`: stock inputs plus a custom range slider, rotary knob, gradient progress bar, and focus-glow text input
- `data_viz.rs`: line/bar/area charts with `egui_plot`, plus painter-drawn radial gauge, sparklines, and donut chart
- `dashboard.rs`: reorderable/resizable 3-column tile grid with sparkline, gauge, stat card, mini controls, and text log tiles
- `panels.rs`: idiomatic panel layout demo, animated navigation tab bar, breadcrumb trail, collapsible file tree, and draggable split pane
- `transitions.rs`: fade/slide/scale/combo transition demos, easing curve visualizer, and instant-vs-animated comparison

M1-M4 are implemented. M5 polish work remains open in `MILESTONES.md`. If touching completed sections, prefer incremental improvement over rewriting them from scratch.

## Recommended Next Work
1. Execute M5 polish work across all six sections
2. Review spacing, theme consistency, and small-window behavior against `SPEC.md`
3. Manually verify the more interaction-heavy sections (`dashboard`, `panels`, `transitions`) after targeted changes
4. Keep `cargo clippy` clean after each change set

## Editing Guidance For Future Sessions
- Match the existing project structure before adding abstractions.
- Check `git status` before editing.
- Do not rewrite completed sections just to make them "cleaner" unless that is necessary for the task.
- If the spec and current code disagree, prefer the spec unless the user has already accepted the deviation.
- Keep `main.rs` minimal. UI work belongs elsewhere.

## Suggested Session Opening
If a new session starts without much context, first:

1. Read `SPEC.md` and `MILESTONES.md`
2. Inspect `src/app.rs` and the target section file
3. Run `cargo clippy` only if the task depends on current code health
4. Then implement the requested change with minimal churn
