/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6795dab8 | REVISION: 2026.04.20                           │
 * │ PATH: crates/03-cognition/smoothie-params/src/smoothing/mod.rs           │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Parameter smoothing implementations.                        │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

/// Technical implementation of the ParameterSmoother structure.
/// Provides one-pole (exponential) smoothing for audio parameters.
pub struct ParameterSmoother {
    value: f32,
    target: f32,
    coeff: f32,
    time_ms: f32,
    sample_rate: f32,
}

impl ParameterSmoother {
    /// Initializes a new instance with an initial value, smoothing time in ms, and sample rate.
    pub fn new(initial_value: f32, time_ms: f32, sample_rate: f32) -> Self {
        let mut s = Self {
            value: initial_value,
            target: initial_value,
            coeff: 0.0,
            time_ms,
            sample_rate,
        };
        s.update_coeff();
        s
    }

    /// Technical implementation of the next logic.
    pub fn process(&mut self) -> f32 {
        self.value = self.value + self.coeff * (self.target - self.value);
        self.value
    }

    /// Technical implementation of the set_target logic.
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sr: f32) {
        self.sample_rate = sr;
        self.update_coeff();
    }

    /// Snaps the current value immediately to the target.
    pub fn snap(&mut self) {
        self.value = self.target;
    }

    /// Internal helper to update smoothing coefficient based on time and sample rate.
    fn update_coeff(&mut self) {
        if self.time_ms <= 0.0 {
            self.coeff = 1.0;
        } else {
            // Standard one-pole coefficient for T60 (time to reach 60dB decay)
            // or here simplified to a 5ms-ish default if time_ms is small.
            self.coeff = 1.0 - (-1.0 / (self.time_ms * 0.001 * self.sample_rate)).exp();
        }
    }
}

/// Technical implementation of the LinearSmoother structure.
pub struct LinearSmoother {
    value: f32,
    target: f32,
    step: f32,
    remaining_samples: usize,
}

impl LinearSmoother {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            target: value,
            step: 0.0,
            remaining_samples: 0,
        }
    }

    pub fn set_target(&mut self, target: f32, samples: usize) {
        self.target = target;
        if samples > 0 {
            self.step = (target - self.value) / samples as f32;
            self.remaining_samples = samples;
        } else {
            self.value = target;
            self.remaining_samples = 0;
        }
    }

    pub fn process(&mut self) -> f32 {
        if self.remaining_samples > 0 {
            self.value += self.step;
            self.remaining_samples -= 1;
            if self.remaining_samples == 0 {
                self.value = self.target;
            }
        }
        self.value
    }
}

/// Technical implementation of the OnePoleSmoother structure (Legacy wrapper).
pub struct OnePoleSmoother(ParameterSmoother);
impl OnePoleSmoother {
    pub fn new(value: f32, time_ms: f32, sample_rate: f32) -> Self {
        Self(ParameterSmoother::new(value, time_ms, sample_rate))
    }
    pub fn process(&mut self) -> f32 {
        self.0.process()
    }
}
