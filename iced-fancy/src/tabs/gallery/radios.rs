use iced::widget::{column, radio};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        radio(
            "Small",
            super::RadioChoice::A,
            state.radio_selected,
            super::Message::RadioSelected,
        ),
        radio(
            "Medium",
            super::RadioChoice::B,
            state.radio_selected,
            super::Message::RadioSelected,
        ),
        radio(
            "Large",
            super::RadioChoice::C,
            state.radio_selected,
            super::Message::RadioSelected,
        ),
    ]
    .spacing(8);

    super::demo_card("Radio Buttons", content)
}
