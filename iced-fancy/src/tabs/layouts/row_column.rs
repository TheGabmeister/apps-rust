use iced::widget::{column, container, pick_list, row, slider, text};
use iced::{Alignment, Element, Fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlignmentChoice {
    Start,
    Center,
    End,
}

impl std::fmt::Display for AlignmentChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlignmentChoice::Start => write!(f, "Start"),
            AlignmentChoice::Center => write!(f, "Center"),
            AlignmentChoice::End => write!(f, "End"),
        }
    }
}

impl AlignmentChoice {
    pub const ALL: &'static [AlignmentChoice] = &[
        AlignmentChoice::Start,
        AlignmentChoice::Center,
        AlignmentChoice::End,
    ];

    pub fn to_alignment(self) -> Alignment {
        match self {
            AlignmentChoice::Start => Alignment::Start,
            AlignmentChoice::Center => Alignment::Center,
            AlignmentChoice::End => Alignment::End,
        }
    }
}

pub fn view(
    rc_spacing: f32,
    rc_padding: f32,
    rc_alignment: AlignmentChoice,
) -> Element<'static, super::Message> {
    let controls = column![
        row![
            text("Spacing").size(14),
            slider(0.0..=40.0, rc_spacing, super::Message::RcSpacingChanged),
            text(format!("{:.0}px", rc_spacing)).size(13),
        ]
        .spacing(8),
        row![
            text("Padding").size(14),
            slider(0.0..=40.0, rc_padding, super::Message::RcPaddingChanged),
            text(format!("{:.0}px", rc_padding)).size(13),
        ]
        .spacing(8),
        row![
            text("Align").size(14),
            pick_list(
                AlignmentChoice::ALL,
                Some(rc_alignment),
                super::Message::RcAlignmentChanged,
            )
            .text_size(13),
        ]
        .spacing(8),
    ]
    .spacing(6);

    let align = rc_alignment.to_alignment();

    let row_demo = column![
        text("Row").size(13),
        container(
            row![
                sized_box("A", 60.0, 40.0),
                sized_box("B", 80.0, 60.0),
                sized_box("C", 50.0, 30.0),
            ]
            .spacing(rc_spacing)
            .padding(rc_padding)
            .align_y(align),
        )
        .style(container::bordered_box)
        .width(Fill),
    ]
    .spacing(4);

    let col_demo = column![
        text("Column").size(13),
        container(
            column![
                sized_box("X", 120.0, 28.0),
                sized_box("Y", 80.0, 28.0),
                sized_box("Z", 160.0, 28.0),
            ]
            .spacing(rc_spacing)
            .padding(rc_padding)
            .align_x(align),
        )
        .style(container::bordered_box)
        .width(Fill),
    ]
    .spacing(4);

    super::demo_card(
        "Row & Column",
        column![controls, row_demo, col_demo].spacing(12),
    )
}

fn sized_box<'a>(label: &str, width: f32, height: f32) -> Element<'a, super::Message> {
    container(text(label.to_string()).size(13))
        .width(width)
        .height(height)
        .center_x(width)
        .center_y(height)
        .style(container::bordered_box)
        .into()
}
