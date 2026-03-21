mod row_column;
mod containers;
mod responsive;
mod wrapping;
mod scrollable;
mod nesting;

use iced::widget::{column, container, rule, scrollable as scroll_widget, text};
use iced::{Element, Fill};

pub use row_column::AlignmentChoice;

#[derive(Debug)]
pub struct State {
    pub spacing: f32,
    pub padding: f32,
    pub wrap_items: Vec<String>,
    pub wrap_spacing: f32,
    pub nest_spacing: f32,
    pub nest_padding: f32,
    pub rc_spacing: f32,
    pub rc_padding: f32,
    pub rc_alignment: AlignmentChoice,
    pub container_max_width: f32,
}

impl Default for State {
    fn default() -> Self {
        let labels = ["Tag", "Chip", "Badge", "Label", "Item", "Token"];
        Self {
            spacing: 12.0,
            padding: 8.0,
            wrap_items: labels.iter().map(|s| s.to_string()).collect(),
            wrap_spacing: 8.0,
            nest_spacing: 8.0,
            nest_padding: 8.0,
            rc_spacing: 12.0,
            rc_padding: 8.0,
            rc_alignment: AlignmentChoice::Center,
            container_max_width: 400.0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SpacingChanged(f32),
    PaddingChanged(f32),
    WrapAdd,
    WrapRemove,
    WrapSpacingChanged(f32),
    NestSpacingChanged(f32),
    NestPaddingChanged(f32),
    RcSpacingChanged(f32),
    RcPaddingChanged(f32),
    RcAlignmentChanged(AlignmentChoice),
    ContainerMaxWidthChanged(f32),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SpacingChanged(v) => state.spacing = v,
        Message::PaddingChanged(v) => state.padding = v,
        Message::WrapAdd => {
            let names = ["Tag", "Chip", "Badge", "Label", "Item", "Token"];
            let idx = state.wrap_items.len() % names.len();
            state.wrap_items.push(names[idx].to_string());
        }
        Message::WrapRemove => {
            state.wrap_items.pop();
        }
        Message::WrapSpacingChanged(v) => state.wrap_spacing = v,
        Message::NestSpacingChanged(v) => state.nest_spacing = v,
        Message::NestPaddingChanged(v) => state.nest_padding = v,
        Message::RcSpacingChanged(v) => state.rc_spacing = v,
        Message::RcPaddingChanged(v) => state.rc_padding = v,
        Message::RcAlignmentChanged(v) => state.rc_alignment = v,
        Message::ContainerMaxWidthChanged(v) => state.container_max_width = v,
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let content = column![
        text("Layout Demos").size(24),
        rule::horizontal(1),
        row_column::view(state.rc_spacing, state.rc_padding, state.rc_alignment),
        containers::view(state.container_max_width),
        responsive::view(state.spacing, state.padding),
        wrapping::view(&state.wrap_items, state.wrap_spacing),
        scrollable::view(),
        nesting::view(state.nest_spacing, state.nest_padding),
    ]
    .spacing(16)
    .padding(16)
    .width(Fill);

    scroll_widget(content).height(Fill).into()
}

pub fn demo_card<'a>(
    title: &str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    container(
        column![
            text(title.to_string()).size(16),
            rule::horizontal(1),
            content.into(),
        ]
        .spacing(8)
        .padding(12),
    )
    .style(container::bordered_box)
    .width(Fill)
    .into()
}
