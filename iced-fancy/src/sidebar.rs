use iced::widget::{button, column, container, row, rule, text, toggler, Space};
use iced::{Element, Fill, Font, Theme};

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

    pub fn icon(self) -> &'static str {
        match self {
            TabId::Gallery => "\u{25A6}",   // ▦
            TabId::Layouts => "\u{2637}",   // ☷
            TabId::PaneGrid => "\u{2590}",  // ▐
            TabId::Canvas => "\u{25D0}",    // ◐
            TabId::Mixer => "\u{266B}",     // ♫
        }
    }
}

pub fn view(active: TabId, is_dark: bool) -> Element<'static, Message, Theme> {
    let header = text("iced-fancy")
        .size(20)
        .font(Font::MONOSPACE);

    let mut tabs = column![].spacing(4);
    for &tab in TabId::ALL {
        let icon = text(tab.icon()).size(16);
        let label = text(tab.label()).size(14);
        let content = row![icon, label].spacing(8).align_y(iced::Alignment::Center);

        let btn = if tab == active {
            button(content)
                .width(Fill)
                .style(button::primary)
                .on_press(Message::TabSelected(tab))
        } else {
            button(content)
                .width(Fill)
                .style(button::secondary)
                .on_press(Message::TabSelected(tab))
        };
        tabs = tabs.push(btn);
    }

    let theme_toggle = toggler(is_dark)
        .label("Dark mode")
        .on_toggle(Message::ToggleTheme)
        .size(20);

    let sidebar = column![
        header,
        rule::horizontal(1),
        tabs,
        Space::new().height(Fill),
        rule::horizontal(1),
        theme_toggle,
    ]
    .spacing(12)
    .padding(16)
    .width(200);

    container(sidebar)
        .height(Fill)
        .style(container::bordered_box)
        .into()
}
