use iced::widget::column;
use iced::Element;
use iced_aw::NumberInput;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![NumberInput::new(
        &state.number_value,
        0.0..=1000.0,
        super::Message::NumberChanged,
    )
    .step(0.5),]
    .spacing(8);

    super::demo_card("Number Input", content)
}
