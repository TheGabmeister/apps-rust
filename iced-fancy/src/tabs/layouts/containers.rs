use iced::widget::{column, container, row, slider, text};
use iced::{Alignment, Element, Fill};

pub fn view(max_width: f32) -> Element<'static, super::Message> {
    let control = row![
        text("Max Width").size(14),
        slider(100.0..=800.0, max_width, super::Message::ContainerMaxWidthChanged),
        text(format!("{:.0}px", max_width)).size(13),
    ]
    .spacing(8);

    let centered = container(
        container(
            text("Centered content with max-width constraint").size(13),
        )
        .padding(16)
        .max_width(max_width)
        .center_x(Fill)
        .style(container::bordered_box),
    )
    .width(Fill)
    .center_x(Fill);

    let styled_boxes = row![
        container(
            column![
                text("Default").size(13),
                text("No background").size(11),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Fill)
        .style(container::bordered_box),
        container(
            column![
                text("Primary").size(13),
                text("Styled background").size(11),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Fill)
        .style(container::primary),
        container(
            column![
                text("Secondary").size(13),
                text("Another style").size(11),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Fill)
        .style(container::secondary),
        container(
            column![
                text("Dark").size(13),
                text("Dark variant").size(11),
            ]
            .spacing(4),
        )
        .padding(12)
        .width(Fill)
        .style(container::dark),
    ]
    .spacing(8);

    let alignment_demo = row![
        container(text("Start").size(13))
            .padding(12)
            .width(Fill)
            .height(80)
            .align_y(Alignment::Start)
            .style(container::bordered_box),
        container(text("Center").size(13))
            .padding(12)
            .width(Fill)
            .height(80)
            .center_y(80)
            .style(container::bordered_box),
        container(text("End").size(13))
            .padding(12)
            .width(Fill)
            .height(80)
            .align_y(Alignment::End)
            .style(container::bordered_box),
    ]
    .spacing(8);

    super::demo_card(
        "Container",
        column![
            control,
            text("Centering with max-width:").size(13),
            centered,
            text("Styled backgrounds:").size(13),
            styled_boxes,
            text("Vertical alignment:").size(13),
            alignment_demo,
        ]
        .spacing(10),
    )
}
