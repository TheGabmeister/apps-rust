use iced::widget::{
    button, column, container, pick_list, row, rule, scrollable, text, text_input,
    pane_grid::{self, PaneGrid},
};
use iced::{Element, Fill};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaneContent {
    Editor,
    ColorSwatch,
    Counter,
    Placeholder,
}

impl PaneContent {
    const ALL: &'static [PaneContent] = &[
        PaneContent::Editor,
        PaneContent::ColorSwatch,
        PaneContent::Counter,
        PaneContent::Placeholder,
    ];
}

impl std::fmt::Display for PaneContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneContent::Editor => write!(f, "Editor"),
            PaneContent::ColorSwatch => write!(f, "Color Swatch"),
            PaneContent::Counter => write!(f, "Counter"),
            PaneContent::Placeholder => write!(f, "Placeholder"),
        }
    }
}

#[derive(Debug)]
pub struct PaneState {
    pub content: PaneContent,
    pub counter: i32,
    pub editor_text: String,
}

impl PaneState {
    fn new(content: PaneContent) -> Self {
        Self {
            content,
            counter: 0,
            editor_text: String::new(),
        }
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
            a: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::Editor))),
            b: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.5,
                a: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::Counter))),
                b: Box::new(pane_grid::Configuration::Pane(PaneState::new(PaneContent::ColorSwatch))),
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
    EditorChanged(pane_grid::Pane, String),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SplitH(pane) => {
            if let Some((new_pane, _)) =
                state.panes.split(pane_grid::Axis::Horizontal, pane, PaneState::new(PaneContent::Placeholder))
            {
                state.focus = Some(new_pane);
                state.pane_count += 1;
            }
        }
        Message::SplitV(pane) => {
            if let Some((new_pane, _)) =
                state.panes.split(pane_grid::Axis::Vertical, pane, PaneState::new(PaneContent::Placeholder))
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
                pane_state.editor_text.clear();
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
        Message::EditorChanged(pane, value) => {
            if let Some(pane_state) = state.panes.get_mut(pane) {
                pane_state.editor_text = value;
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
            PaneContent::Editor => view_editor_content(pane, &pane_state.editor_text),
            PaneContent::ColorSwatch => view_color_swatch(),
            PaneContent::Counter => view_counter_content(pane, pane_state.counter),
            PaneContent::Placeholder => view_placeholder(),
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

fn view_editor_content(pane: pane_grid::Pane, editor_text: &str) -> Element<'_, Message> {
    let input = text_input("Type something here...", editor_text)
        .on_input(move |val| Message::EditorChanged(pane, val));

    let preview = container(
        scrollable(
            text(if editor_text.is_empty() {
                "Your text will appear here as you type...".to_string()
            } else {
                editor_text.to_string()
            })
            .size(13),
        )
        .height(Fill),
    )
    .padding(8)
    .width(Fill)
    .height(Fill)
    .style(container::bordered_box);

    column![
        text("Text Editor").size(13),
        input,
        text("Preview:").size(12),
        preview,
    ]
    .spacing(6)
    .height(Fill)
    .into()
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

fn view_color_swatch<'a>() -> Element<'a, Message> {
    let swatches = [
        ("Red",    [0.90, 0.30, 0.30]),
        ("Orange", [0.95, 0.60, 0.20]),
        ("Yellow", [0.95, 0.85, 0.25]),
        ("Green",  [0.30, 0.75, 0.35]),
        ("Blue",   [0.30, 0.50, 0.90]),
        ("Purple", [0.65, 0.30, 0.85]),
    ];

    let mut col = column![text("Color Swatches").size(13)].spacing(4);
    for (name, [r, g, b]) in swatches {
        let swatch = container(
            text(format!("  {}  ", name)).size(14),
        )
        .padding([10, 16])
        .width(Fill)
        .style(move |_theme: &iced::Theme| {
            container::Style {
                background: Some(iced::Background::Color(iced::Color::from_rgb(r, g, b))),
                text_color: Some(iced::Color::BLACK),
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..container::Style::default()
            }
        });
        col = col.push(swatch);
    }

    scrollable(col).height(Fill).into()
}

fn view_placeholder<'a>() -> Element<'a, Message> {
    let placeholder_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
        nisi ut aliquip ex ea commodo consequat.\n\n\
        This is a placeholder pane. Use the dropdown above to switch \
        to a different content type, or split this pane to create more.";

    scrollable(
        column![
            text("Placeholder").size(16),
            text(placeholder_text).size(13),
        ]
        .spacing(8),
    )
    .height(Fill)
    .into()
}
