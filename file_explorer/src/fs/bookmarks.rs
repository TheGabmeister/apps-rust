use std::path::PathBuf;

pub struct Bookmark {
    pub label: &'static str,
    pub icon: &'static str,
    pub path: Option<PathBuf>,
}

pub fn default_bookmarks() -> Vec<Bookmark> {
    vec![
        Bookmark {
            label: "Home",
            icon: "\u{1F3E0}",
            path: dirs::home_dir(),
        },
        Bookmark {
            label: "Desktop",
            icon: "\u{1F5A5}",
            path: dirs::desktop_dir(),
        },
        Bookmark {
            label: "Documents",
            icon: "\u{1F4C4}",
            path: dirs::document_dir(),
        },
        Bookmark {
            label: "Downloads",
            icon: "\u{2B07}",
            path: dirs::download_dir(),
        },
    ]
}
