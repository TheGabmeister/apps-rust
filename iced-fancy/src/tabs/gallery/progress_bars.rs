use iced::widget::{column, progress_bar, slider, text};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        text(format!("Progress: {:.0}%", state.progress_value)).size(14),
        progress_bar(0.0..=100.0, state.progress_value),
        slider(
            0.0..=100.0,
            state.progress_value,
            super::Message::ProgressChanged,
        ),
    ]
    .spacing(8);

    super::demo_card("Progress Bar", content)
}
