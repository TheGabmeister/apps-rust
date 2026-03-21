# egui Showcase — Specification

## Overview

A portfolio-quality egui/eframe widget gallery that demonstrates egui's capabilities when pushed to its limits. The app is a categorized, visually polished showcase of buttons, sliders, panels, layouts, data visualizations, animations, and custom-painted widgets — organized as a 6-section gallery with a collapsible sidebar.

**Target**: Desktop only (native)
**Goal**: Maximum visual polish — a portfolio piece, not a learning tool or stress test
**Window**: 1400×900, titled "egui Showcase"

---

## Architecture

### Platform & Dependencies

| Crate | Purpose |
|---|---|
| `eframe` 0.33.x | Application framework, window management |
| `egui_extras` | Tables, date pickers, image loading |
| `egui_plot` | Line/bar/area charts with axes, legends, zoom |

No other third-party crates. All custom widgets and effects built with egui's Painter API.

### Theming

Light and dark modes only, using egui's built-in `Visuals` with minor tweaks for polish. A toggle in the sidebar header switches between them. No custom theme system.

### Performance

No guardrails or throttling. On-demand repaint only — `ctx.request_repaint()` is called only when animations are actively running. When idle (no hover, no drag, no transition in progress), the app does not repaint. This keeps CPU usage near zero when the user isn't interacting, while allowing full-speed rendering during interactions.

---

## Animation System

### Reusable Animation Helper

A shared `Animation` struct that any widget can use to drive timed transitions. Stored in widget state, tracks start time and progress.

```
Animation {
    start_time: f64,
    duration: f32,
    easing: EasingFn,
    direction: Forward | Reverse,
}
```

**Core API**:
- `Animation::new(duration, easing)` — create an animation
- `animation.start(ctx)` — begin (or restart), calls `ctx.request_repaint()`
- `animation.progress(ctx) -> f32` — returns 0.0..=1.0 with easing applied; continues requesting repaint until complete
- `animation.is_active() -> bool` — whether animation is in progress

### Easing Curves (5-6)

| Curve | Use case |
|---|---|
| Linear | Uniform motion, progress bars |
| Ease-in-out (cubic) | General-purpose smooth transitions |
| Ease-out-cubic | Decelerating motion (sidebar slide, fade-in) |
| Ease-out-elastic | Overshoot + settle (bouncy button press) |
| Ease-out-bounce | Discrete bounces (playful element drops) |
| Ease-in-quad | Accelerating motion (fade-out, exit transitions) |

All easing functions are pure `fn(f32) -> f32` taking normalized time (0.0–1.0) and returning eased progress.

---

## Navigation

### Collapsible Overlay Sidebar

The sidebar is the primary navigation element. It has two states:

- **Expanded**: Shows icon + text label for each section, plus theme toggle and app title
- **Collapsed**: Shows icon only, narrow width

**Behavior**: The sidebar overlays the content (does not push it). When expanded, it floats over the gallery content with a subtle shadow/backdrop. The collapse/expand transition is animated using the animation helper with ease-out-cubic.

**Toggle**: A hamburger/arrow icon button at the top of the sidebar triggers the transition.

**Active section**: Highlighted with a distinct background color. Clicking a section navigates to it.

---

## Gallery Sections

### 1. Buttons & Interactions

Demonstrates every button variant egui supports, plus custom-painted enhancements.

