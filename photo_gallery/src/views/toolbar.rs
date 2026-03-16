use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};

use crate::app::{Message, View};

pub fn view<'a>(
    current_view: &View,
    image_count: usize,
    selected: usize,
    current_folder: &Option<std::path::PathBuf>,
) -> Element<'a, Message> {
    let open_btn = button(text("Open Folder")).on_press(Message::OpenFolder);

    let folder_label = match current_folder {
        Some(p) => text(p.display().to_string()).size(14),
        None => text("No folder selected").size(14),
    };

    let left_section = row![open_btn, folder_label]
        .spacing(10)
        .align_y(Alignment::Center);

    let right_section: Element<'a, Message> = match current_view {
        View::Grid => {
            text(format!("{} images", image_count))
                .size(14)
                .into()
        }
        View::Detail => {
            let back_btn = button(text("Back")).on_press(Message::BackToGrid);
            let prev_btn = button(text("<")).on_press(Message::PrevImage);
            let next_btn = button(text(">")).on_press(Message::NextImage);
            let label = text(format!("{} / {}", selected + 1, image_count)).size(14);

            row![back_btn, prev_btn, label, next_btn]
                .spacing(8)
                .align_y(Alignment::Center)
                .into()
        }
    };

    container(
        row![left_section, right_section]
            .spacing(20)
            .align_y(Alignment::Center)
            .width(Length::Fill),
    )
    .padding(10)
    .width(Length::Fill)
    .into()
}
