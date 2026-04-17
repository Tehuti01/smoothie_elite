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
    /// Spring-damper — overshoot, then settle.
    Spring { stiffness: f32, damping: f32 },
}

/// A per-sample/block smoother for a single f32 value.
///
/// Runs on the audio thread with zero allocation.
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

    /// Update the sample rate and recalculate coefficients.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        if (self.sample_rate - sample_rate).abs() > 1e-3 {
            self.sample_rate = sample_rate;
            self.coeff = Self::compute_coeff(&self.style, sample_rate);
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
                    // One-pole smoothing: y = y + (x - y) * (1 - exp(-1/tau))
                    // Simplified: y = target + coeff * (current - target)
                    (-1.0_f32 / samples).exp()
                } else {
                    0.0
                }
            }
            SmoothingStyle::Spring { .. } => 0.0,
        }
    }

    /// Set new target value.
    #[inline]
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Advance one sample.
    #[inline]
    pub fn next(&mut self) -> f32 {
        match self.style {
            SmoothingStyle::None => {
                self.current = self.target;
            }
            SmoothingStyle::Linear(_) => {
                let diff = self.target - self.current;
                if diff.abs() < 1e-9 {
                    self.current = self.target;
                } else {
                    self.current += diff.signum() * self.coeff.min(diff.abs());
                }
            }
            SmoothingStyle::Logarithmic(_) => {
                // One-pole lowpass filter
                self.current = self.target + self.coeff * (self.current - self.target);
                if (self.current - self.target).abs() < 1e-9 {
                    self.current = self.target;
                }
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

    /// Process a block of samples, filling the buffer with smoothed values.
    /// This is significantly more efficient than calling `next()` in a loop.
    pub fn process_block(&mut self, buffer: &mut [f32]) {
        match self.style {
            SmoothingStyle::None => {
                self.current = self.target;
                buffer.fill(self.current);
            }
            SmoothingStyle::Linear(_) => {
                for sample in buffer.iter_mut() {
                    *sample = self.next();
                }
            }
            SmoothingStyle::Logarithmic(_) => {
                let c = self.coeff;
                let t = self.target;
                let mut curr = self.current;
                
                for sample in buffer.iter_mut() {
                    curr = t + c * (curr - t);
                    *sample = curr;
                }
                
                if (curr - t).abs() < 1e-9 {
                    curr = t;
                }
                self.current = curr;
            }
            SmoothingStyle::Spring { .. } => {
                for sample in buffer.iter_mut() {
                    *sample = self.next();
                }
            }
        }
    }

    /// Current smoothed value without advancing.
    #[inline]
    pub fn current(&self) -> f32 { self.current }

    /// Is the smoother settled at the target?
    #[inline]
    pub fn is_settled(&self) -> bool { (self.current - self.target).abs() < 1e-9 }
}
