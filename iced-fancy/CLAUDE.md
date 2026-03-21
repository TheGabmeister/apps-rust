# CLAUDE.md

## Build & Run

```sh
cargo build
cargo run
```

No tests. Single binary, no external assets.

## Project Overview

Desktop showcase app pushing iced 0.14 + iced_aw 0.13 to their limits. Five tabs navigated via an iced_aw `Sidebar`, with a global light/dark toggle.

## Architecture

- `main.rs` — entry point, `iced::application` builder, font registration
- `app.rs` — `App` state, top-level `Message` enum, `update`/`view`/`subscription`/`theme`. Each tab's messages are wrapped via `Message::TabName(tab::Message)`.
- `sidebar.rs` — `TabId` enum (Gallery, Layouts, PaneGrid, Canvas, Mixer), sidebar view
- `theme.rs` — theme helpers (minimal)
- `tabs/` — each tab is a submodule with its own `State`, `Message`, `update()`, `view()`

### Tab 1: Widget Gallery (`tabs/gallery/`)
Responsive grid of demo cards for every core iced widget and iced_aw widget. `mod.rs` holds state and the responsive grid layout; each widget demo is a separate file (e.g. `buttons.rs`, `sliders.rs`, `color_picker.rs`).

### Tab 2: Layouts (`tabs/layouts/`)
Interactive layout demos: `responsive.rs`, `wrapping.rs` (iced_aw Wrap), `scrollable.rs`, `nesting.rs`, `row_column.rs`, `containers.rs`. Sliders control spacing/padding live.

### Tab 3: PaneGrid (`tabs/pane_grid/`)
Single `mod.rs`. Split/close/resize/drag panes with selectable content types (Editor, ColorSwatch, Counter, Placeholder).

### Tab 4: Canvas & Art (`tabs/canvas/`)
- `procedural_art.rs` — `canvas::Program` with 3 patterns (Spirograph, Lissajous, Particles) and 4 palettes (Rainbow, Ocean, Fire, Neon)
- `data_viz.rs` — `canvas::Program` with animated bar chart + scrolling line chart
- `mod.rs` — controls (pattern, palette, speed, complexity, randomize)

### Tab 5: Audio Mixer (`tabs/mixer/`)
- `channel.rs` — `Channel` struct (volume, EQ bass/treble, wave type, frequency, amplitude, color, enabled) and `WaveType` enum (Sine, Square, Sawtooth, Triangle) with `sample()` math
- `waveform.rs` — `draw_channel_waveform()` and `draw_master_waveform()` rendering functions
- `controls.rs` — `channel_strip()` builds the per-channel control panel
- `mod.rs` — 4-channel mixer state, responsive layout (horizontal wide / vertical narrow), master output canvas

### Subscriptions
- `time::every(16ms)` active only on Canvas or Mixer tabs (checked in `App::subscription`)
- `Tick(Instant)` in app.rs forwards `dt` to the active tab's update
- Other tabs return `Subscription::none()` — zero CPU overhead

## iced 0.14 API Notes

These patterns differ from older iced versions. Use these, not guesses from memory:

- `iced::application(boot_fn, update_fn, view_fn)` — first arg is a boot function `Fn() -> (State, Task<Message>)`, NOT a title string. Title is set via `.title()` on the builder.
- `iced::widget::rule::horizontal(height)` — free function in the `rule` module. Not `horizontal_rule(...)` or `Rule::horizontal(...)`.
- `Space::new().height(Fill)` — no `Space::with_height()` shorthand.
- `fn view(&self) -> Element<'_, Message>` — use explicit `'_` lifetime to avoid `mismatched_lifetime_syntaxes` warning.
- `button::primary`, `button::secondary` — style functions passed to `.style()`.
- `container::bordered_box` — style function for bordered containers.
- `pane_grid::Content::style(|&Theme| -> container::Style)` — uses container styles, not pane_grid-specific ones. No `pane_grid::default` or `pane_grid::default_focused` exists.
- `checkbox(is_checked).label("...").on_toggle(Msg)` — builder pattern; NOT `checkbox("label", bool)`.
- `toggler(is_toggled).label("...").on_toggle(Msg)` — builder pattern, not positional args.

