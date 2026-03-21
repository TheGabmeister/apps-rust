# iced-fancy — Specification

A desktop-only showcase application that pushes iced 0.14 to its absolute limits, demonstrating every widget, layout, canvas, and animation capability available.

---

## Dependencies

| Crate | Version | Features |
|-------|---------|----------|
| `iced` | 0.14.0 | `canvas`, `tokio` |
| `iced_aw` | 0.13.1 | `tabs`, `tab_bar`, `badge`, `card`, `color_picker`, `date_picker`, `time_picker`, `number_input`, `spinner`, `context_menu`, `wrap`, `sidebar`, `drop_down`, `menu`, `selection_list`, `slide_bar`, `typed_input` |

- **Zero external assets** — everything embedded or procedurally generated. Single binary, no asset folder.
- `tokio` feature required for `iced::time::every` (animation subscriptions).
- `canvas` feature required for Canvas widget and iced_aw's `color_picker`.
- iced_aw's bundled font (`ICED_AW_FONT_BYTES`) registered for internal widget glyphs. For sidebar/tab icons, use Unicode symbols (the iced_aw font does not contain general-purpose icons).

---

## App Structure

### Navigation
- **Sidebar** using iced_aw's `Sidebar` widget (vertical tab list with active tab tracking)
- **No menu bar** — sidebar is the sole navigation mechanism
- **Light/dark toggle** via `toggler` widget (uses built-in `Theme::Light` / `Theme::Dark`)

### Tabs
1. **Widget Gallery** — catalog of all available widgets
2. **Layouts** — layout system demonstrations
3. **PaneGrid** — resizable split-panel playground
4. **Canvas & Art** — procedural generative art + data visualization
5. **Audio Mixer** — connected dashboard where widgets drive real-time waveforms

### Responsiveness
- Fully responsive from 800x600 to ultrawide
- Uses `iced::widget::responsive` to adapt grid column counts and layout flow based on available window size
- No fixed minimum window size enforced, but designed for 800x600+

### Theming
- Light/dark toggle only. No custom theme engine.
- Uses iced's built-in `Theme::Light` and `Theme::Dark` variants.
- Toggle persists across tab switches (global app state).

---

## Tab 1: Widget Gallery

A responsive grid of isolated widget demos. Each demo is a self-contained card showing the widget in action with its current value/state displayed.

### Core iced widgets
- **Button** — default, styled variants, disabled state
- **Slider** — horizontal, with live value display
- **Text Input** — single-line, with placeholder and live echo
- **Toggler** — on/off with label
- **Checkbox** — checked/unchecked with label
- **Radio buttons** — group of 3-4 options
- **Pick List** — dropdown selection
- **Progress Bar** — with a slider to control its value
- **Tooltip** — hover a button to see tooltip text

### iced_aw widgets
- **Card** — header, body, footer sections
- **Badge** — status indicators with different colors
- **Color Picker** — interactive color selection with preview swatch
- **Date Picker** — calendar popup
- **Time Picker** — clock popup
- **Number Input** — numeric entry with increment/decrement
- **Spinner** — loading indicator
- **Context Menu** — right-click on a region to see options

### Layout
- `responsive` widget determines columns: 1 col (<500px), 2 cols (<800px), 3 cols (<1200px), 4 cols (1200px+)
- Each demo wrapped in a styled `container` with a title label
- Vertically scrollable if content exceeds viewport

---

## Tab 2: Layout Demos

Interactive demonstrations of iced's layout system.

### Sections
- **Row & Column** — items with different alignments (start, center, end, fill), spacing, padding
- **Container** — centering, max-width constraints, styled backgrounds
- **Scrollable** — vertical and horizontal scrolling with large content
- **Responsive** — live demo that shows column count changing as you resize the window
- **Wrap** (iced_aw) — horizontal and vertical wrapping of dynamically-added items
- **Nesting** — deeply nested Row/Column/Container compositions showing how layouts compose