**Stock egui widgets** (shown for comparison):
- `ui.button()`, `ui.small_button()`
- `ui.toggle_value()` (checkbox-style toggle)
- `ui.selectable_label()` / `ui.selectable_value()`
- `ui.radio_value()`
- `ui.hyperlink()`, `ui.link()`
- Button with icon (using egui's built-in emoji/unicode)

**Custom-painted enhancements** (section-specific chrome):
- **Hover glow**: Buttons emit a soft colored glow on hover, painted as a larger rounded rect behind the button with reduced opacity, animated fade-in
- **Click ripple**: On click, a circular ripple expands from the click point and fades out, painted with `Painter::circle()` at decreasing opacity
- **Animated toggle switch**: Custom-painted toggle that slides a circle indicator with ease-out-elastic, inspired by iOS/Material toggles
- **Button group**: Row of buttons where the active one has an animated background indicator that slides between positions

**Layout**: Two-column layout — left column shows stock widgets with labels, right column shows the enhanced versions side by side.

### 2. Sliders & Inputs

Demonstrates input widgets with custom-painted alternatives.

**Stock egui widgets**:
- `ui.add(egui::Slider::new(...))` — horizontal and vertical
- `ui.add(egui::DragValue::new(...))` — numeric drag input
- `egui::TextEdit::singleline()` and `multiline()`
- `egui::color_picker::color_edit_button_rgba()`
- `egui::ComboBox`
- `ui.checkbox()`
- `ui.spinner()`

**Custom-painted enhancements**:
- **Styled range slider**: Two-thumb slider for selecting a range, track filled between thumbs with a gradient, painted with `Painter`
- **Rotary knob**: Circular knob widget drawn with `Painter::circle()` and an arc indicator. Drag to rotate. Value displayed in center
- **Custom progress bar**: Gradient-filled progress bar with rounded caps and animated fill, using `Painter::rect_filled()` with a mesh for the gradient
- **Animated text input**: Text field with an animated focus border that glows on focus using the animation helper

**Layout**: Grid layout — widgets arranged in labeled rows, stock and custom side by side.

### 3. Data Visualization

Demonstrates charting and data display using egui_plot for structured charts and Painter for custom gauges.

**egui_plot charts** (procedural math data):
- **Line chart**: Multiple sine waves with different frequencies/phases, with legend
- **Bar chart**: Random-walk histogram that regenerates on button press
- **Area chart**: Stacked area chart from layered sine functions

All charts use procedural data: sine waves (`(time * freq).sin()`), noise (simple pseudo-random), and random walks (cumulative random steps). Data updates smoothly on interaction (e.g., a slider controls frequency, dragging pans the view).

**Painter-drawn custom viz**:
- **Radial gauge**: Speedometer-style arc gauge drawn with `Painter::add(Shape::Path(...))`. Needle animates to target value with ease-out-elastic. Tick marks and labels painted manually
- **Sparklines**: Tiny inline line charts drawn with `Painter::line_segment()` chains. Shown in a row, each with different data
- **Animated donut chart**: Concentric arcs representing proportions, drawn as path segments. On value change, arcs animate to new sizes

**Layout**: Top row: egui_plot charts in equal columns. Bottom row: Painter-drawn custom viz in a flex row.

### 4. Dashboard Grid

A resizable, reorderable grid of tiles — the exotic layout showcase.

**Grid behavior**:
- Tiles are rectangular cards arranged in a grid
- Each tile has a title bar and content area
- **Resize**: Drag tile edges/corners to resize. Hit-test a ~4px border zone. During drag, adjacent tiles adjust to fill or shrink. Minimum tile size enforced
- **Reorder**: Drag a tile by its title bar to move it. A placeholder outline shows the drop position. Other tiles animate to make room. Drop snaps to grid alignment

**State management**:
- Grid state: `Vec<Tile>` where each `Tile` has `id, rect (grid units), content_type`
- Active drag state: which tile is being dragged, original position, current pointer offset
- Resize state: which edge/corner, original rect, pointer delta

**Tile contents** (demonstrating widgets-in-context):
- A sparkline tile (Painter-drawn)
- A gauge tile (radial gauge from Data Viz)
- A stat card tile (large number + label + trend arrow)
- A mini controls tile (a few sliders/toggles)
- A text log tile (scrolling text area)

**Initial layout**: Pre-defined 3-column arrangement with mixed tile sizes (some span 2 columns, some are 1×1).

**Painting**: Tiles have subtle rounded borders and drop shadows (painted behind the tile rect). The dragged tile is painted at elevated opacity/shadow to show it's "lifted."

### 5. Panels & Navigation

Demonstrates egui's panel system and navigation patterns, comparing idiomatic and exotic approaches.

**Idiomatic egui panels**:
- `TopBottomPanel::top()` — app header bar
- `SidePanel::left()` — navigation panel (this is what the gallery sidebar uses)
- `SidePanel::right()` — properties/inspector panel
- `CentralPanel` — main content area
- Nested `ScrollArea` inside panels
- `CollapsingHeader` for accordion sections

**Navigation patterns**:
- **Tab bar**: Horizontal tabs built with `ui.selectable_label()` in a horizontal layout, with an animated active indicator (a colored underline that slides between tabs)
- **Breadcrumbs**: Clickable path segments (`Home > Section > Item`) with `>` separators
- **Collapsible tree**: Nested `CollapsingHeader` elements representing a file tree structure

**Exotic layout comparison**:
- **Split pane**: Two content areas separated by a draggable divider. Drag to resize proportions. The divider is a thin painted line with a grab handle. This is done with manual `allocate_rect()` and pointer tracking — not a built-in egui feature
- Side-by-side label: "Idiomatic" vs "Custom" markers on each panel demo so the viewer knows which is native and which is hand-built

### 6. Animated Transitions

Showcases the animation system itself and demonstrates transition patterns.

**Transition demos**:
- **Fade**: Widget opacity animates from 0 to 1 (simulated via alpha blending on a painted overlay)
- **Slide**: Widget position animates from off-screen to final position (horizontal and vertical variants)
- **Scale**: Widget grows from 0% to 100% size (using `ui.set_min_size()` animated over time)
- **Combination**: Fade + slide together for a "material-style" entrance

Each demo has a "Play" button that triggers the animation and a "Reverse" button to play it backward.

**Easing curve visualizer**:
- Shows all 5-6 easing curves as line graphs (X = time, Y = progress)
- A dot animates along each curve in real-time when "Play" is pressed
- Below each curve: a square element that moves from left to right using that easing, so the viewer can see the motion feel
- Dropdown or radio buttons to select which easing to preview in isolation

**Before/after comparison**:
- A UI element shown twice: once with no animation (instant state change), once with the animated transition
- Toggle to switch between the two, making the value of animation viscerally clear

---

## Code Structure

```
src/
  main.rs                 - eframe::run_native entry point, NativeOptions config
  app.rs                  - FancyShowcaseApp struct, impl eframe::App, sidebar, section routing
  animation.rs            - Animation struct, EasingFn type, all easing curve functions
  theme.rs                - Light/dark Visuals configuration, theme toggle state
  sections/
    mod.rs                - Section enum, trait or display function signature
    buttons.rs            - Section 1: Buttons & Interactions
    sliders.rs            - Section 2: Sliders & Inputs
    data_viz.rs           - Section 3: Data Visualization
    dashboard.rs          - Section 4: Dashboard Grid (tile state, drag/resize logic)
    panels.rs             - Section 5: Panels & Navigation
    transitions.rs        - Section 6: Animated Transitions + easing visualizer
```

### Module Responsibilities

- **main.rs**: Only configures `NativeOptions` (window size 1400×900, title "egui Showcase") and calls `eframe::run_native()`. No UI code
- **app.rs**: Owns the `FancyShowcaseApp` struct containing all state. Implements `eframe::App::update()` which renders the sidebar and delegates to the active section. Manages sidebar collapse state and animation
- **animation.rs**: Self-contained animation module. No egui dependency beyond `egui::Context` for time. Easing functions are pure math
- **theme.rs**: Provides functions to apply light/dark `Visuals` to `ctx.set_visuals()`. Holds the `is_dark: bool` state
- **sections/**: Each file owns its section's state and rendering. Sections receive `&mut Ui` and `&mut AppState` (or relevant subset). Each section is as self-contained as possible

### State Management

The `FancyShowcaseApp` struct holds:
- `active_section: Section` — which gallery section is shown
- `sidebar_expanded: bool` + `sidebar_animation: Animation`
- `is_dark_mode: bool`
- Per-section state structs (e.g., `DashboardState`, `DataVizState`, `TransitionDemoState`)

Per-section state is owned by the app but passed to section rendering functions. This keeps state alive across section switches (navigating away and back preserves slider positions, dashboard layout, etc.).

---

## Window Configuration

```rust
eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([1400.0, 900.0])
        .with_title("egui Showcase")
        .with_min_inner_size([800.0, 600.0]),
    ..Default::default()
}
```

Minimum size of 800×600 prevents layout breakage. The app does not need to be fully responsive — 1400×900 is the designed viewport — but should degrade gracefully if resized smaller.

---

## Custom Painting Scope

Custom Painter effects are **section-specific**, not global. Most of the app uses stock egui styling. Custom chrome (glow borders, gradient fills, ripple effects) appears only in:

- **Buttons & Interactions**: Hover glow, click ripple, animated toggle, sliding button group indicator
- **Sliders & Inputs**: Range slider, rotary knob, gradient progress bar, focus glow
- **Data Visualization**: Radial gauge, sparklines, donut chart
- **Dashboard Grid**: Tile shadows, drag elevation effect
- **Animated Transitions**: Fade overlay, animated position/scale

The **Panels & Navigation** section intentionally uses mostly stock egui to show what the framework provides natively, with the split pane as the one custom-painted exception.

This contrast between stock and custom-painted sections is a deliberate showcase strategy — viewers can see both what egui gives you for free and what you can build on top of it.
