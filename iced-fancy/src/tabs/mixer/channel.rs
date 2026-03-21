use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

impl WaveType {
    pub const ALL: &'static [WaveType] = &[
        WaveType::Sine,
        WaveType::Square,
        WaveType::Sawtooth,
        WaveType::Triangle,
    ];

    /// Sample the waveform at phase t (0..2π) returning -1..1
    pub fn sample(self, t: f32) -> f32 {
        use std::f32::consts::PI;
        match self {
            WaveType::Sine => t.sin(),
            WaveType::Square => {
                if t % (2.0 * PI) < PI { 1.0 } else { -1.0 }
            }
            WaveType::Sawtooth => {
                let p = (t / (2.0 * PI)).fract();
                2.0 * p - 1.0
            }
            WaveType::Triangle => {
                let p = (t / (2.0 * PI)).fract();
                if p < 0.5 { 4.0 * p - 1.0 } else { 3.0 - 4.0 * p }
            }
        }
    }
}

impl std::fmt::Display for WaveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaveType::Sine => write!(f, "Sine"),
            WaveType::Square => write!(f, "Square"),
            WaveType::Sawtooth => write!(f, "Sawtooth"),
            WaveType::Triangle => write!(f, "Triangle"),
        }
    }
}

pub const CHANNEL_COUNT: usize = 4;

pub const DEFAULT_COLORS: [Color; CHANNEL_COUNT] = [
    Color { r: 0.4, g: 0.7, b: 1.0, a: 1.0 },   // Blue
    Color { r: 1.0, g: 0.5, b: 0.3, a: 1.0 },   // Orange
    Color { r: 0.4, g: 0.9, b: 0.5, a: 1.0 },   // Green
    Color { r: 0.9, g: 0.4, b: 0.8, a: 1.0 },   // Pink
];

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub enabled: bool,
    pub volume: f32,      // 0.0 .. 1.0
    pub eq_bass: f32,     // -1.0 .. 1.0
    pub eq_treble: f32,   // -1.0 .. 1.0
    pub wave_type: WaveType,
    pub frequency: f32,   // Hz, 0.5 .. 10.0
    pub amplitude: f32,   // 0.0 .. 1.0
    pub color: Color,
    pub show_color_picker: bool,
}

impl Channel {
    pub fn new(index: usize) -> Self {
        Self {
            name: format!("CH {}", index + 1),
            enabled: true,
            volume: 0.75,
            eq_bass: 0.0,
            eq_treble: 0.0,
            wave_type: WaveType::Sine,
            frequency: 1.0 + index as f32 * 0.7,
            amplitude: 0.8,
            color: DEFAULT_COLORS[index % CHANNEL_COUNT],
            show_color_picker: false,
        }
    }

    /// Compute a sample at time t (seconds), with all channel parameters applied.
    pub fn sample(&self, t: f32) -> f32 {
        if !self.enabled {
            return 0.0;
        }
        let phase = t * self.frequency * std::f32::consts::TAU;
        let raw = self.wave_type.sample(phase);
        raw * self.amplitude * self.volume
    }
}
