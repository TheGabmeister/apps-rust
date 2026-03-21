use iced::widget::column;
use iced::{Element, Length};
use iced_aw::Spinner;

pub fn view(_state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        Spinner::new()
            .width(Length::Fixed(40.0))
            .height(Length::Fixed(40.0))
            .circle_radius(3.0),
    ]
    .spacing(8);

    super::demo_card("Spinner", content)
}
