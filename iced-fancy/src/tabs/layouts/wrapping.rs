use iced::widget::{button, column, container, row, text};
use iced::{Element, Fill};
use iced_aw::Wrap;

pub fn view(items: &[String], wrap_spacing: f32) -> Element<'_, super::Message> {
    let controls = row![
        button(text("+ Add").size(13)).on_press(super::Message::WrapAdd),
        button(text("− Remove").size(13)).on_press_maybe(
            if items.is_empty() {
                None
            } else {
                Some(super::Message::WrapRemove)
            }
        ),
        text(format!("{} items", items.len())).size(13),
    ]
    .spacing(8);

    let spacing_control = row![
        text("Spacing").size(14),
        iced::widget::slider(0.0..=20.0, wrap_spacing, super::Message::WrapSpacingChanged),
        text(format!("{:.0}px", wrap_spacing)).size(13),
    ]
    .spacing(8);

    // Horizontal wrap
    let mut h_wrap = Wrap::new()
        .spacing(wrap_spacing)
        .line_spacing(wrap_spacing)
        .padding(4.0);

    for (i, label) in items.iter().enumerate() {
        let chip = container(
            text(format!("{} {}", label, i + 1)).size(13),
        )
        .padding([4, 12])
        .style(container::bordered_box);
        h_wrap = h_wrap.push(chip);
    }

    // Vertical wrap
    let mut v_wrap = Wrap::new_vertical()
        .spacing(wrap_spacing)
        .line_spacing(wrap_spacing)
        .padding(4.0);

    for (i, label) in items.iter().enumerate() {
        let chip = container(
            text(format!("{} {}", label, i + 1)).size(13),
        )
        .padding([4, 12])
        .style(container::bordered_box);
        v_wrap = v_wrap.push(chip);
    }

    let h_section = column![
        text("Horizontal").size(13),
        container(h_wrap).width(Fill).padding(4),
    ]
    .spacing(4);

    let v_section = column![
        text("Vertical").size(13),
        container(v_wrap).width(Fill).height(200).padding(4),
    ]
    .spacing(4);

    super::demo_card(
        "Wrap (Flow Layout)",
        column![controls, spacing_control, h_section, v_section].spacing(10),
    )
}
