mod app;
mod image_loader;
mod views;

use app::PhotoGallery;

fn main() -> iced::Result {
    iced::application(PhotoGallery::boot, PhotoGallery::update, PhotoGallery::view)
        .title(PhotoGallery::title)
        .subscription(PhotoGallery::subscription)
        .window_size((1200.0, 800.0))
        .run()
}
