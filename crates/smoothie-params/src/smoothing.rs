//! Parameter smoothing — prevents zipper noise on audio thread.

/// How a parameter value transition is smoothed over time.
#[derive(Debug, Clone, Copy)]
pub enum SmoothingStyle {
    /// No smoothing — instant jump (only for non-audio parameters).
    None,
    /// Linear interpolation over N milliseconds.
    Linear(f32),
    /// Logarithmic (exponential) smoothing — better for gain.
    Logarithmic(f32),
    /// Spring-damper — overshoot, then settle.
    Spring { stiffness: f32, damping: f32 },
}

/// A per-sample smoother for a single f32 value.
///
/// Runs on the audio thread with zero allocation.
pub struct Smoother {
    current:    f32,
    target:     f32,
    style:      SmoothingStyle,
    sample_rate: f32,
    // For linear/log: coefficient
    coeff:      f32,
    // For spring:
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

    fn compute_coeff(style: &SmoothingStyle, sample_rate: f32) -> f32 {
        match style {
            SmoothingStyle::None => 1.0,
            SmoothingStyle::Linear(ms) => {
                let samples = (ms / 1000.0) * sample_rate;
                if samples > 0.0 { 1.0 / samples } else { 1.0 }
            }
            SmoothingStyle::Logarithmic(ms) => {
                let samples = (ms / 1000.0) * sample_rate;
                if samples > 0.0 {
                    (-1.0_f32 / samples).exp()
                } else {
                    0.0
                }
            }
            SmoothingStyle::Spring { .. } => 0.0,
        }
    }

    /// Set new target value. Call from UI / param change, not audio thread.
    #[inline]
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Advance one sample. Call from audio thread per sample.
    #[inline]
    pub fn next(&mut self) -> f32 {
        match self.style {
            SmoothingStyle::None => {
                self.current = self.target;
            }
            SmoothingStyle::Linear(_) => {
                let diff = self.target - self.current;
                if diff.abs() < 1e-6 {
                    self.current = self.target;
                } else {
                    self.current += diff.signum() * self.coeff.min(diff.abs());
                }
            }
            SmoothingStyle::Logarithmic(_) => {
                self.current = self.target + self.coeff * (self.current - self.target);
            }
            SmoothingStyle::Spring { stiffness, damping } => {
                let dt = 1.0 / self.sample_rate;
                let force = stiffness * (self.target - self.current) - damping * self.velocity;
                self.velocity += force * dt;
                self.current += self.velocity * dt;
            }
        }
        self.current
    }

    /// Current smoothed value without advancing.
    #[inline]
    pub fn current(&self) -> f32 { self.current }

    /// Is the smoother settled at the target?
    #[inline]
    pub fn is_settled(&self) -> bool { (self.current - self.target).abs() < 1e-6 }
}
