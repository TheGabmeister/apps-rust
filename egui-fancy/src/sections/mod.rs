pub mod buttons;
pub mod dashboard;
pub mod data_viz;
pub mod panels;
pub mod sliders;
pub mod transitions;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    Buttons,
    Sliders,
    DataViz,
    Dashboard,
    Panels,
    Transitions,
}

impl Section {
    pub const ALL: &[Section] = &[
        Section::Buttons,
        Section::Sliders,
        Section::DataViz,
        Section::Dashboard,
        Section::Panels,
        Section::Transitions,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Section::Buttons => "Buttons & Interactions",
            Section::Sliders => "Sliders & Inputs",
            Section::DataViz => "Data Visualization",
            Section::Dashboard => "Dashboard Grid",
            Section::Panels => "Panels & Navigation",
            Section::Transitions => "Animated Transitions",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Section::Buttons => "\u{1f5b1}",    // mouse
            Section::Sliders => "\u{1f39a}",     // sliders
            Section::DataViz => "\u{1f4ca}",     // chart
            Section::Dashboard => "\u{1f4cb}",   // clipboard
            Section::Panels => "\u{1f5c2}",      // folders
            Section::Transitions => "\u{2728}",  // sparkles
        }
    }
}
