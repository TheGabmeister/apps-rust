use std::path::{Path, PathBuf};

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"];
const THUMBNAIL_SIZE: u32 = 200;

pub fn scan_folder(folder: &Path) -> Vec<PathBuf> {
    let mut images = Vec::new();

    let entries = match std::fs::read_dir(folder) {
        Ok(entries) => entries,
        Err(_) => return images,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    images.push(path);
                }
            }
        }
    }

    images.sort();
    images
}

pub fn load_thumbnail(path: &Path) -> Option<iced::widget::image::Handle> {
    let img = image::open(path).ok()?;
    let thumb = img.thumbnail(THUMBNAIL_SIZE, THUMBNAIL_SIZE);
    let rgba = thumb.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(iced::widget::image::Handle::from_rgba(w, h, rgba.into_raw()))
}

pub fn load_full_image(path: &Path) -> Option<iced::widget::image::Handle> {
    let img = image::open(path).ok()?;
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    Some(iced::widget::image::Handle::from_rgba(w, h, rgba.into_raw()))
}
