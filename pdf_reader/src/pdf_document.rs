use anyhow::{Context, Result};
use pdfium_render::prelude::*;
use std::path::{Path, PathBuf};

pub struct PdfDocumentWrapper {
    // Pdfium is leaked to 'static since it lives for the app lifetime.
    // PdfDocument borrows from it, so both share the 'static lifetime.
    #[allow(dead_code)]
    pdfium: &'static Pdfium,
    document: PdfDocument<'static>,
    file_path: PathBuf,
}

fn get_or_create_pdfium() -> &'static Pdfium {
    // Leak a Pdfium instance to get 'static lifetime.
    // This is intentional - Pdfium lives for the entire app.
    // We use a thread_local to avoid creating multiple instances,
    // but the leaked reference is 'static and valid forever.
    thread_local! {
        static PDFIUM: &'static Pdfium = Box::leak(Box::new(Pdfium::new(
            Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
                .or_else(|_| Pdfium::bind_to_system_library())
                .expect("Failed to bind to Pdfium library. Place pdfium.dll next to the executable."),
        )));
    }
    PDFIUM.with(|p| *p)
}

impl PdfDocumentWrapper {
    pub fn open(path: &Path) -> Result<Self> {
        let pdfium = get_or_create_pdfium();
        let document = pdfium
            .load_pdf_from_file(path, None)
            .with_context(|| format!("Failed to load PDF: {}", path.display()))?;

        Ok(Self {
            pdfium,
            document,
            file_path: path.to_path_buf(),
        })
    }

    pub fn page_count(&self) -> usize {
        self.document.pages().len() as usize
    }

    #[allow(dead_code)]
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    /// Render a page to RGBA bytes at the given target width.
    pub fn render_page(
        &self,
        page_index: usize,
        target_width: u32,
    ) -> Result<(u32, u32, Vec<u8>)> {
        let page = self
            .document
            .pages()
            .get(page_index as u16)
            .with_context(|| format!("Page {} not found", page_index))?;

        let config = PdfRenderConfig::new()
            .set_target_width(target_width as i32)
            .set_maximum_height(target_width as i32 * 4);

        let bitmap = page
            .render_with_config(&config)
            .with_context(|| format!("Failed to render page {}", page_index))?;

        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;

        let image = bitmap.as_image();
        let rgba = image.to_rgba8().into_raw();

        Ok((width, height, rgba))
    }

    /// Get page aspect ratio (width / height) without rendering.
    pub fn page_aspect_ratio(&self, page_index: usize) -> Result<f32> {
        let page = self
            .document
            .pages()
            .get(page_index as u16)
            .with_context(|| format!("Page {} not found", page_index))?;

        let w = page.width().value;
        let h = page.height().value;
        Ok(w / h)
    }
}
