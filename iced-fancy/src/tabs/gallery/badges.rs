use iced::widget::{row, text};
use iced::Element;
use iced_aw::Badge;

pub fn view(_state: &super::State) -> Element<'_, super::Message> {
    let content = row![
        Badge::new(text("Info")),
        Badge::new(text("3")),
        Badge::new(text("New")),
        Badge::new(text("Beta")),
    ]
    .spacing(8);

    super::demo_card("Badges", content)
}
