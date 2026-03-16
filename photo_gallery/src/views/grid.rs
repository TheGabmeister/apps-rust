use iced::widget::{button, column, container, image, row, scrollable, text};
use iced::{Element, Length};

use crate::app::Message;

const THUMB_SIZE: f32 = 180.0;
const COLUMNS: usize = 5;

pub fn view<'a>(
    images: &[std::path::PathBuf],
    thumbnails: &[Option<iced::widget::image::Handle>],
) -> Element<'a, Message> {
    if images.is_empty() {
        return container(
            text("Open a folder to browse images").size(20),
        )
        .center(Length::Fill)
        .into();
    }

    let mut grid_column = column![].spacing(8).padding(10);
    let mut current_row = row![].spacing(8);
    let mut col_count = 0;

    for (i, thumb) in thumbnails.iter().enumerate() {
        let cell: Element<'a, Message> = match thumb {
            Some(handle) => {
                button(
                    image(handle.clone())
                        .width(THUMB_SIZE)
                        .height(THUMB_SIZE)
                        .content_fit(iced::ContentFit::Cover),
                )
                .on_press(Message::SelectImage(i))
                .padding(2)
                .into()
            }
            None => {
                button(
                    container(text("Loading...").size(12))
                        .width(THUMB_SIZE)
                        .height(THUMB_SIZE)
                        .center(Length::Fill),
                )
                .on_press(Message::SelectImage(i))
                .padding(2)
                .into()
            }
        };

        current_row = current_row.push(cell);
        col_count += 1;

        if col_count >= COLUMNS {
            grid_column = grid_column.push(current_row);
            current_row = row![].spacing(8);
            col_count = 0;
        }
    }

    if col_count > 0 {
        grid_column = grid_column.push(current_row);
    }

    scrollable(grid_column)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
