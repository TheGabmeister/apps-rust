use iced::widget::{column, text_input};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        text_input("Type something...", &state.text_value)
            .on_input(super::Message::TextChanged),
        text_input("Password...", &state.password_value)
            .on_input(super::Message::PasswordChanged)
            .secure(true),
    ]
    .spacing(8);

    super::demo_card("Text Inputs", content)
}
