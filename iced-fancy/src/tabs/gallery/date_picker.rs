use iced::widget::{button, column, text};
use iced::Element;
use iced_aw::DatePicker;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let btn = button(text("Choose Date")).on_press(super::Message::ToggleDatePicker);

    let content = column![
        DatePicker::new(
            state.show_date_picker,
            state.date.clone(),
            btn,
            super::Message::DateCancel,
            super::Message::DateSubmit,
        ),
        text(format!("Date: {}", state.date_label)).size(14),
    ]
    .spacing(8);

    super::demo_card("Date Picker", content)
}
