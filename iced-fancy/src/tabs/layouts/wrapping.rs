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

    let mut wrap = Wrap::new()
        .spacing(wrap_spacing)
        .line_spacing(wrap_spacing)
        .padding(4.0);

    for (i, label) in items.iter().enumerate() {
        let chip = container(
            text(format!("{} {}", label, i + 1)).size(13),
        )
        .padding([4, 12])
        .style(container::bordered_box);
        wrap = wrap.push(chip);
    }

    let wrap_container = container(wrap).width(Fill).padding(8);

    super::demo_card(
        "Wrap (Flow Layout)",
        column![controls, spacing_control, wrap_container].spacing(10),
    )
}
