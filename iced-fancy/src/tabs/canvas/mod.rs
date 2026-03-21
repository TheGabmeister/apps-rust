pub mod procedural_art;
pub mod data_viz;

use iced::widget::{button, canvas, column, container, pick_list, row, rule, scrollable, slider, text};
use iced::{Element, Fill};

use procedural_art::{Palette, Pattern};

#[derive(Debug)]
pub struct State {
    pub art: procedural_art::ProceduralArt,
    pub viz: data_viz::DataViz,
}

impl Default for State {
    fn default() -> Self {
        Self {
            art: procedural_art::ProceduralArt::new(),
            viz: data_viz::DataViz::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetPattern(Pattern),
    SetPalette(Palette),
    SetSpeed(f32),
    SetComplexity(f32),
    Randomize,
    Tick(f32),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SetPattern(p) => {
            state.art.pattern = p;
            state.art.clear_cache();
        }
        Message::SetPalette(p) => {
            state.art.palette = p;
            state.art.clear_cache();
        }
        Message::SetSpeed(v) => {
            state.art.speed = v;
        }
        Message::SetComplexity(v) => {
            state.art.complexity = v;
            state.art.clear_cache();
        }
        Message::Randomize => {
            state.viz.randomize();
        }
        Message::Tick(dt) => {
            state.art.tick(dt);
            state.viz.tick(dt);
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let header = column![
        text("Canvas & Procedural Art").size(24),
        text("Generative art, animated charts, and real-time canvas rendering").size(13),
        rule::horizontal(1),
    ]
    .spacing(8);

    // --- Controls ---
    let pattern_pick = row![
        text("Pattern:").size(13),
        pick_list(
            Pattern::ALL,
            Some(state.art.pattern),
            Message::SetPattern,
        )
        .text_size(13),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let palette_pick = row![
        text("Palette:").size(13),
        pick_list(
            Palette::ALL,
            Some(state.art.palette),
            Message::SetPalette,
        )
        .text_size(13),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let speed_ctrl = row![
        text(format!("Speed: {:.1}x", state.art.speed)).size(13).width(100),
        slider(0.1..=5.0, state.art.speed, Message::SetSpeed).step(0.1),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let complexity_ctrl = row![
        text(format!("Complexity: {:.0}", state.art.complexity)).size(13).width(100),
        slider(1.0..=15.0, state.art.complexity, Message::SetComplexity).step(1.0),
    ]
    .spacing(8)
    .align_y(iced::Alignment::Center);

    let randomize_btn = button(text("Randomize Charts").size(13))
        .on_press(Message::Randomize)
        .padding([6, 16]);

    let controls = row![
        pattern_pick,
        palette_pick,
        speed_ctrl,
        complexity_ctrl,
        randomize_btn,
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);

    let controls_scroll = scrollable(
        container(controls).padding(4)
    )
    .direction(scrollable::Direction::Horizontal(
        scrollable::Scrollbar::new(),
    ));

    // --- Canvases ---
    let art_canvas = container(
        canvas(&state.art)
            .width(Fill)
            .height(Fill),
    )
    .width(Fill)
    .height(Fill);

    let viz_canvas = container(
        canvas(&state.viz)
            .width(Fill)
            .height(Fill),
    )
    .width(Fill)
    .height(Fill);

    let canvases = row![art_canvas, viz_canvas]
        .spacing(8)
        .height(Fill);

    column![
        header,
        controls_scroll,
        canvases,
    ]
    .spacing(12)
    .padding(16)
    .width(Fill)
    .height(Fill)
    .into()
}
