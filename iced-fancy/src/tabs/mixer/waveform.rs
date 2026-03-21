use iced::widget::canvas::{self, Cache, Frame};
use iced::{Color, Rectangle};

use super::channel::Channel;

/// Draws a single channel waveform onto the canvas.
pub fn draw_channel_waveform(
    frame: &mut Frame,
    bounds: Rectangle,
    channel: &Channel,
    elapsed: f32,
) {
    draw_background(frame, bounds);
    draw_grid(frame, bounds);

    if !channel.enabled {
        frame.fill_text(canvas::Text {
            content: "MUTED".to_string(),
            position: iced::Point::new(bounds.width / 2.0 - 24.0, bounds.height / 2.0 - 8.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
            size: iced::Pixels(16.0),
            ..Default::default()
        });
        return;
    }

    let w = bounds.width;
    let h = bounds.height;
    let cy = h / 2.0;
    let amp = cy * 0.85;
    let steps = (w as usize).max(100);

    let mut path = canvas::path::Builder::new();
    let mut first = true;

    let time_window = 2.0;
    for i in 0..=steps {
        let frac = i as f32 / steps as f32;
        let t = elapsed - time_window / 2.0 + frac * time_window;
        let sample = channel.sample(t);
        let x = frac * w;
        let y = cy - sample * amp;

        if first {
            path.move_to(iced::Point::new(x, y));
            first = false;
        } else {
            path.line_to(iced::Point::new(x, y));
        }
    }

    let stroke = canvas::Stroke {
        style: canvas::Style::Solid(channel.color),
        width: 2.0,
        ..Default::default()
    };
    frame.stroke(&path.build(), stroke);
}

/// Draws the master combined waveform.
pub fn draw_master_waveform(
    frame: &mut Frame,
    bounds: Rectangle,
    channels: &[Channel],
    elapsed: f32,
) {
    draw_background(frame, bounds);
    draw_grid(frame, bounds);

    frame.fill_text(canvas::Text {
        content: "Master Output".to_string(),
        position: iced::Point::new(8.0, 4.0),
        color: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
        size: iced::Pixels(12.0),
        ..Default::default()
    });

    let w = bounds.width;
    let h = bounds.height;
    let cy = h / 2.0;
    let amp = cy * 0.8;
    let steps = (w as usize).max(100);
    let time_window = 2.0;

    let enabled_count = channels.iter().filter(|c| c.enabled).count();
    if enabled_count == 0 {
        frame.fill_text(canvas::Text {
            content: "No channels enabled".to_string(),
            position: iced::Point::new(w / 2.0 - 60.0, cy - 8.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
            size: iced::Pixels(14.0),
            ..Default::default()
        });
        return;
    }

    // Draw each channel's contribution as a faint line
    for ch in channels.iter().filter(|c| c.enabled) {
        let mut path = canvas::path::Builder::new();
        let mut first = true;
        for i in 0..=steps {
            let frac = i as f32 / steps as f32;
            let t = elapsed - time_window / 2.0 + frac * time_window;
            let sample = ch.sample(t);
            let x = frac * w;
            let y = cy - sample * amp / enabled_count.max(1) as f32;
            if first {
                path.move_to(iced::Point::new(x, y));
                first = false;
            } else {
                path.line_to(iced::Point::new(x, y));
            }
        }
        let mut c = ch.color;
        c.a = 0.25;
        frame.stroke(
            &path.build(),
            canvas::Stroke {
                style: canvas::Style::Solid(c),
                width: 1.0,
                ..Default::default()
            },
        );
    }

    // Draw combined waveform
    let mut path = canvas::path::Builder::new();
    let mut first = true;
    for i in 0..=steps {
        let frac = i as f32 / steps as f32;
        let t = elapsed - time_window / 2.0 + frac * time_window;
        let combined: f32 = channels.iter().map(|c| c.sample(t)).sum();
        let normalized = combined / enabled_count.max(1) as f32;
        let x = frac * w;
        let y = cy - normalized * amp;
        if first {
            path.move_to(iced::Point::new(x, y));
            first = false;
        } else {
            path.line_to(iced::Point::new(x, y));
        }
    }

    frame.stroke(
        &path.build(),
        canvas::Stroke {
            style: canvas::Style::Solid(Color::WHITE),
            width: 2.5,
            ..Default::default()
        },
    );
}

fn draw_background(frame: &mut Frame, bounds: Rectangle) {
    frame.fill_rectangle(
        iced::Point::ORIGIN,
        bounds.size(),
        Color::from_rgb(0.06, 0.06, 0.10),
    );
}

fn draw_grid(frame: &mut Frame, bounds: Rectangle) {
    let w = bounds.width;
    let h = bounds.height;
    let cy = h / 2.0;

    let mut center = canvas::path::Builder::new();
    center.move_to(iced::Point::new(0.0, cy));
    center.line_to(iced::Point::new(w, cy));
    frame.stroke(
        &center.build(),
        canvas::Stroke {
            style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.15)),
            width: 1.0,
            ..Default::default()
        },
    );

    for &frac in &[0.25, 0.75] {
        let mut line = canvas::path::Builder::new();
        let y = h * frac;
        line.move_to(iced::Point::new(0.0, y));
        line.line_to(iced::Point::new(w, y));
        frame.stroke(
            &line.build(),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.06)),
                width: 1.0,
                ..Default::default()
            },
        );
    }
}

/// Per-channel canvas — caches cleared each tick.
#[derive(Debug)]
pub struct SingleChannelCanvas {
    pub cache: Cache,
}

impl SingleChannelCanvas {
    pub fn new(_index: usize) -> Self {
        Self { cache: Cache::new() }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

/// Master canvas — cache cleared each tick.
#[derive(Debug)]
pub struct MasterCanvas {
    pub cache: Cache,
}

impl MasterCanvas {
    pub fn new() -> Self {
        Self { cache: Cache::new() }
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}
