use iced::widget::{container, image, text};
use iced::{Element, Length};

use crate::app::Message;

pub fn view<'a>(
    path: &std::path::PathBuf,
    full_image: &Option<iced::widget::image::Handle>,
) -> Element<'a, Message> {
    match full_image {
        Some(handle) => {
            container(
                image(handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(iced::ContentFit::Contain),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center(Length::Fill)
            .into()
        }
        None => {
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");

            container(text(format!("Loading {}...", filename)).size(20))
                .width(Length::Fill)
                .height(Length::Fill)
                .center(Length::Fill)
                .into()
        }
    }
}
