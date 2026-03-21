use iced::widget::{button, column, text};
use iced::{Element, Fill};
use iced_aw::ContextMenu;

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    let underlay = button(text("Right-click me")).on_press(super::Message::ButtonClicked);

    let content = column![
        ContextMenu::new(underlay, || {
            column![
                button(text("Cut"))
                    .width(Fill)
                    .on_press(super::Message::ContextAction("Cut".into())),
                button(text("Copy"))
                    .width(Fill)
                    .on_press(super::Message::ContextAction("Copy".into())),
                button(text("Paste"))
                    .width(Fill)
                    .on_press(super::Message::ContextAction("Paste".into())),
            ]
            .spacing(2)
            .into()
        }),
        text(format!("Last action: {}", state.context_action)).size(14),
    ]
    .spacing(8);

    super::demo_card("Context Menu", content)
}
