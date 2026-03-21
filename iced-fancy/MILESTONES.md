# iced-fancy — Milestones

## Milestone 1: Scaffolding
**Status:** [x] Complete

Set up the full project structure with empty tab stubs. The app should compile and open a working window with sidebar navigation before any real tab content exists.

### Deliverables
- `main.rs` — entry point, `iced::application` builder, font registration
- `app.rs` — `App` state, `Message` enum, `update` / `view` / `subscription` / `theme`
- `sidebar.rs` — `TabId` enum, sidebar construction
- `theme.rs` — theme helpers, style closures
- All tab `mod.rs` stubs — `State`, `Message`, `update()`, `view()` returning placeholder content
- `Cargo.toml` — all features enabled

### Visual Check
- [x] Window opens
- [x] Sidebar shows all 5 tab labels with icons
- [x] Clicking each tab switches the active view
- [x] Light/dark toggle works globally

---

## Milestone 2: Widget Gallery
**Status:** [x] Complete

Implement Tab 1 in full — all core iced widgets and all iced_aw widgets, laid out in a responsive grid of demo cards.

### Deliverables
- `tabs/gallery/mod.rs` — `Gallery` state, responsive grid view
- Core widget demos: `buttons.rs`, `sliders.rs`, `text_inputs.rs`, `togglers.rs`, `checkboxes.rs`, `radios.rs`, `pick_lists.rs`, `progress_bars.rs`, `tooltips.rs`
- iced_aw widget demos: `cards.rs`, `badges.rs`, `color_picker.rs`, `date_picker.rs`, `time_picker.rs`, `number_input.rs`, `spinner.rs`, `context_menu.rs`

### Visual Check
- [x] All widget cards render without panics
- [x] Responsive column count changes on window resize (1 / 2 / 3 / 4 cols)
- [x] Each widget is interactive (sliders slide, inputs accept text, pickers open, etc.)
- [x] Color picker, date picker, time picker popups open and close correctly
- [x] Works in both light and dark theme

---

## Milestone 3: Layouts + PaneGrid
**Status:** [ ] Not started

Implement Tab 2 (Layout Demos) and Tab 3 (PaneGrid Playground).

### Deliverables
- `tabs/layouts/mod.rs` — layouts state and top-level view
- `tabs/layouts/responsive.rs` — live column-count demo
- `tabs/layouts/wrapping.rs` — iced_aw `Wrap` with add/remove buttons
- `tabs/layouts/scrollable.rs` — vertical + horizontal scroll demos
- `tabs/layouts/nesting.rs` — nested Row/Column/Container composition
- `tabs/pane_grid/mod.rs` — full PaneGrid with split, close, resize, drag, content types

### Visual Check
- [ ] Layout demos respond to spacing/padding sliders in real time
- [ ] Wrap demo adds and removes items correctly
- [ ] PaneGrid opens with 2–3 panes
- [ ] Split horizontal / vertical works on each pane
- [ ] Close button disabled on last remaining pane
- [ ] Drag and resize work correctly
- [ ] Works in both light and dark theme

---

## Milestone 4: Canvas & Procedural Art
**Status:** [ ] Not started

Implement Tab 4 — generative art canvas, animated charts, and the shared `Tick` subscription.

### Deliverables
- `tabs/canvas/mod.rs` — state, message, update, view, subscription logic
- `tabs/canvas/procedural_art.rs` — spirograph / Lissajous / particle field `canvas::Program`
- `tabs/canvas/data_viz.rs` — animated bar chart + scrolling line chart `canvas::Program`
- Subscription: `time::every(16ms)` active only on this tab

### Visual Check
- [ ] Procedural art animates continuously at ~60fps
- [ ] Speed, complexity, palette, and pattern controls update the canvas in real time
- [ ] Bar chart and line chart animate smoothly
- [ ] Randomize button changes chart data with animated transition
- [ ] Switching away from this tab stops the subscription (CPU drops to idle)
- [ ] Works in both light and dark theme

---

## Milestone 5: Audio Mixer
**Status:** [ ] Not started

Implement Tab 5 — the full connected audio mixer dashboard with per-channel controls and real-time waveform rendering.

### Deliverables
- `tabs/mixer/mod.rs` — mixer state, message, update, view, subscription logic
- `tabs/mixer/channel.rs` — per-channel state (volume, EQ, wave type, frequency, amplitude, color, enabled)
- `tabs/mixer/waveform.rs` — waveform `canvas::Program` (sine, square, sawtooth, triangle)
- `tabs/mixer/controls.rs` — reusable control components
- Master section with combined waveform canvas
- Shares `Tick` subscription with Canvas tab logic

### Visual Check
- [ ] 4 channel strips render with all controls
- [ ] Volume, EQ, frequency, and amplitude sliders update the waveform instantly
- [ ] Wave type pick list switches waveform shape in real time
- [ ] Color picker sets the waveform stroke color per channel
- [ ] Enable toggler mutes/unmutes a channel (removed from master mix)
- [ ] Master waveform combines all enabled channels
- [ ] Layout adapts: horizontal strips on wide screens, vertical stack on narrow
- [ ] Switching away stops the subscription (CPU idles)
- [ ] Works in both light and dark theme
