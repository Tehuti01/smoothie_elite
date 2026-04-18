//! Parameter smoothing — prevents zipper noise on audio thread.

/// How a parameter value transition is smoothed over time.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SmoothingStyle {
    /// No smoothing — instant jump (only for non-audio parameters).
    None,
    /// Linear interpolation over N milliseconds.
    Linear(f32),
    /// Logarithmic (exponential) smoothing — better for gain.
    Logarithmic(f32),
    /// Golden ratio smoothing (PHI-aligned time constant).
    Golden,
    /// Spring-damper — overshoot, then settle.
    Spring { stiffness: f32, damping: f32 },
}

/// A per-sample/block smoother for a single f32 value.
pub struct Smoother {
    current:    f32,
    target:     f32,
    style:      SmoothingStyle,
    sample_rate: f32,
    coeff:      f32,
    velocity:   f32,
}

impl Smoother {
    pub fn new(initial: f32, sample_rate: f32, style: SmoothingStyle) -> Self {
        let coeff = Self::compute_coeff(&style, sample_rate);
        Self {
            current: initial,
            target:  initial,
            style,
            sample_rate,
            coeff,
            velocity: 0.0,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if (self.sample_rate - sample_rate).abs() > 1e-3 {
            self.sample_rate = sample_rate;
            self.coeff = Self::compute_coeff(&self.style, sample_rate);
        }
    }

    fn compute_coeff(style: &SmoothingStyle, sample_rate: f32) -> f32 {
        const PHI: f32 = 1.61803398875;
        match style {
            SmoothingStyle::None => 1.0,
            SmoothingStyle::Linear(ms) => {
                let samples = (ms / 1000.0) * sample_rate;
                if samples > 0.0 { 1.0 / samples } else { 1.0 }
            }
            SmoothingStyle::Logarithmic(ms) => {
                let samples = (ms / 1000.0) * sample_rate;
                if samples > 0.0 { (-1.0_f32 / samples).exp() } else { 0.0 }
            }
            SmoothingStyle::Golden => {
                let samples = (PHI / 1000.0) * sample_rate;
                (-1.0_f32 / samples).exp()
            }
            SmoothingStyle::Spring { .. } => 0.0,
        }
    }

    #[inline]
    pub fn set_target(&mut self, target: f32) { self.target = target; }

    #[inline]
    pub fn next(&mut self) -> f32 {
        match self.style {
            SmoothingStyle::None => { self.current = self.target; }
            SmoothingStyle::Linear(_) => {
                let diff = self.target - self.current;
                if diff.abs() < 1e-9 { self.current = self.target; }
                else { self.current += diff.signum() * self.coeff.min(diff.abs()); }
            }
            SmoothingStyle::Logarithmic(_) | SmoothingStyle::Golden => {
                self.current = self.target + self.coeff * (self.current - self.target);
                if (self.current - self.target).abs() < 1e-9 { self.current = self.target; }
            }
            SmoothingStyle::Spring { stiffness, damping } => {
                let dt = 1.0 / self.sample_rate;
                let force = stiffness * (self.target - self.current) - damping * self.velocity;
                self.velocity += force * dt;
                self.current += self.velocity * dt;
                if (self.current - self.target).abs() < 1e-9 && self.velocity.abs() < 1e-7 {
                    self.current = self.target;
                    self.velocity = 0.0;
                }
            }
        }
        self.current
    }

    pub fn process_block(&mut self, buffer: &mut [f32]) {
        match self.style {
            SmoothingStyle::None => {
                self.current = self.target;
                buffer.fill(self.current);
            }
            _ => {
                for sample in buffer.iter_mut() {
                    *sample = self.next();
                }
            }
        }
    }

    #[inline]
    pub fn current(&self) -> f32 { self.current }

    #[inline]
    pub fn is_settled(&self) -> bool { (self.current - self.target).abs() < 1e-9 }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
