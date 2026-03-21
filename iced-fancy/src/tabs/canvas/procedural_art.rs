use iced::widget::canvas::{self, Cache, Frame, Geometry};
use iced::{Color, Rectangle, Renderer, Theme};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pattern {
    Spirograph,
    Lissajous,
    Particles,
}

impl Pattern {
    pub const ALL: &'static [Pattern] = &[Pattern::Spirograph, Pattern::Lissajous, Pattern::Particles];
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Spirograph => write!(f, "Spirograph"),
            Pattern::Lissajous => write!(f, "Lissajous"),
            Pattern::Particles => write!(f, "Particles"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    Rainbow,
    Ocean,
    Fire,
    Neon,
}

impl Palette {
    pub const ALL: &'static [Palette] = &[Palette::Rainbow, Palette::Ocean, Palette::Fire, Palette::Neon];

    pub fn color(self, t: f32) -> Color {
        let t = t.fract();
        match self {
            Palette::Rainbow => {
                let r = (t * 2.0 * PI).sin() * 0.5 + 0.5;
                let g = (t * 2.0 * PI + 2.094).sin() * 0.5 + 0.5;
                let b = (t * 2.0 * PI + 4.189).sin() * 0.5 + 0.5;
                Color::from_rgb(r, g, b)
            }
            Palette::Ocean => {
                let r = 0.1;
                let g = 0.3 + t * 0.4;
                let b = 0.5 + t * 0.5;
                Color::from_rgb(r, g, b)
            }
            Palette::Fire => {
                let r = 0.8 + t * 0.2;
                let g = 0.2 + t * 0.5;
                let b = 0.05;
                Color::from_rgb(r, g, b)
            }
            Palette::Neon => {
                let r = (t * 4.0 * PI).sin() * 0.5 + 0.5;
                let g = (t * 4.0 * PI + 1.0).cos() * 0.5 + 0.5;
                let b = 0.8 + (t * 2.0 * PI).sin() * 0.2;
                Color::from_rgb(r, g, b)
            }
        }
    }
}

impl std::fmt::Display for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Palette::Rainbow => write!(f, "Rainbow"),
            Palette::Ocean => write!(f, "Ocean"),
            Palette::Fire => write!(f, "Fire"),
            Palette::Neon => write!(f, "Neon"),
        }
    }
}

#[derive(Debug)]
pub struct ProceduralArt {
    pub pattern: Pattern,
    pub speed: f32,
    pub complexity: f32,
    pub palette: Palette,
    pub elapsed: f32,
    cache: Cache,
}