### Interactivity
- Sliders to control spacing and padding values live
- Buttons to add/remove items from Wrap demos
- All demos visually respond to window resizing in real-time

---

## Tab 3: PaneGrid Demo

A dedicated playground for iced's `pane_grid` widget.

### Features
- Initial layout: 2-3 panes with different content types
- **Split** — buttons to split any pane horizontally or vertically
- **Close** — close button per pane (disabled on last remaining pane)
- **Resize** — drag borders to resize panes
- **Drag** — drag pane title bars to rearrange
- **Content types** — each pane shows one of: text editor, color swatch, counter, placeholder text

### State
- `pane_grid::State<PaneContent>` where `PaneContent` is an enum of content variants
- Focus tracking for the active pane (highlighted border)

---

## Tab 4: Canvas & Procedural Art

Two sub-sections: generative art and data visualization. Both use `canvas::Program` and animate via subscription.

### Procedural Art
- **Autonomous animation** — art generates and evolves continuously without user interaction
- **Control panel** (sliders + pick list alongside the canvas):
  - Speed slider (0.1x to 3.0x)
  - Complexity slider (controls detail level)
  - Color palette pick list (3-4 palettes: warm, cool, neon, monochrome)
  - Pattern type pick list (spirograph, Lissajous, particle field)
- **Rendering**: `canvas::Program` impl draws to a `Frame` each tick
- **Cache strategy**: `canvas::Cache` for static grid/background only; animated layer redrawn every frame

### Data Visualization
- **Animated bar chart** — bars smoothly transition heights when values change
- **Animated line chart** — scrolling window of data points
- **Controls**: button to randomize data, slider to control animation speed
- **Rendering**: separate `canvas::Program` impl

### Animation
- `iced::time::every(Duration::from_millis(16))` subscription (~60fps)
- Subscription only active when this tab is visible (returns `Subscription::none()` otherwise)
- `tick(Instant)` method on tab state updates time accumulator; delta-time computed from consecutive instants

---

## Tab 5: Audio Mixer / Synthesizer (Connected Dashboard)

The showcase section. Multiple audio channels where every control is cross-wired — changing any parameter instantly updates the real-time waveform visualization.

### Channel Strip (x4 channels)
Each channel has:
- **Volume slider** (vertical fader, 0–100%)
- **EQ sliders** — Treble / Mid / Bass (each 0–100%)
- **Wave type** — pick list: Sine, Square, Sawtooth, Triangle
- **Frequency slider** (20 Hz – 2000 Hz)
- **Amplitude slider** (0.0 – 1.0)
- **Color picker** (iced_aw) — sets the waveform display color for this channel
- **Enable toggler** — mute/unmute the channel
- **Real-time waveform canvas** — small Canvas per channel showing the animated wave

### Master Section
- **Master volume slider**
- **Master waveform canvas** — large Canvas showing the combined waveform of all enabled channels

### Waveform Rendering
- `canvas::Program` impl per waveform display
- Wave math functions:
  - Sine: `sin(t)`
  - Square: `sign(sin(t))`
  - Sawtooth: `fract(t/PI) * 2 - 1`
  - Triangle: `abs(fract(t/PI)) * 4 - 2`
- Phase scrolls continuously based on `time * frequency`
- Amplitude, frequency, and wave type changes reflected instantly
- Channel color from color picker applied to waveform stroke

### Animation
- Shares the same `time::every(16ms)` subscription as the Canvas tab
- Only active when Mixer tab is visible
- Each channel's waveform canvas redraws every tick

### Responsive Layout
- Wide screens: channels laid out horizontally as vertical strips
- Narrow screens: channels stack vertically

---

## Architecture

