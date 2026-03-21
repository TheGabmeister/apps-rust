use iced::widget::{button, text};
use iced::{Element, Fill};

pub fn view(state: &super::State) -> Element<'_, super::Message> {
    if state.card_closed {
        let content = button(text("Reopen Card")).on_press(super::Message::CardReopen);
        super::demo_card("Card (closed)", content)
    } else {
        iced_aw::Card::new(
            text("iced_aw Card").size(16),
            text("This card has a close button and a footer. Try clicking the X!"),
        )
        .foot(text("Footer content").size(12))
        .on_close(super::Message::CardClosed)
        .width(Fill)
        .into()
    }
}
