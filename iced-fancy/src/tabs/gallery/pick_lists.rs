use iced::widget::{column, pick_list, text};
use iced::Element;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let options = vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
    ];

    let content = column![
        pick_list(options, state.pick_selected.clone(), super::Message::PickSelected),
        text(format!(
            "Selected: {}",
            state.pick_selected.as_deref().unwrap_or("None")
        ))
        .size(14),
    ]
    .spacing(8);

    super::demo_card("Pick List", content)
}
