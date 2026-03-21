use iced::widget::{button, container, row, rule, text, toggler, Space};
use iced::{Element, Length, Padding, Theme};
use iced_aw::menu::{Item, Menu, MenuBar};

use crate::app::Message;
use crate::sidebar::TabId;

/// Builds a styled menu item button with consistent width.
fn menu_button<'a>(label: &'a str, msg: Message) -> Element<'a, Message, Theme, iced::Renderer> {
    button(text(label).size(13))
        .on_press(msg)
        .style(button::text)
        .padding(Padding::from([4, 12]))
        .width(Length::Fill)
        .into()
}

/// Builds the top-level menu bar label button.
fn bar_button<'a>(label: &'a str) -> Element<'a, Message, Theme, iced::Renderer> {
    button(text(label).size(13))
        .style(button::text)
        .padding(Padding::from([4, 10]))
        .into()
}

pub fn view(active_tab: TabId, is_dark: bool) -> Element<'static, Message, Theme> {
    // -- View menu ----------------------------------------------------------
    let view_items: Vec<Item<'_, Message, Theme, iced::Renderer>> = vec![
        Item::new(
            toggler(is_dark)
                .label("Dark mode")
                .on_toggle(Message::ToggleTheme)
                .size(16)
                .text_size(13),
        ),
        Item::new(rule::horizontal(1)),
        Item::new(menu_button("Gallery", Message::TabSelected(TabId::Gallery))),
        Item::new(menu_button("Layouts", Message::TabSelected(TabId::Layouts))),
        Item::new(menu_button("PaneGrid", Message::TabSelected(TabId::PaneGrid))),
        Item::new(menu_button("Canvas & Art", Message::TabSelected(TabId::Canvas))),
        Item::new(menu_button("Audio Mixer", Message::TabSelected(TabId::Mixer))),
    ];

    let view_menu = Item::with_menu(
        bar_button("View"),
        Menu::new(view_items)
            .max_width(200.0)
            .offset(4.0),
    );

    // -- Help menu ----------------------------------------------------------
    let help_items: Vec<Item<'_, Message, Theme, iced::Renderer>> = vec![
        Item::new(
            container(
                text("iced-fancy v0.1.0\nA showcase for iced 0.14 + iced_aw 0.13")
                    .size(12),
            )
            .padding(Padding::from([6, 12]))
            .width(Length::Fill),
        ),
    ];

    let help_menu = Item::with_menu(
        bar_button("Help"),
        Menu::new(help_items)
            .max_width(260.0)
            .offset(4.0),
    );

    // -- Assemble bar -------------------------------------------------------
    let bar = MenuBar::new(vec![view_menu, help_menu])
        .spacing(2.0)
        .padding(Padding::from([2, 6]));

    let bar_row = row![
        Element::from(bar),
        Space::new().width(Length::Fill),
        text(format!("Tab: {}", active_tab.label())).size(12),
    ]
    .spacing(8)
    .padding(Padding::from([0, 8]))
    .align_y(iced::Center);

    container(bar_row)
        .width(Length::Fill)
        .style(container::bordered_box)
        .into()
}
