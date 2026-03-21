use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Fill, Length};

pub fn view() -> Element<'static, super::Message> {
    // Vertical scrollable
    let mut v_items = column![].spacing(4).padding(8);
    for i in 1..=30 {
        v_items = v_items.push(
            container(text(format!("Row {}", i)).size(13))
                .padding([4, 8])
                .width(Fill)
                .style(container::bordered_box),
        );
    }

    let vertical = column![
        text("Vertical Scroll").size(14),
        container(
            scrollable(v_items).height(200),
        )
        .style(container::bordered_box)
        .width(Fill),
    ]
    .spacing(6);

    // Horizontal scrollable
    let mut h_items = row![].spacing(4).padding(8);
    for i in 1..=20 {
        h_items = h_items.push(
            container(text(format!("Col {}", i)).size(13))
                .padding([8, 16])
                .style(container::bordered_box),
        );
    }

    let horizontal = column![
        text("Horizontal Scroll").size(14),
        container(
            scrollable(h_items)
                .direction(scrollable::Direction::Horizontal(
                    scrollable::Scrollbar::default(),
                ))
                .width(Fill),
        )
        .height(Length::Shrink)
        .style(container::bordered_box)
        .width(Fill),
    ]
    .spacing(6);

    super::demo_card(
        "Scrollable",
        column![vertical, horizontal].spacing(16),
    )
}
