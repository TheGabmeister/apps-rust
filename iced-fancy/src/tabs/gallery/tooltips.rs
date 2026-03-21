use iced::widget::tooltip::Position;
use iced::widget::{button, row, text, Tooltip};
use iced::Element;

pub fn view(_state: &super::State) -> Element<'_, super::Message> {
    let content = row![
        Tooltip::new(
            button(text("Hover me")).on_press(super::Message::ButtonClicked),
            text("Top tooltip!"),
            Position::Top,
        ),
        Tooltip::new(
            button(text("Or me")).on_press(super::Message::ButtonClicked),
            text("Bottom tooltip!"),
            Position::Bottom,
        ),
    ]
    .spacing(8);

    super::demo_card("Tooltips", content)
}
