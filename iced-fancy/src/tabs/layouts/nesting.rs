use iced::widget::{column, container, row, slider, text};
use iced::{Alignment, Element, Fill, Length};

pub fn view(nest_spacing: f32, nest_padding: f32) -> Element<'static, super::Message> {
    let controls = column![
        row![
            text("Spacing").size(14),
            slider(0.0..=30.0, nest_spacing, super::Message::NestSpacingChanged),
            text(format!("{:.0}px", nest_spacing)).size(13),
        ]
        .spacing(8),
        row![
            text("Padding").size(14),
            slider(0.0..=30.0, nest_padding, super::Message::NestPaddingChanged),
            text(format!("{:.0}px", nest_padding)).size(13),
        ]
        .spacing(8),
    ]
    .spacing(6);

    let sp = nest_spacing;
    let pd = nest_padding;

    let inner_col = container(
        column![
            labeled_box("A", pd),
            labeled_box("B", pd),
        ]
        .spacing(sp)
    )
    .padding(pd)
    .style(container::bordered_box)
    .width(Fill);

    let inner_row = container(
        row![
            labeled_box("C", pd),
            labeled_box("D", pd),
        ]
        .spacing(sp)
    )
    .padding(pd)
    .style(container::bordered_box)
    .width(Fill);

    let nested = container(
        column![
            text("Outer Column").size(13),
            row![
                container(
                    column![
                        text("Inner Column").size(12),
                        inner_col,
                    ]
                    .spacing(sp)
                )
                .padding(pd)
                .style(container::bordered_box)
                .width(Fill),
                container(
                    column![
                        text("Inner Row").size(12),
                        inner_row,
                    ]
                    .spacing(sp)
                )
                .padding(pd)
                .style(container::bordered_box)
                .width(Fill),
            ]
            .spacing(sp),
        ]
        .spacing(sp)
    )
    .padding(pd)
    .style(container::bordered_box)
    .width(Fill);

    super::demo_card(
        "Nested Composition",
        column![controls, nested].spacing(12),
    )
}

fn labeled_box<'a>(label: &str, padding: f32) -> Element<'a, super::Message> {
    container(
        text(label.to_string()).size(14).align_x(Alignment::Center),
    )
    .padding(padding)
    .width(Fill)
    .center_y(Length::Shrink)
    .style(container::bordered_box)
    .into()
}