### Code Organization
```
src/
  main.rs                          — entry point, iced::application builder
  app.rs                           — App state, Message enum, update/view/subscription/theme
  sidebar.rs                       — TabId enum, sidebar construction
  theme.rs                         — theme helpers, custom style closures
  tabs/
    mod.rs                         — re-exports
    gallery/
      mod.rs                       — Gallery state, message, update, view (responsive grid)
      buttons.rs                   — Button demo
      sliders.rs                   — Slider demo
      text_inputs.rs               — TextInput demo
      togglers.rs                  — Toggler demo
      checkboxes.rs                — Checkbox demo
      radios.rs                    — Radio button demo
      pick_lists.rs                — PickList demo
      progress_bars.rs             — ProgressBar demo
      tooltips.rs                  — Tooltip demo
      cards.rs                     — iced_aw Card demo
      badges.rs                    — iced_aw Badge demo
      color_picker.rs              — iced_aw ColorPicker demo
      date_picker.rs               — iced_aw DatePicker demo
      time_picker.rs               — iced_aw TimePicker demo
      number_input.rs              — iced_aw NumberInput demo
      spinner.rs                   — iced_aw Spinner demo
      context_menu.rs              — iced_aw ContextMenu demo
    layouts/
      mod.rs                       — Layouts state, message, update, view
      responsive.rs                — Responsive layout demo
      wrapping.rs                  — Wrap demo
      scrollable.rs                — Scrollable demo
      nesting.rs                   — Nested layout demo
    pane_grid/
      mod.rs                       — PaneGrid state, message, update, view
    canvas/
      mod.rs                       — Canvas tab state, message, update, view
      procedural_art.rs            — Generative art canvas::Program
      data_viz.rs                  — Chart canvas::Program
    mixer/
      mod.rs                       — Mixer state, message, update, view
      channel.rs                   — Channel state and controls
      waveform.rs                  — Waveform canvas::Program
      controls.rs                  — Reusable mixer control components
```

### State Architecture
- `App` holds: `active_tab: TabId`, `is_dark_theme: bool`, and each tab's state struct
- Each tab module defines its own `State`, `Message`, `update()`, and `view()` functions
- `view()` returns `Element<'_, tab::Message>` which gets `.map(Message::TabVariant)` at the app level
- `Message::Tick(Instant)` routed only to the active animated tab

### Subscription Strategy
- `subscription()` checks `active_tab`:
  - `TabId::Canvas | TabId::Mixer` → `iced::time::every(Duration::from_millis(16)).map(Message::Tick)`
  - All other tabs → `Subscription::none()`
- Zero CPU overhead when viewing non-animated tabs

---

## Technical Constraints & Pitfalls

1. **`time::every` requires `tokio` feature** — the `thread-pool` backend has an empty `time` module
2. **`canvas` feature must be enabled** — gates `iced::widget::canvas` and required by iced_aw's `color_picker`
3. **`Message` cannot derive `PartialEq`** — `Instant` doesn't implement it
4. **`TabId` must implement `Eq + Clone`** — required by iced_aw's `Sidebar` widget
7. **`TabBarPosition` is top/bottom only** — left-side navigation must use iced_aw's `Sidebar` widget, not `Tabs`
5. **PaneGrid last-pane guard** — `pane_grid::State::close()` returns `None` for the last pane; must prevent closing it
6. **`responsive` widget lifetime** — closure borrows from state; use references, don't move owned data
7. **Canvas cache invalidation** — only use `canvas::Cache` for static elements; animated layers must be redrawn each frame
8. **iced_aw font** — must register via `.font(iced_aw::ICED_AW_FONT_BYTES)` on the application builder for icon tab labels

---

## Verification Checklist

- [ ] `cargo build` succeeds with no errors
- [ ] `cargo run` opens a responsive window with sidebar tabs
- [ ] All 5 tabs render their full content
- [ ] Light/dark toggle applies across all widgets and tabs
- [ ] Canvas procedural art animates smoothly at ~60fps
- [ ] Mixer waveforms animate in real-time and respond to control changes
- [ ] Animations stop (near-zero CPU) when switching to non-animated tabs
- [ ] Window resize from 800x600 to ultrawide reflows layouts correctly
- [ ] PaneGrid split/close/resize/drag all functional
- [ ] All iced_aw widgets render and interact correctly
- [ ] Single binary with zero external asset dependencies
