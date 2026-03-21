use iced::widget::canvas::{self, Cache, Frame, Geometry};
use iced::{Color, Rectangle, Renderer, Theme};
use std::f32::consts::PI;

const BAR_COUNT: usize = 10;
const LINE_HISTORY: usize = 120;

#[derive(Debug)]
pub struct DataViz {
    // Bar chart
    pub bar_current: Vec<f32>,
    pub bar_target: Vec<f32>,
    // Line chart (scrolling)
    pub line_data: Vec<f32>,
    pub line_offset: f32,
    elapsed: f32,
    cache: Cache,
    seed: f32,
}

impl DataViz {
    pub fn new() -> Self {
        let bars = gen_bars(1.0);
        Self {
            bar_current: bars.clone(),
            bar_target: bars,
            line_data: vec![0.5; LINE_HISTORY],
            line_offset: 0.0,
            elapsed: 0.0,
            cache: Cache::new(),
            seed: 1.0,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
        self.cache.clear();

        // Animate bars toward targets
        for (cur, tgt) in self.bar_current.iter_mut().zip(self.bar_target.iter()) {
            *cur += (*tgt - *cur) * (dt * 5.0).min(1.0);
        }

        // Push new line data point (smooth wave + noise)
        self.line_offset += dt;
        let val = 0.5
            + 0.25 * (self.line_offset * 1.5).sin()
            + 0.15 * (self.line_offset * 3.7 + self.seed).sin()
            + 0.1 * (self.line_offset * 7.3 + self.seed * 2.0).cos();
        self.line_data.push(val.clamp(0.0, 1.0));
        if self.line_data.len() > LINE_HISTORY {
            self.line_data.remove(0);
        }
    }

    pub fn randomize(&mut self) {
        self.seed += 3.7;
        self.bar_target = gen_bars(self.seed);
    }

    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

fn gen_bars(seed: f32) -> Vec<f32> {
    (0..BAR_COUNT)
        .map(|i| {
            let s = seed + i as f32 * 1.618;
            ((s * 13.37).sin() * 0.5 + 0.5).clamp(0.1, 1.0)
        })
        .collect()
}

impl canvas::Program<super::Message> for DataViz {
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
            let w = bounds.width;
            let h = bounds.height;

            // Background
            frame.fill_rectangle(
                iced::Point::ORIGIN,
                bounds.size(),
                Color::from_rgb(0.08, 0.08, 0.12),
            );

            let bar_area_h = h * 0.5 - 20.0;
            let line_area_y = h * 0.5 + 10.0;
            let line_area_h = h * 0.5 - 20.0;

            // --- Bar Chart ---
            draw_bar_chart(frame, 20.0, 10.0, w - 40.0, bar_area_h, &self.bar_current);

            // --- Line Chart ---
            draw_line_chart(frame, 20.0, line_area_y, w - 40.0, line_area_h, &self.line_data, self.elapsed);

            // Divider
            let mut div = canvas::path::Builder::new();
            div.move_to(iced::Point::new(10.0, h * 0.5));
            div.line_to(iced::Point::new(w - 10.0, h * 0.5));
            frame.stroke(
                &div.build(),
                canvas::Stroke {
                    style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.15)),
                    width: 1.0,
                    ..Default::default()
                },
            );
        });

        vec![geom]
    }
}

