pub mod channel;
pub mod controls;
pub mod waveform;

use iced::widget::{canvas, column, container, responsive, row, rule, scrollable, text};
use iced::{Color, Element, Fill, Length};

use channel::{Channel, WaveType, CHANNEL_COUNT};
use waveform::{MasterCanvas, SingleChannelCanvas};

#[derive(Debug)]
pub struct State {
    pub channels: Vec<Channel>,
    pub elapsed: f32,
    pub channel_canvases: Vec<SingleChannelCanvas>,
    pub master_canvas: MasterCanvas,
}

impl Default for State {
    fn default() -> Self {
        Self {
            channels: (0..CHANNEL_COUNT).map(Channel::new).collect(),
            elapsed: 0.0,
            channel_canvases: (0..CHANNEL_COUNT).map(SingleChannelCanvas::new).collect(),
            master_canvas: MasterCanvas::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetEnabled(usize, bool),
    SetVolume(usize, f32),
    SetFrequency(usize, f32),
    SetAmplitude(usize, f32),
    SetEqBass(usize, f32),
    SetEqTreble(usize, f32),
    SetWaveType(usize, WaveType),
    SetColor(usize, Color),
    ToggleColorPicker(usize),
    CancelColorPicker(usize),
    Tick(f32),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SetEnabled(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.enabled = v;
            }
        }
        Message::SetVolume(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.volume = v;
            }
        }
        Message::SetFrequency(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.frequency = v;
            }
        }
        Message::SetAmplitude(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.amplitude = v;
            }
        }
        Message::SetEqBass(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.eq_bass = v;
            }
        }
        Message::SetEqTreble(i, v) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.eq_treble = v;
            }
        }
        Message::SetWaveType(i, w) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.wave_type = w;
            }
        }
        Message::SetColor(i, c) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.color = c;
                ch.show_color_picker = false;
            }
        }
        Message::ToggleColorPicker(i) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.show_color_picker = !ch.show_color_picker;
            }
        }
        Message::CancelColorPicker(i) => {
            if let Some(ch) = state.channels.get_mut(i) {
                ch.show_color_picker = false;
            }
        }
        Message::Tick(dt) => {
            state.elapsed += dt;
            for c in &mut state.channel_canvases {
                c.clear();
            }
            state.master_canvas.clear();
        }
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let header = column![
        text("Audio Mixer").size(24),
        text("4-channel mixer with real-time waveform rendering. Each channel has independent controls.").size(13),
        rule::horizontal(1),
    ]
    .spacing(8);

    // Use responsive to switch between horizontal (wide) and vertical (narrow) layout
    let channels = &state.channels;
    let elapsed = state.elapsed;

    let content = responsive(move |size| {
        let wide = size.width > 800.0;

        // Build channel strips with their waveforms
        let mut strips: Vec<Element<'_, Message>> = Vec::new();
        for (i, ch) in channels.iter().enumerate() {
            let controls = controls::channel_strip(i, ch);

            // Per-channel waveform — draw directly using canvas closure
            let ch_clone = ch.clone();
            let waveform = canvas(
                ChannelRenderer { channel: ch_clone, elapsed },
            )
            .width(Fill)
            .height(Length::Fixed(100.0));

            let strip = column![controls, waveform].spacing(4);

            if wide {
                strips.push(
                    container(strip)
                        .width(Fill)
                        .into(),
                );
            } else {
                strips.push(strip.width(Fill).into());
            }
        }

        // Master waveform
        let master = canvas(
            MasterRenderer { channels: channels.to_vec(), elapsed },
        )
        .width(Fill)
        .height(Length::Fixed(120.0));

        let master_section = column![
            text("Master Output").size(16),
            master,
        ]
        .spacing(4);

        let channel_layout: Element<'_, Message> = if wide {
            let mut r = row![].spacing(8);
            for s in strips {
                r = r.push(s);
            }
            r.into()
        } else {
            let mut c = column![].spacing(8);
            for s in strips {
                c = c.push(s);
            }
            c.into()
        };

        let body = column![channel_layout, rule::horizontal(1), master_section]
            .spacing(12);

        scrollable(body).height(Fill).into()
    });

    column![header, content]
        .spacing(12)
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
}

/// Helper struct to render a single channel's waveform via canvas::Program.
struct ChannelRenderer {
    channel: Channel,
    elapsed: f32,
}

impl std::fmt::Debug for ChannelRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChannelRenderer").finish()
    }
}

impl canvas::Program<Message> for ChannelRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // Draw directly — no cache since we recreate each frame
        let geom = iced::widget::canvas::Cache::new().draw(renderer, bounds.size(), |frame| {
            waveform::draw_channel_waveform(frame, bounds, &self.channel, self.elapsed);
        });
        vec![geom]
    }
}

/// Helper struct to render the master combined waveform.
struct MasterRenderer {
    channels: Vec<Channel>,
    elapsed: f32,
}

impl std::fmt::Debug for MasterRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MasterRenderer").finish()
    }
}

impl canvas::Program<Message> for MasterRenderer {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geom = iced::widget::canvas::Cache::new().draw(renderer, bounds.size(), |frame| {
            waveform::draw_master_waveform(frame, bounds, &self.channels, self.elapsed);
        });
        vec![geom]
    }
}
