use iced::widget::{column, slider, text};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        text(format!("Value: {:.1}", state.slider_value)).size(14),
        slider(0.0..=100.0, state.slider_value, super::Message::SliderChanged),
    ]
    .spacing(8);

    super::demo_card("Sliders", content)
}