## iced_aw 0.13 Font

- `iced_aw::ICED_AW_FONT` / `ICED_AW_FONT_BYTES` exists but only has a handful of internal widget glyphs (close, arrows). It is NOT a general-purpose icon font.
- There is no `BOOTSTRAP_FONT` in iced_aw. Bootstrap icons are in the `iced_fonts` crate (`features = ["bootstrap"]`), which is not a dependency.
- For sidebar/tab icons, use Unicode symbols with the default font.
- `pane_grid` and `canvas` are NOT in iced_aw — import from `iced::widget::pane_grid` and `iced::widget::canvas` directly.

## iced_aw 0.13 Widget API Reference

All widgets below are re-exported at the crate root (e.g. `iced_aw::Card`, `iced_aw::Badge`).

### Card
```rust
Card::new(head, body)           // head/body: Into<Element>
    .foot(footer)               // optional footer element
    .on_close(Message)          // enables close icon
    .width(Length) / .height(Length)
    .padding(Padding)
    .style(|&Theme, Status| -> Style)
```

### Badge
```rust
Badge::new(content)             // content: Into<Element>
    .style(|&Theme, Status| -> Style)
    .padding(u16)
```

### ColorPicker
```rust
ColorPicker::new(
    show_picker: bool,          // toggle visibility
    color: Color,               // current color
    underlay,                   // element shown when picker is closed
    on_cancel: Message,         // message when cancelled
    |Color| -> Message,         // on_submit callback
)
```

### DatePicker
```rust
DatePicker::new(
    show_picker: bool,
    date: impl Into<Date>,      // iced_aw::date_picker::Date
    underlay,
    on_cancel: Message,
    |Date| -> Message,          // on_submit
)
```

### TimePicker
```rust
TimePicker::new(
    show_picker: bool,
    time: impl Into<Time>,      // iced_aw::time_picker::Time
    underlay,
    on_cancel: Message,
    |Time| -> Message,          // on_submit
)
    .use_24h()                  // 24-hour format
    .show_seconds()
```

### NumberInput
```rust
NumberInput::new(
    value: &T,                  // T: Num + Display + FromStr + PartialOrd + Bounded
    bounds: impl RangeBounds<T>,
    |T| -> Message,             // on_change
)
    .step(T)                    // increment/decrement step
```

### Spinner
```rust
Spinner::new()
    .width(Length)
    .height(Length)
    .circle_radius(f32)
```

### ContextMenu
```rust
ContextMenu::new(
    underlay,                   // element that triggers context menu
    || -> Element,              // closure returning menu content
)
```

### Wrap
```rust
// Horizontal (default)
Wrap::new()                             // or Wrap::with_elements(vec)
    .push(element)
    .spacing(Pixels)
    .line_spacing(Pixels)
    .padding(Padding)
    .align_items(Alignment)

// Vertical
Wrap::new_vertical()                    // or Wrap::with_elements_vertical(vec)
```

### Sidebar (iced_aw widget)
```rust
Sidebar::new(|TabId| -> Message)        // on_select callback
    .push(tab_id, TabLabel::IconText(char, String))
    .set_active_tab(&active_tab)
    .width(Length) / .height(Length)
    .icon_size(f32) / .text_size(f32)
    .spacing(Pixels)

// TabLabel variants:
TabLabel::Icon(char)
TabLabel::Text(String)
TabLabel::IconText(char, String)
```
Note: Currently using iced_aw's `Sidebar` widget (per spec). If it causes visual issues later, it's a single-file swap back to a custom Column of buttons — the `TabId` enum and app integration are identical either way. `TabId` must implement `Eq + Clone`. Use `.icon_font(Font::DEFAULT)` to render Unicode icons instead of iced_aw's internal font.
