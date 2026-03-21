# CLAUDE.md

## Build & Run

```sh
cargo build
cargo run
```

No tests. Single binary, no external assets.

## iced 0.14 API Notes

These patterns differ from older iced versions. Use these, not guesses from memory:

- `iced::application(boot_fn, update_fn, view_fn)` — first arg is a boot function `Fn() -> (State, Task<Message>)`, NOT a title string. Title is set via `.title()` on the builder.
- `iced::widget::rule::horizontal(height)` — free function in the `rule` module. Not `horizontal_rule(...)` or `Rule::horizontal(...)`.
- `Space::new().height(Fill)` — no `Space::with_height()` shorthand.
- `fn view(&self) -> Element<'_, Message>` — use explicit `'_` lifetime to avoid `mismatched_lifetime_syntaxes` warning.
- `button::primary`, `button::secondary` — style functions passed to `.style()`.
- `container::bordered_box` — style function for bordered containers.
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
Note: We built a custom sidebar with buttons instead. If switching to iced_aw's Sidebar, the TabId type must implement `Eq + Clone`.
