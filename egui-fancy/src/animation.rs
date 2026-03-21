use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Easing {
    Linear,
    EaseInOutCubic,
    EaseOutCubic,
    EaseOutElastic,
    EaseOutBounce,
    EaseInQuad,
}

impl Easing {
    pub fn apply(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Easing::Linear => t,
            Easing::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
                }
            }
            Easing::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            Easing::EaseOutElastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    (2.0_f32).powf(-10.0 * t) * ((t * 10.0 - 0.75) * (2.0 * PI) / 3.0).sin() + 1.0
                }
            }
            Easing::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    let t = t - 1.5 / d1;
                    n1 * t * t + 0.75
                } else if t < 2.5 / d1 {
                    let t = t - 2.25 / d1;
                    n1 * t * t + 0.9375
                } else {
                    let t = t - 2.625 / d1;
                    n1 * t * t + 0.984375
                }
            }
            Easing::EaseInQuad => t * t,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Animation {
    start_time: Option<f64>,
    duration: f32,
    easing: Easing,
    forward: bool,
}

impl Animation {
    pub fn new(duration: f32, easing: Easing) -> Self {
        Self {
            start_time: None,
            duration,
            easing,
            forward: true,
        }
    }

    pub fn start(&mut self, ctx: &egui::Context) {
        self.forward = true;
        self.start_time = Some(ctx.input(|i| i.time));
        ctx.request_repaint();
    }

    pub fn reverse(&mut self, ctx: &egui::Context) {
        self.forward = false;
        self.start_time = Some(ctx.input(|i| i.time));
        ctx.request_repaint();
    }

    pub fn progress(&self, ctx: &egui::Context) -> f32 {
        let Some(start) = self.start_time else {
            return if self.forward { 0.0 } else { 1.0 };
        };
        let elapsed = (ctx.input(|i| i.time) - start) as f32;
        let raw = (elapsed / self.duration).clamp(0.0, 1.0);
        let directed = if self.forward { raw } else { 1.0 - raw };
        let eased = self.easing.apply(directed);
        if raw < 1.0 {
            ctx.request_repaint();
        }
        eased
    }

    pub fn is_active(&self, ctx: &egui::Context) -> bool {
        let Some(start) = self.start_time else {
            return false;
        };
        let elapsed = (ctx.input(|i| i.time) - start) as f32;
        elapsed < self.duration
    }
}
