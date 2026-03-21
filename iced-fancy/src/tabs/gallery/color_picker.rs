use iced::widget::{button, column, text};
use iced::Element;
use iced_aw::ColorPicker;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let btn = button(text("Choose Color")).on_press(super::Message::ToggleColorPicker);

    let content = column![
        ColorPicker::new(
            state.show_color_picker,
            state.color,
            btn,
            super::Message::ColorCancel,
            super::Message::ColorSubmit,
        ),
        text(format!(
            "RGB: ({:.2}, {:.2}, {:.2})",
            state.color.r, state.color.g, state.color.b
        ))
        .size(14),
    ]
    .spacing(8);

    super::demo_card("Color Picker", content)
}
