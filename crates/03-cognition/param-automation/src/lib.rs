/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb66db557 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/param-automation/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicU32, Ordering};
use core::f32::consts::PI;

/// Parameter automation envelope
#[derive(Clone, Copy)]
/// Technical implementation of the AutomationCurve enumeration.
pub enum AutomationCurve {
    /// Linear interpolation between points
    Linear,
    /// Exponential curve for perceptually-smooth changes
    Exponential { base: f32 },
    /// Smooth cubic (Catmull-Rom) interpolation
    Cubic,
    /// Instant (step) change
    Instant,
}

/// Automation point in time
#[derive(Clone, Copy)]
/// Technical implementation of the AutomationPoint structure.
pub struct AutomationPoint {
    pub sample_offset: u32,  // Sample number (from frame start)
    pub value: f32,          // 0.0..1.0 normalized
    pub curve: AutomationCurve,
}

/// Technical implementation of the ParameterTrack structure.
pub struct ParameterTrack {
    name: &'static str,
    value: AtomicU32,        // Bit-cast f32 for atomic access
    min: f32,
    max: f32,
    lookahead_buffer: Vec<AutomationPoint>,
    lookahead_index: usize,
}

impl ParameterTrack {
    /// Create new parameter track with min/max bounds
    pub fn new(name: &'static str, min: f32, max: f32, initial: f32) -> Self {
        Self {
            name,
            value: AtomicU32::new(initial.to_bits()),
            min,
            max,
            lookahead_buffer: Vec::with_capacity(4096),
            lookahead_index: 0,
        }
    }

    /// Get current parameter value
    pub fn get(&self) -> f32 {
        f32::from_bits(self.value.load(Ordering::Acquire))
    }

    /// Set parameter value from UI/MIDI thread (lock-free)
    pub fn set_immediate(&self, value: f32) {
        let clamped = value.clamp(self.min, self.max);
        self.value.store(clamped.to_bits(), Ordering::Release);
    }

    /// Queue automation event for future processing
    pub fn queue_automation(&mut self, point: AutomationPoint) {
        if self.lookahead_buffer.len() < self.lookahead_buffer.capacity() {
            self.lookahead_buffer.push(point);
        }
    }

    /// Get parameter value at specific sample offset with lookahead
    pub fn get_at_sample(&mut self, sample_offset: u32) -> f32 {
        // Update lookahead index
        while self.lookahead_index < self.lookahead_buffer.len() {
            if self.lookahead_buffer[self.lookahead_index].sample_offset > sample_offset {
                break;
            }
            self.lookahead_index += 1;
        }

        // Find applicable automation point
        if self.lookahead_index == 0 {
            // No automation yet, use current value
            self.get()
        } else {
            let point = self.lookahead_buffer[self.lookahead_index - 1];
            point.value * (self.max - self.min) + self.min
        }
    }
}

/// Technical implementation of the ParameterRamp structure.
pub struct ParameterRamp {
    current_value: f32,
    target_value: f32,
    samples_remaining: u32,
    curve: AutomationCurve,
}

impl ParameterRamp {
    /// Initializes a new instance of the associated type.
    pub fn new(start: f32, target: f32, duration_samples: u32, curve: AutomationCurve) -> Self {
        Self {
            current_value: start,
            target_value: target,
            samples_remaining: duration_samples,
            curve,
        }
    }

    /// Get next sample value and advance ramp
    pub fn next(&mut self) -> f32 {
        if self.samples_remaining == 0 {
            return self.target_value;
        }

        let progress = 1.0 - (self.samples_remaining as f32 / (self.samples_remaining as f32 + 1.0));

        let value = match self.curve {
            AutomationCurve::Linear => {
                self.current_value + (self.target_value - self.current_value) * progress
            }
            AutomationCurve::Exponential { base } => {
                // Exponential interpolation
                let ratio = self.target_value / (self.current_value + 1e-9);
                self.current_value * ratio.powf(progress)
            }
            AutomationCurve::Cubic => {
                // Catmull-Rom cubic (smooth)
                let p = progress * progress * (3.0 - 2.0 * progress);
                self.current_value + (self.target_value - self.current_value) * p
            }
            AutomationCurve::Instant => {
                if self.samples_remaining == 1 {
                    self.target_value
                } else {
                    self.current_value
                }
            }
        };

        self.samples_remaining -= 1;
        self.current_value = value;
        value
    }

    /// Technical implementation of the is_done logic.
    pub fn is_done(&self) -> bool {
        self.samples_remaining == 0
    }
}

/// Technical implementation of the PolyphonicModulation structure.
pub struct PolyphonicModulation {
    voice_id: u8,
    lfo_frequency: f32,
    lfo_depth: f32,
    lfo_phase: f32,
    envelope_samples_remaining: u32,
    envelope_depth: f32,
}

