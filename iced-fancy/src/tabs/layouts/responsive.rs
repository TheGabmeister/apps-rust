use iced::widget::{column, container, responsive, row, slider, text, Space};
use iced::{Element, Fill};

pub fn view(spacing: f32, padding: f32) -> Element<'static, super::Message> {
    let controls = column![
        text("Spacing").size(14),
        row![
            slider(0.0..=40.0, spacing, super::Message::SpacingChanged),
            text(format!("{:.0}px", spacing)).size(13),
        ]
        .spacing(8),
        text("Padding").size(14),
        row![
            slider(0.0..=40.0, padding, super::Message::PaddingChanged),
            text(format!("{:.0}px", padding)).size(13),
        ]
        .spacing(8),
    ]
    .spacing(6);

    let demo = responsive(move |size| {
        let cols = if size.width < 300.0 {
            1
        } else if size.width < 500.0 {
            2
        } else if size.width < 700.0 {
            3
        } else {
            4
        };

        let label = text(format!(
            "Width: {:.0}px → {} column{}",
            size.width,
            cols,
            if cols == 1 { "" } else { "s" }
        ))
        .size(13);

        let items: Vec<Element<super::Message>> = (1..=8)
            .map(|i| {
                container(text(format!("Item {}", i)).size(13))
                    .padding(12)
                    .style(container::bordered_box)
                    .width(Fill)
                    .into()
            })
            .collect();

        let mut grid = column![label].spacing(spacing);
        let mut items_iter = items.into_iter();
        loop {
            let mut current_row = row![].spacing(spacing);
            let mut count = 0;
            for _ in 0..cols {
                if let Some(item) = items_iter.next() {
                    current_row = current_row.push(item);
                    count += 1;
                }
            }
            if count == 0 {
                break;
            }
            for _ in count..cols {
                current_row = current_row.push(Space::new().width(Fill));
            }
            grid = grid.push(current_row);
        }

        container(grid).padding(padding).width(Fill).into()
    });

    super::demo_card("Responsive Columns", column![controls, demo].spacing(12))
}
