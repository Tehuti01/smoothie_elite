/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5a5d979f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/limiter/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// lookahead delay, programmable attack/release ballistics, and optional
///
///
/// 2. The peak level is computed over the lookahead window.
///    and ramped in over the attack time so that the peak exits at ceiling.
/// 4. Release ballistics decay the gain reduction smoothly after the peak.
use alloc::vec::Vec;

/// Limiter operating mode affecting the release ballistic character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the LimiterMode enumeration.
pub enum LimiterMode {
    /// Classical logarithmic release for transparent transient handling.
    Logarithmic,
    /// Clinical linear release for maximum signal density in mastering.
    Linear,
    /// Context-aware release that adapts timing based on signal transient density.
    Adaptive,
}

/// Technical implementation of the Limiter structure.
pub struct Limiter {
    /// The current operational release mode.
    mode: LimiterMode,
    /// The maximum signal ceiling (linear level).
    ceiling_linear: f32,
    /// Left channel lookahead delay buffer.
    delay_l: Vec<f32>,
    /// Right channel lookahead delay buffer.
    delay_r: Vec<f32>,
    /// Current write position in the circular buffers.
    delay_write: usize,
    /// Current read position in the circular buffers.
    delay_read: usize,
    /// Number of samples used for looking ahead (prediction window).
    lookahead_samples: usize,
    /// Calculated gain factor (0.0 to 1.0).
    gain: f32,
    /// Attack time constant coefficient.
    attack_coeff: f32,
    /// Release time constant coefficient.
    release_coeff: f32,
    /// State variable for adaptive release calculations.
    adaptive_release_state: f32,
}

impl Limiter {
    /// Initializes a new instance of the associated type.
    pub fn new(
        ceiling_db: f32,
        lookahead_ms: f32,
        attack_ms: f32,
        release_ms: f32,
        sample_rate: f32,
        mode: LimiterMode,
    ) -> Self {
        let lookahead_samples = ((lookahead_ms / 1000.0) * sample_rate) as usize;
        let buf_size = lookahead_samples + 1;

        let tau = |ms: f32| -> f32 {
            let tau_s = (ms / 1000.0) * sample_rate;
            if tau_s < 1.0 {
                1.0
            } else {
                1.0 - 1.0 / (tau_s + 1.0)
            }
        };

        Self {
            mode,
            ceiling_linear: db_to_linear(ceiling_db),
            delay_l: vec![0.0; buf_size],
            delay_r: vec![0.0; buf_size],
            delay_write: 0,
            delay_read: buf_size.saturating_sub(lookahead_samples),
            lookahead_samples,
            gain: 1.0,
            attack_coeff: tau(attack_ms),
            release_coeff: tau(release_ms),
            adaptive_release_state: 0.0,
        }
    }

    /// Process one stereo sample, returning the limited `(out_l, out_r)`.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let buf_size = self.delay_l.len();

        // Write new samples into delay line
        self.delay_l[self.delay_write] = in_l;
        self.delay_r[self.delay_write] = in_r;

        // Compute peak over entire lookahead window
        let mut peak = 0.0_f32;
        for i in 0..self.lookahead_samples {
            let idx = (self.delay_write + buf_size - i) % buf_size;
            peak = peak
                .max(self.delay_l[idx].abs())
                .max(self.delay_r[idx].abs());
        }

        // Determine required gain to bring peak to ceiling
        let target_gain = if peak > self.ceiling_linear {
            self.ceiling_linear / peak
        } else {
            1.0
        };

        // Apply gain ballistics
        if target_gain < self.gain {
            self.gain = self.gain * self.attack_coeff + target_gain * (1.0 - self.attack_coeff);
        } else {
            let release = match self.mode {
                LimiterMode::Logarithmic => self.release_coeff,
                LimiterMode::Linear => {
                    // Linear decay: fixed dB/sample decrement
                    let step = 0.0001;
                    self.gain = (self.gain + step).min(1.0);
                    return self.read_output();
                }
                LimiterMode::Adaptive => {
                    // Slow the release when signal is dense (high adaptive state)
                    let coeff_slow = self.release_coeff * 0.5;
                    let coeff_fast = self.release_coeff;
                    self.adaptive_release_state =
                        self.adaptive_release_state * 0.999 + (1.0 - self.gain) * 0.001;
                    if self.adaptive_release_state > 0.05 {
                        coeff_slow
                    } else {
                        coeff_fast
                    }
                }
            };
            self.gain = self.gain * release + 1.0 * (1.0 - release);
        }
        self.gain = self.gain.clamp(0.0, 1.0);

        // Advance pointers
        self.delay_write = (self.delay_write + 1) % buf_size;
        self.delay_read = (self.delay_read + 1) % buf_size;

        // Read from delay output
        let out_l = self.delay_l[self.delay_read] * self.gain;
        let out_r = self.delay_r[self.delay_read] * self.gain;
        (out_l, out_r)
    }

    /// Technical implementation of the read_output logic.
    fn read_output(&self) -> (f32, f32) {
        let out_l = self.delay_l[self.delay_read] * self.gain;
        let out_r = self.delay_r[self.delay_read] * self.gain;
        (out_l, out_r)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for s in self.delay_l.iter_mut() {
            *s = 0.0;
        }
        for s in self.delay_r.iter_mut() {
            *s = 0.0;
        }
        self.gain = 1.0;
        self.adaptive_release_state = 0.0;
    }
}

/// Technical implementation of the db_to_linear logic.
fn db_to_linear(db: f32) -> f32 {
    smoothie_core::math::exp_approx(db * 0.115_129_255_f32)
}