fn draw_bar_chart(frame: &mut Frame, x: f32, y: f32, w: f32, h: f32, bars: &[f32]) {
    let n = bars.len() as f32;
    let gap = 6.0;
    let bar_w = ((w - gap * (n + 1.0)) / n).max(4.0);

    // Title
    frame.fill_text(canvas::Text {
        content: "Bar Chart".to_string(),
        position: iced::Point::new(x, y),
        color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
        size: iced::Pixels(14.0),
        ..Default::default()
    });

    let chart_y = y + 22.0;
    let chart_h = h - 22.0;

    // Grid lines
    for i in 0..=4 {
        let gy = chart_y + chart_h * (1.0 - i as f32 / 4.0);
        let mut line = canvas::path::Builder::new();
        line.move_to(iced::Point::new(x, gy));
        line.line_to(iced::Point::new(x + w, gy));
        frame.stroke(
            &line.build(),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.08)),
                width: 1.0,
                ..Default::default()
            },
        );
    }

    for (i, &val) in bars.iter().enumerate() {
        let bx = x + gap + i as f32 * (bar_w + gap);
        let bh = val * chart_h;
        let by = chart_y + chart_h - bh;

        // Bar color gradient effect
        let hue = i as f32 / n;
        let r = (hue * 2.0 * PI).sin() * 0.3 + 0.5;
        let g = (hue * 2.0 * PI + 2.094).sin() * 0.3 + 0.5;
        let b = (hue * 2.0 * PI + 4.189).sin() * 0.3 + 0.5;

        frame.fill_rectangle(
            iced::Point::new(bx, by),
            iced::Size::new(bar_w, bh),
            Color::from_rgb(r, g, b),
        );

        // Value label
        let label = format!("{:.0}%", val * 100.0);
        frame.fill_text(canvas::Text {
            content: label,
            position: iced::Point::new(bx + bar_w / 2.0 - 10.0, by - 14.0),
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
            size: iced::Pixels(10.0),
            ..Default::default()
        });
    }
}

fn draw_line_chart(frame: &mut Frame, x: f32, y: f32, w: f32, h: f32, data: &[f32], elapsed: f32) {
    // Title
    frame.fill_text(canvas::Text {
        content: "Scrolling Line Chart".to_string(),
        position: iced::Point::new(x, y),
        color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
        size: iced::Pixels(14.0),
        ..Default::default()
    });

    let chart_y = y + 22.0;
    let chart_h = h - 22.0;

    // Grid lines
    for i in 0..=4 {
        let gy = chart_y + chart_h * (1.0 - i as f32 / 4.0);
        let mut line = canvas::path::Builder::new();
        line.move_to(iced::Point::new(x, gy));
        line.line_to(iced::Point::new(x + w, gy));
        frame.stroke(
            &line.build(),
            canvas::Stroke {
                style: canvas::Style::Solid(Color::from_rgba(1.0, 1.0, 1.0, 0.08)),
                width: 1.0,
                ..Default::default()
            },
        );
    }

    if data.len() < 2 {
        return;
    }

    // Fill area under line
    let step = w / (data.len() - 1) as f32;
    let mut fill_path = canvas::path::Builder::new();
    fill_path.move_to(iced::Point::new(x, chart_y + chart_h));
    for (i, &val) in data.iter().enumerate() {
        let px = x + i as f32 * step;
        let py = chart_y + chart_h * (1.0 - val);
        fill_path.line_to(iced::Point::new(px, py));
    }
    fill_path.line_to(iced::Point::new(x + w, chart_y + chart_h));
    fill_path.close();
    frame.fill(
        &fill_path.build(),
        Color::from_rgba(0.3, 0.6, 1.0, 0.15),
    );

    // Line
    let mut line_path = canvas::path::Builder::new();
    let mut first = true;
    for (i, &val) in data.iter().enumerate() {
        let px = x + i as f32 * step;
        let py = chart_y + chart_h * (1.0 - val);
        if first {
            line_path.move_to(iced::Point::new(px, py));
            first = false;
        } else {
            line_path.line_to(iced::Point::new(px, py));
        }
    }

    let color_phase = (elapsed * 0.3).sin() * 0.2;
    let line_color = Color::from_rgb(0.3 + color_phase, 0.6, 1.0);
    frame.stroke(
        &line_path.build(),
        canvas::Stroke {
            style: canvas::Style::Solid(line_color),
            width: 2.0,
            ..Default::default()
        },
    );

    // Dot at latest point
    if let Some(&last_val) = data.last() {
        let lx = x + w;
        let ly = chart_y + chart_h * (1.0 - last_val);
        frame.fill(
            &canvas::Path::circle(iced::Point::new(lx, ly), 4.0),
            Color::from_rgb(0.4, 0.7, 1.0),
        );
    }
}
