use iced::widget::{button, column, row, text};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        row![
            button(text("Primary"))
                .style(button::primary)
                .on_press(super::Message::ButtonClicked),
            button(text("Secondary"))
                .style(button::secondary)
                .on_press(super::Message::ButtonClicked),
            button(text("Disabled")),
        ]
        .spacing(8),
        text(format!("Clicked {} times", state.click_count)).size(14),
    ]
    .spacing(8);

    super::demo_card("Buttons", content)
}