impl ProceduralArt {
    pub fn new() -> Self {
        Self {
            pattern: Pattern::Spirograph,
            speed: 1.0,
            complexity: 5.0,
            palette: Palette::Rainbow,
            elapsed: 0.0,
            cache: Cache::new(),
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt * self.speed;
        self.cache.clear();
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl canvas::Program<super::Message> for ProceduralArt {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<Geometry> {
        let geom = self.cache.draw(renderer, bounds.size(), |frame| {
            let cx = bounds.width / 2.0;
            let cy = bounds.height / 2.0;
            let radius = cx.min(cy) * 0.85;

            // Dark background
            frame.fill_rectangle(
                iced::Point::ORIGIN,
                bounds.size(),
                Color::from_rgb(0.08, 0.08, 0.12),
            );

            match self.pattern {
                Pattern::Spirograph => draw_spirograph(frame, cx, cy, radius, self),
                Pattern::Lissajous => draw_lissajous(frame, cx, cy, radius, self),
                Pattern::Particles => draw_particles(frame, cx, cy, radius, self),
            }
        });

        vec![geom]
    }
}

fn draw_spirograph(frame: &mut Frame, cx: f32, cy: f32, radius: f32, art: &ProceduralArt) {
    let k = art.complexity.round() as u32;
    let l = 0.6;
    let steps = 2000;
    let t = art.elapsed;

    let big_r = radius * 0.5;
    let _small_r = big_r / (k.max(2) as f32);

    let mut path = canvas::path::Builder::new();
    let mut first = true;

    for i in 0..=steps {
        let theta = (i as f32 / steps as f32) * 2.0 * PI * (k.max(2) as f32) + t;
        let k_f = k.max(2) as f32;
        let x = cx + big_r * ((1.0 - 1.0 / k_f) * theta.cos() + l / k_f * ((k_f - 1.0) * theta).cos());
        let y = cy + big_r * ((1.0 - 1.0 / k_f) * theta.sin() - l / k_f * ((k_f - 1.0) * theta).sin());

        if first {
            path.move_to(iced::Point::new(x, y));
            first = false;
        } else {
            path.line_to(iced::Point::new(x, y));
        }

        // Draw colored dots along the path periodically
        if i % 20 == 0 {
            let color_t = (i as f32 / steps as f32 + t * 0.1).fract();
            let color = art.palette.color(color_t);
            let dot_size = 2.0 + (theta * 0.5).sin().abs() * 2.0;
            frame.fill(
                &canvas::Path::circle(iced::Point::new(x, y), dot_size),
                color,
            );
        }
    }

    let stroke = canvas::Stroke {
        style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
        width: 1.0,
        ..Default::default()
    };
    frame.stroke(&path.build(), stroke);
}

fn draw_lissajous(frame: &mut Frame, cx: f32, cy: f32, radius: f32, art: &ProceduralArt) {
    let a = art.complexity.round().max(1.0);
    let b = a + 1.0;
    let delta = art.elapsed;
    let steps = 1500;

    let mut path = canvas::path::Builder::new();
    let mut first = true;

    for i in 0..=steps {
        let t = (i as f32 / steps as f32) * 2.0 * PI;
        let x = cx + radius * 0.9 * (a * t + delta).sin();
        let y = cy + radius * 0.9 * (b * t).sin();

        if first {
            path.move_to(iced::Point::new(x, y));
            first = false;
        } else {
            path.line_to(iced::Point::new(x, y));
        }

        if i % 15 == 0 {
            let color_t = (i as f32 / steps as f32 + delta * 0.05).fract();
            let color = art.palette.color(color_t);
            let dot_size = 1.5 + (t * 3.0 + delta).sin().abs() * 3.0;
            frame.fill(
                &canvas::Path::circle(iced::Point::new(x, y), dot_size),
                color,
            );
        }
    }

    let stroke = canvas::Stroke {
        style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.25)),
        width: 1.0,
        ..Default::default()
    };
    frame.stroke(&path.build(), stroke);

    // Draw a "trace point" moving along the curve
    let trace_t = (art.elapsed * 0.5) % (2.0 * PI);
    let tx = cx + radius * 0.9 * (a * trace_t + delta).sin();
    let ty = cy + radius * 0.9 * (b * trace_t).sin();
    let trace_color = art.palette.color((art.elapsed * 0.1).fract());
    frame.fill(
        &canvas::Path::circle(iced::Point::new(tx, ty), 6.0),
        trace_color,
    );
}

fn draw_particles(frame: &mut Frame, cx: f32, cy: f32, radius: f32, art: &ProceduralArt) {
    let count = (art.complexity * 40.0).round() as u32;
    let t = art.elapsed;

    for i in 0..count {
        let seed = i as f32 * 2.399; // golden angle
        let r_base = (seed * 7.13).sin() * 0.5 + 0.5;
        let angle = seed + t * (0.3 + r_base * 0.5);
        let dist = radius * (0.1 + r_base * 0.85) * (1.0 + 0.15 * (t * 0.7 + seed).sin());

        let x = cx + dist * angle.cos();
        let y = cy + dist * angle.sin();

        let color_t = (i as f32 / count as f32 + t * 0.05).fract();
        let color = art.palette.color(color_t);
        let size = 2.0 + (seed * 3.7 + t).sin().abs() * 5.0;

        frame.fill(
            &canvas::Path::circle(iced::Point::new(x, y), size),
            color,
        );
    }

    // Draw connecting lines between nearby particles (limited for performance)
    let line_count = count.min(80);
    for i in 0..line_count {
        let seed_a = i as f32 * 2.399;
        let r_a = (seed_a * 7.13).sin() * 0.5 + 0.5;
        let angle_a = seed_a + t * (0.3 + r_a * 0.5);
        let dist_a = radius * (0.1 + r_a * 0.85) * (1.0 + 0.15 * (t * 0.7 + seed_a).sin());
        let ax = cx + dist_a * angle_a.cos();
        let ay = cy + dist_a * angle_a.sin();

        let j = (i + 1) % line_count;
        let seed_b = j as f32 * 2.399;
        let r_b = (seed_b * 7.13).sin() * 0.5 + 0.5;
        let angle_b = seed_b + t * (0.3 + r_b * 0.5);
        let dist_b = radius * (0.1 + r_b * 0.85) * (1.0 + 0.15 * (t * 0.7 + seed_b).sin());
        let bx = cx + dist_b * angle_b.cos();
        let by = cy + dist_b * angle_b.sin();

        let dx = ax - bx;
        let dy = ay - by;
        let d = (dx * dx + dy * dy).sqrt();
        if d < radius * 0.4 {
            let alpha = 0.15 * (1.0 - d / (radius * 0.4));
            let mut line = canvas::path::Builder::new();
            line.move_to(iced::Point::new(ax, ay));
            line.line_to(iced::Point::new(bx, by));
            let stroke = canvas::Stroke {
                style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, alpha)),
                width: 0.5,
                ..Default::default()
            };
            frame.stroke(&line.build(), stroke);
        }
    }
}
