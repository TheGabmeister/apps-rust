use iced::widget::{column, text, toggler};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        toggler(state.toggler_value)
            .label("Toggle me")
            .on_toggle(super::Message::TogglerToggled),
        text(if state.toggler_value { "ON" } else { "OFF" }).size(14),
    ]
    .spacing(8);

    super::demo_card("Togglers", content)
}