impl PolyphonicModulation {
    /// Initializes a new instance of the associated type.
    pub fn new(
        voice_id: u8,
        lfo_freq: f32,
        lfo_depth: f32,
        envelope_depth: f32,
    ) -> Self {
        Self {
            voice_id,
            lfo_frequency: lfo_freq,
            lfo_depth,
            lfo_phase: 0.0,
            envelope_samples_remaining: 0,
            envelope_depth,
        }
    }

    /// Get modulated parameter value
    pub fn modulate(&mut self, base_value: f32, sample_rate: f32) -> f32 {
        // LFO component (sine wave)
        let lfo = (self.lfo_phase * 2.0 * PI).sin() * self.lfo_depth;

        // Envelope component (exponential decay)
        let envelope = if self.envelope_samples_remaining > 0 {
            let progress = 1.0 - (self.envelope_samples_remaining as f32 / 1000.0);
            let decay = (-progress * 5.0).exp(); // Exponential decay
            decay * self.envelope_depth
        } else {
            0.0
        };

        // Update LFO phase
        self.lfo_phase += self.lfo_frequency / sample_rate;
        if self.lfo_phase >= 1.0 {
            self.lfo_phase -= 1.0;
        }

        // Decay envelope
        if self.envelope_samples_remaining > 0 {
            self.envelope_samples_remaining -= 1;
        }

        base_value + lfo + envelope
    }

    /// Trigger envelope for this voice
    pub fn trigger_envelope(&mut self, duration_samples: u32) {
        self.envelope_samples_remaining = duration_samples;
    }
}

/// Technical implementation of the LookaheadAutomation structure.
pub struct LookaheadAutomation {
    lookahead_samples: u32,
    prediction_buffer: Vec<f32>,
    history: Vec<(u32, f32)>, // (sample_offset, value) pairs
}

impl LookaheadAutomation {
    /// Create lookahead processor with N samples of prediction
    pub fn new(lookahead_samples: u32) -> Self {
        Self {
            lookahead_samples,
            prediction_buffer: vec![0.0; lookahead_samples as usize],
            history: Vec::with_capacity(256),
        }
    }

    /// Predict parameter trajectory (simple linear regression)
    pub fn predict_trajectory(&mut self, recent_values: &[f32]) -> Vec<f32> {
        if recent_values.is_empty() {
            return self.prediction_buffer.clone();
        }

        let n = recent_values.len() as f32;
        let sum_x: f32 = (0..recent_values.len()).map(|i| i as f32).sum();
        let sum_y: f32 = recent_values.iter().sum();
        let sum_xy: f32 = (0..recent_values.len())
            .map(|i| (i as f32) * recent_values[i])
            .sum();
        let sum_x2: f32 = (0..recent_values.len()).map(|i| (i as f32).powi(2)).sum();

        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
        let intercept = (sum_y - slope * sum_x) / n;

        for i in 0..self.lookahead_samples as usize {
            let x = (recent_values.len() + i) as f32;
            self.prediction_buffer[i] = intercept + slope * x;
        }

        self.prediction_buffer.clone()
    }

    /// Smooth predicted trajectory with Kalman-like filtering
    pub fn smooth_trajectory(&self, trajectory: &[f32]) -> Vec<f32> {
        let mut smoothed = vec![0.0; trajectory.len()];
        let alpha = 0.3; // Smoothing factor

        if trajectory.is_empty() {
            return smoothed;
        }

        smoothed[0] = trajectory[0];
        for i in 1..trajectory.len() {
            smoothed[i] = alpha * trajectory[i] + (1.0 - alpha) * smoothed[i - 1];
        }

        smoothed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_parameter_ramp logic.
    fn test_parameter_ramp() {
        let mut ramp = ParameterRamp::new(0.0, 1.0, 100, AutomationCurve::Linear);
        let first = ramp.next();
        assert!(first > 0.0 && first < 1.0);
    }

    #[test]
    /// Technical implementation of the test_parameter_track logic.
    fn test_parameter_track() {
        let track = ParameterTrack::new("test", 0.0, 1.0, 0.5);
        track.set_immediate(0.75);
        assert!((track.get() - 0.75).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_polyphonic_modulation logic.
    fn test_polyphonic_modulation() {
        let mut mod_gen = PolyphonicModulation::new(0, 5.0, 0.1, 0.05);
        let modulated = mod_gen.modulate(0.5, 44100.0);
        assert!(modulated.is_finite());
    }

    #[test]
    /// Technical implementation of the test_lookahead_prediction logic.
    fn test_lookahead_prediction() {
        let mut lookahead = LookaheadAutomation::new(64);
        let recent = vec![0.0, 0.1, 0.2, 0.3];
        let predicted = lookahead.predict_trajectory(&recent);
        assert_eq!(predicted.len(), 64);
    }
}
