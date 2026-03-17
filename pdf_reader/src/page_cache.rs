use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct CacheKey {
    page_index: usize,
    render_width: u32,
}

pub struct PageCache {
    textures: HashMap<CacheKey, TextureHandle>,
}

impl PageCache {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn get(&self, page_index: usize, render_width: u32) -> Option<&TextureHandle> {
        self.textures.get(&CacheKey {
            page_index,
            render_width,
        })
    }

    pub fn insert(
        &mut self,
        page_index: usize,
        render_width: u32,
        ctx: &egui::Context,
        width_px: u32,
        height_px: u32,
        rgba_bytes: Vec<u8>,
    ) -> &TextureHandle {
        let pixels: Vec<Color32> = rgba_bytes
            .chunks_exact(4)
            .map(|c| Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
            .collect();

        let size = [width_px as usize, height_px as usize];
        let image = ColorImage::new(size, pixels);

        let texture = ctx.load_texture(
            format!("page-{}-{}", page_index, render_width),
            image,
            TextureOptions::LINEAR,
        );

        let key = CacheKey {
            page_index,
            render_width,
        };
        self.textures.insert(key, texture);
        self.textures.get(&key).unwrap()
    }

    pub fn evict_distant(&mut self, current_page: usize, keep_range: usize) {
        self.textures
            .retain(|key, _| key.page_index.abs_diff(current_page) <= keep_range);
    }

    pub fn clear(&mut self) {
        self.textures.clear();
    }
}
