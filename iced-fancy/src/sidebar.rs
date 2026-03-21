use iced::widget::{column, container, rule, toggler, Space};
use iced::{Element, Fill, Font, Length, Theme};
use iced_aw::sidebar::{Sidebar, TabLabel};

use crate::app::Message;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabId {
    Gallery,
    Layouts,
    PaneGrid,
    Canvas,
    Mixer,
}

impl TabId {
    pub const ALL: &'static [TabId] = &[
        TabId::Gallery,
        TabId::Layouts,
        TabId::PaneGrid,
        TabId::Canvas,
        TabId::Mixer,
    ];

    pub fn label(self) -> &'static str {
        match self {
            TabId::Gallery => "Widget Gallery",
            TabId::Layouts => "Layouts",
            TabId::PaneGrid => "PaneGrid",
            TabId::Canvas => "Canvas & Art",
            TabId::Mixer => "Audio Mixer",
        }
    }

    pub fn icon(self) -> char {
        match self {
            TabId::Gallery => '\u{25A6}',   // ▦
            TabId::Layouts => '\u{2637}',   // ☷
            TabId::PaneGrid => '\u{2590}',  // ▐
            TabId::Canvas => '\u{25D0}',    // ◐
            TabId::Mixer => '\u{266B}',     // ♫
        }
    }
}

pub fn view(active: TabId, is_dark: bool) -> Element<'static, Message, Theme> {
    let mut sidebar = Sidebar::new(Message::TabSelected)
        .icon_font(Font::DEFAULT)
        .text_size(14.0)
        .icon_size(16.0)
        .width(Length::Fill)
        .spacing(4.0);

    for &tab in TabId::ALL {
        sidebar = sidebar.push(tab, TabLabel::IconText(tab.icon(), tab.label().to_string()));
    }
    sidebar = sidebar.set_active_tab(&active);

    let theme_toggle = toggler(is_dark)
        .label("Dark mode")
        .on_toggle(Message::ToggleTheme)
        .size(20);

    let sidebar_col = column![
        sidebar,
        Space::new().height(Fill),
        rule::horizontal(1),
        theme_toggle,
    ]
    .spacing(12)
    .padding(16)
    .width(200);

    container(sidebar_col)
        .height(Fill)
        .style(container::bordered_box)
        .into()
}
