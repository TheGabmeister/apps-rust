use iced::widget::{button, column, text};
use iced::Element;
use iced_aw::TimePicker;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let btn = button(text("Choose Time")).on_press(super::Message::ToggleTimePicker);

    let content = column![
        TimePicker::new(
            state.show_time_picker,
            state.time.clone(),
            btn,
            super::Message::TimeCancel,
            super::Message::TimeSubmit,
        )
        .use_24h(),
        text(format!("Time: {}", state.time_label)).size(14),
    ]
    .spacing(8);

    super::demo_card("Time Picker", content)
}
