use std::path::PathBuf;

use iced::widget::{column, container, rule};
use iced::{Element, Length, Task};

use crate::image_loader;
use crate::views;

#[derive(Debug, Clone)]
pub enum Message {
    OpenFolder,
    FolderSelected(Option<PathBuf>),
    ThumbnailLoaded((usize, Option<iced::widget::image::Handle>)),
    FullImageLoaded(Option<iced::widget::image::Handle>),
    SelectImage(usize),
    BackToGrid,
    NextImage,
    PrevImage,
    KeyboardEvent(iced::keyboard::Event),
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Grid,
    Detail,
}

pub struct PhotoGallery {
    view: View,
    images: Vec<PathBuf>,
    thumbnails: Vec<Option<iced::widget::image::Handle>>,
    full_image: Option<iced::widget::image::Handle>,
    selected: usize,
    current_folder: Option<PathBuf>,
}

impl PhotoGallery {
    pub fn boot() -> (Self, Task<Message>) {
        (
            Self {
                view: View::Grid,
                images: Vec::new(),
                thumbnails: Vec::new(),
                full_image: None,
                selected: 0,
                current_folder: None,
            },
            Task::none(),
        )
    }

    pub fn title(&self) -> String {
        match &self.current_folder {
            Some(folder) => format!(
                "Photo Gallery - {}",
                folder.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown")
            ),
            None => "Photo Gallery".to_string(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenFolder => {
                return Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select Image Folder")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_path_buf())
                    },
                    Message::FolderSelected,
                );
            }

            Message::FolderSelected(Some(folder)) => {
                let images = image_loader::scan_folder(&folder);
                self.current_folder = Some(folder);
                self.images = images;
                self.thumbnails = vec![None; self.images.len()];
                self.full_image = None;
                self.selected = 0;
                self.view = View::Grid;

                return self.load_all_thumbnails();
            }

            Message::FolderSelected(None) => {}

            Message::ThumbnailLoaded((index, handle)) => {
                if index < self.thumbnails.len() {
                    self.thumbnails[index] = handle;
                }
            }

            Message::FullImageLoaded(handle) => {
                self.full_image = handle;
            }

            Message::SelectImage(index) => {
                if index < self.images.len() {
                    self.selected = index;
                    self.view = View::Detail;
                    self.full_image = None;
                    return self.load_selected_image();
                }
            }

            Message::BackToGrid => {
                self.view = View::Grid;
                self.full_image = None;
            }

            Message::NextImage => {
                if !self.images.is_empty() {
                    self.selected = (self.selected + 1) % self.images.len();
                    self.full_image = None;
                    return self.load_selected_image();
                }
            }

            Message::PrevImage => {
                if !self.images.is_empty() {
                    self.selected = if self.selected == 0 {
                        self.images.len() - 1
                    } else {
                        self.selected - 1
                    };
                    self.full_image = None;
                    return self.load_selected_image();
                }
            }

            Message::KeyboardEvent(event) => {
                if self.view == View::Detail {
                    use iced::keyboard::{key::Named, Event, Key};
                    if let Event::KeyPressed { key, .. } = event {
                        match key {
                            Key::Named(Named::ArrowRight) => {
                                return self.update(Message::NextImage);
                            }
                            Key::Named(Named::ArrowLeft) => {
                                return self.update(Message::PrevImage);
                            }
                            Key::Named(Named::Escape) => {
                                return self.update(Message::BackToGrid);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = views::toolbar::view(
            &self.view,
            self.images.len(),
            self.selected,
            &self.current_folder,
        );

        let content: Element<Message> = match self.view {
            View::Grid => views::grid::view(&self.images, &self.thumbnails),
            View::Detail => {
                if let Some(path) = self.images.get(self.selected) {
                    views::detail::view(path, &self.full_image)
                } else {
                    container("No image selected")
                        .center(Length::Fill)
                        .into()
                }
            }
        };

        column![toolbar, rule::horizontal(1), content]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::listen().map(Message::KeyboardEvent)
    }

    fn load_all_thumbnails(&self) -> Task<Message> {
        let tasks: Vec<Task<Message>> = self
            .images
            .iter()
            .enumerate()
            .map(|(i, path)| {
                let path = path.clone();
                Task::perform(
                    async move { (i, image_loader::load_thumbnail(&path)) },
                    Message::ThumbnailLoaded,
                )
            })
            .collect();

        Task::batch(tasks)
    }

    fn load_selected_image(&self) -> Task<Message> {
        let path = self.images[self.selected].clone();
        Task::perform(
            async move { image_loader::load_full_image(&path) },
            Message::FullImageLoaded,
        )
    }
}
