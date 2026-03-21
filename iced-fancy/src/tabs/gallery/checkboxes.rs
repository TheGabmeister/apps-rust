use iced::widget::{checkbox, column};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let content = column![
        checkbox(state.check1).label("Option A").on_toggle(super::Message::Check1Toggled),
        checkbox(state.check2).label("Option B").on_toggle(super::Message::Check2Toggled),
        checkbox(state.check3).label("Option C").on_toggle(super::Message::Check3Toggled),
    ]
    .spacing(8);

    super::demo_card("Checkboxes", content)
}
