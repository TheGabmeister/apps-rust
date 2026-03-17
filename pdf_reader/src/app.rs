use crate::page_cache::PageCache;
use crate::pdf_document::PdfDocumentWrapper;
use crate::recent_files::RecentFiles;
use crate::toolbar;
use crate::viewer;
use egui;

#[derive(Clone, Copy, PartialEq)]
pub enum ZoomMode {
    FitWidth,
    FitPage,
    Custom,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    SinglePage,
    ContinuousScroll,
}

pub struct PdfReaderApp {
    pub pdf_doc: Option<PdfDocumentWrapper>,
    pub page_cache: PageCache,
    pub current_page: usize,
    pub total_pages: usize,
    pub zoom_mode: ZoomMode,
    pub zoom_percent: f32,
    pub view_mode: ViewMode,
    pub recent_files: RecentFiles,
    pub error_message: Option<String>,
    pub page_jump_input: String,
    pub pending_open_path: Option<std::path::PathBuf>,
}

impl PdfReaderApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            pdf_doc: None,
            page_cache: PageCache::new(),
            current_page: 0,
            total_pages: 0,
            zoom_mode: ZoomMode::FitWidth,
            zoom_percent: 100.0,
            view_mode: ViewMode::SinglePage,
            recent_files: RecentFiles::load(),
            error_message: None,
            page_jump_input: String::new(),
            pending_open_path: None,
        }
    }

    pub fn open_file_dialog(&mut self, ctx: &egui::Context) {
        let file = rfd::FileDialog::new()
            .set_title("Open PDF")
            .add_filter("PDF files", &["pdf"])
            .add_filter("All files", &["*"])
            .pick_file();

        if let Some(path) = file {
            self.load_pdf(&path, ctx);
        }
    }

    pub fn load_pdf(&mut self, path: &std::path::Path, ctx: &egui::Context) {
        self.page_cache.clear();
        self.error_message = None;

        match PdfDocumentWrapper::open(path) {
            Ok(doc) => {
                self.total_pages = doc.page_count();
                self.current_page = 0;
                self.page_jump_input = "1".to_string();
                self.recent_files.add(path);
                self.pdf_doc = Some(doc);

                if let Some(name) = path.file_name() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title(
                        format!("{} - PDF Reader", name.to_string_lossy()),
                    ));
                }
            }
            Err(e) => {
                self.pdf_doc = None;
                self.total_pages = 0;
                self.error_message = Some(format!("Failed to open PDF: {}", e));
            }
        }
    }

    pub fn go_to_page(&mut self, page: usize) {
        if self.total_pages > 0 {
            self.current_page = page.min(self.total_pages - 1);
            self.page_jump_input = format!("{}", self.current_page + 1);
        }
    }

    pub fn next_page(&mut self) {
        if self.current_page + 1 < self.total_pages {
            self.go_to_page(self.current_page + 1);
        }
    }

    pub fn prev_page(&mut self) {
        if self.current_page > 0 {
            self.go_to_page(self.current_page - 1);
        }
    }

    pub fn first_page(&mut self) {
        self.go_to_page(0);
    }

    pub fn last_page(&mut self) {
        if self.total_pages > 0 {
            self.go_to_page(self.total_pages - 1);
        }
    }

    pub fn zoom_in(&mut self) {
        self.zoom_mode = ZoomMode::Custom;
        self.zoom_percent = (self.zoom_percent + 25.0).min(500.0);
        self.page_cache.clear();
    }

    pub fn zoom_out(&mut self) {
        self.zoom_mode = ZoomMode::Custom;
        self.zoom_percent = (self.zoom_percent - 25.0).max(25.0);
        self.page_cache.clear();
    }

    pub fn set_zoom(&mut self, mode: ZoomMode, percent: Option<f32>) {
        self.zoom_mode = mode;
        if let Some(p) = percent {
            self.zoom_percent = p.clamp(25.0, 500.0);
        }
        self.page_cache.clear();
    }

    pub fn compute_render_width(&self, available_width: f32, available_height: f32) -> u32 {
        let width = match self.zoom_mode {
            ZoomMode::FitWidth => available_width,
            ZoomMode::FitPage => {
                if let Some(ref doc) = self.pdf_doc {
                    if let Ok(aspect) = doc.page_aspect_ratio(self.current_page) {
                        let width_from_height = available_height * aspect;
                        available_width.min(width_from_height)
                    } else {
                        available_width
                    }
                } else {
                    available_width
                }
            }
            ZoomMode::Custom => available_width * self.zoom_percent / 100.0,
        };
        (width as u32).max(100)
    }
}

impl eframe::App for PdfReaderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle pending file open from recent files
        if let Some(path) = self.pending_open_path.take() {
            self.load_pdf(&path, ctx);
        }

        // Keyboard shortcuts
        let ctrl_o = ctx.input(|i: &egui::InputState| {
            i.modifiers.ctrl && i.key_pressed(egui::Key::O)
        });
        let page_up = ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::PageUp));
        let page_down = ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::PageDown));
        let home = ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::Home));
        let end = ctx.input(|i: &egui::InputState| i.key_pressed(egui::Key::End));
        let zoom_in_key = ctx.input(|i: &egui::InputState| {
            i.modifiers.ctrl
                && (i.key_pressed(egui::Key::Plus) || i.key_pressed(egui::Key::Equals))
        });
        let zoom_out_key =
            ctx.input(|i: &egui::InputState| i.modifiers.ctrl && i.key_pressed(egui::Key::Minus));
        let zoom_reset =
            ctx.input(|i: &egui::InputState| i.modifiers.ctrl && i.key_pressed(egui::Key::Num0));

        if ctrl_o {
            self.open_file_dialog(ctx);
        }
        if page_up {
            self.prev_page();
        }
        if page_down {
            self.next_page();
        }
        if home {
            self.first_page();
        }
        if end {
            self.last_page();
        }
        if zoom_in_key {
            self.zoom_in();
        }
        if zoom_out_key {
            self.zoom_out();
        }
        if zoom_reset {
            self.set_zoom(ZoomMode::Custom, Some(100.0));
        }

        toolbar::render_toolbar(self, ctx);
        viewer::render_viewer(self, ctx);
    }
}
