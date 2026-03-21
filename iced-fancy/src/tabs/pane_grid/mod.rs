use iced::widget::{
    button, column, container, pick_list, row, rule, scrollable, text,
    pane_grid::{self, PaneGrid},
};
use iced::{Element, Fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneContent {
    Text,
    Counter,
    Colors,
}

impl PaneContent {
    const ALL: &'static [PaneContent] = &[PaneContent::Text, PaneContent::Counter, PaneContent::Colors];
}

impl std::fmt::Display for PaneContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneContent::Text => write!(f, "Text"),
            PaneContent::Counter => write!(f, "Counter"),
            PaneContent::Colors => write!(f, "Colors"),
        }
    }
}

#[derive(Debug)]
pub struct PaneState {
    pub content: PaneContent,
    pub counter: i32,
}

impl PaneState {
    fn new(content: PaneContent) -> Self {
        Self { content, counter: 0 }
    }
}

#[derive(Debug)]
pub struct State {
    pub panes: pane_grid::State<PaneState>,
    pub focus: Option<pane_grid::Pane>,
    pub pane_count: usize,
}

impl Default for State {
    fn default() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.5,
            a: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::Text))),
            b: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.5,
                a: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::Counter))),
                b: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::Colors))),
            }),
        };
        let panes = pane_grid::State::with_configuration(config);
        Self {
            panes,
            focus: None,
            pane_count: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SplitH(pane_grid::Pane),
    SplitV(pane_grid::Pane),
    Close(pane_grid::Pane),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    ContentChanged(pane_grid::Pane, PaneContent),
    Increment(pane_grid::Pane),
    Decrement(pane_grid::Pane),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SplitH(pane) => {
            if let Some((new_pane, _)) =
                state.panes.split(pane_grid::Axis::Horizontal, pane, PaneState::new(PaneContent::Text))
            {
                state.focus = Some(new_pane);
                state.pane_count += 1;
            }
        }
        Message::SplitV(pane) => {
            if let Some((new_pane, _)) =
                state.panes.split(pane_grid::Axis::Vertical, pane, PaneState::new(PaneContent::Text))
            {
                state.focus = Some(new_pane);
                state.pane_count += 1;
            }
        }
        Message::Close(pane) => {
            if state.pane_count > 1 {
                if let Some((_, sibling)) = state.panes.close(pane) {
                    state.focus = Some(sibling);
                    state.pane_count -= 1;
                }
            }
        }
        Message::Clicked(pane) => {
            state.focus = Some(pane);
        }
        Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
            state.panes.drop(pane, target);
        }
        Message::Dragged(_) => {}
        Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
            state.panes.resize(split, ratio);
        }
        Message::ContentChanged(pane, content) => {
            if let Some(pane_state) = state.panes.get_mut(pane) {
                pane_state.content = content;
                pane_state.counter = 0;
            }
        }
        Message::Increment(pane) => {
            if let Some(pane_state) = state.panes.get_mut(pane) {
                pane_state.counter += 1;
            }
        }
        Message::Decrement(pane) => {
            if let Some(pane_state) = state.panes.get_mut(pane) {
                pane_state.counter -= 1;
            }
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let pane_count = state.pane_count;
    let focus = state.focus;

    let pane_grid = PaneGrid::new(&state.panes, move |pane, pane_state, _is_maximized| {
        let is_focused = focus == Some(pane);

        let title_bar = pane_grid::TitleBar::new(
            row![
                text(format!("{}", pane_state.content)).size(14),
                pick_list(
                    PaneContent::ALL,
                    Some(pane_state.content),
                    move |c| Message::ContentChanged(pane, c),
                )
                .text_size(12),
            ]
            .spacing(8),
        )
        .controls(pane_grid::Controls::dynamic(
            view_controls(pane, pane_count),
            view_controls(pane, pane_count),
        ))
        .padding(6);

        let body: Element<Message> = match pane_state.content {
            PaneContent::Text => view_text_content(),
            PaneContent::Counter => view_counter_content(pane, pane_state.counter),
            PaneContent::Colors => view_color_content(),
        };

        let content = container(body)
            .padding(8)
            .width(Fill)
            .height(Fill);

        pane_grid::Content::new(content)
            .title_bar(title_bar)
            .style(if is_focused {
                container::bordered_box
            } else {
                |_theme: &iced::Theme| container::Style::default()
            })
    })
    .on_click(Message::Clicked)
    .on_drag(Message::Dragged)
    .on_resize(10, Message::Resized)
    .spacing(4);

    let header = column![
        text("PaneGrid Playground").size(24),
        text("Split, close, resize, and drag panes. Each pane can display different content types.").size(13),
        rule::horizontal(1),
    ]
    .spacing(8);

    column![header, pane_grid]
        .spacing(12)
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
}

fn view_controls<'a>(pane: pane_grid::Pane, pane_count: usize) -> Element<'a, Message> {
    let close_btn = if pane_count > 1 {
        button(text("×").size(14))
            .on_press(Message::Close(pane))
            .padding([2, 6])
    } else {
        button(text("×").size(14)).padding([2, 6])
    };

    row![
        button(text("━").size(14))
            .on_press(Message::SplitH(pane))
            .padding([2, 6]),
        button(text("┃").size(14))
            .on_press(Message::SplitV(pane))
            .padding([2, 6]),
        close_btn,
    ]
    .spacing(4)
    .into()
}

fn view_text_content<'a>() -> Element<'a, Message> {
    let lines = [
        "This is a text pane.",
        "You can split it horizontally or vertically.",
        "Try dragging panes to rearrange them.",
        "Resize by dragging the borders between panes.",
        "Change the content type using the dropdown above.",
    ];

    let mut col = column![].spacing(4);
    for line in lines {
        col = col.push(text(line).size(13));
    }

    scrollable(col).height(Fill).into()
}

fn view_counter_content(pane: pane_grid::Pane, counter: i32) -> Element<'static, Message> {
    column![
        text(format!("Count: {}", counter)).size(32),
        row![
            button(text("−").size(16))
                .on_press(Message::Decrement(pane))
                .padding([4, 12]),
            button(text("+").size(16))
                .on_press(Message::Increment(pane))
                .padding([4, 12]),
        ]
        .spacing(8),
    ]
    .spacing(12)
    .into()
}

fn view_color_content<'a>() -> Element<'a, Message> {
    let colors = [
        ("Red", [1.0, 0.3, 0.3]),
        ("Green", [0.3, 0.8, 0.3]),
        ("Blue", [0.3, 0.5, 1.0]),
        ("Yellow", [1.0, 0.9, 0.3]),
        ("Purple", [0.7, 0.3, 1.0]),
        ("Cyan", [0.3, 0.9, 0.9]),
    ];

    let mut col = column![].spacing(4);
    for (name, [r, g, b]) in colors {
        let swatch = container(text(name).size(13))
            .padding([6, 12])
            .width(Fill)
            .style(move |_theme: &iced::Theme| {
                container::Style {
                    background: Some(iced::Background::Color(iced::Color::from_rgb(r, g, b))),
                    text_color: Some(iced::Color::BLACK),
                    ..container::Style::default()
                }
            });
        col = col.push(swatch);
    }

    scrollable(col).height(Fill).into()
}
