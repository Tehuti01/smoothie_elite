/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4315e266 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/fdn/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
///
///
/// ```text
///        [1 -1]                               [H_2 -H_2]
/// Normalised by 1/√N, this is unitary: `H_N · H_N^T = I`.
/// Using a unitary feedback matrix guarantees:
/// 2. **No modal coloration**: all N delay lines contribute equally.
///
///
/// the delay length in samples and RT60 is the reverberation time.
use alloc::vec::Vec;

/// Supported FDN orders (number of delay lines).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the FdnOrder enumeration.
pub enum FdnOrder {
    N4 = 4,
    N8 = 8,
    N16 = 16,
}

impl FdnOrder {
    /// Technical implementation of the n logic.
    pub fn n(&self) -> usize {
        *self as usize
    }
}

/// Models frequency-dependent air absorption and surface damping.
#[derive(Clone, Copy, Default, Debug)]
struct AbsorbFilter {
    state: f32,
    coeff: f32,
}

impl AbsorbFilter {
    /// Damping coefficient computed from `rt60` target at high frequencies.
    fn new(damping: f32) -> Self {
        Self {
            state: 0.0,
            coeff: damping.clamp(0.0, 0.9999),
        }
    }

    #[inline(always)]
    /// Primary real-time signal processing execution block.
    fn process(&mut self, x: f32) -> f32 {
        self.state = self.state * self.coeff + x * (1.0 - self.coeff);
        self.state
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.state = 0.0;
    }
}

/// avoid common factors — preventing frequency-domain resonances.
/// Values are scaled proportionally when the configured `size` parameter or
/// `sample_rate` differs from 44100 Hz.
const PRIME_DELAYS_44100: [u32; 16] = [
    1423, 1777, 2053, 2381, 2731, 3079, 3457, 3823, 4219, 4591, 4987, 5381, 5783, 6199, 6619, 7057,
];

/// Technical implementation of the FeedbackDelayNetwork structure.
pub struct FeedbackDelayNetwork {
    order: FdnOrder,
    /// Circular delay buffers, one per channel.
    delays: Vec<Vec<f32>>,
    /// Write/read position per delay line.
    write_pos: Vec<usize>,
    delay_lengths: Vec<usize>,
    /// Absorptive one-pole damping filter per delay tap.
    absorb: Vec<AbsorbFilter>,
    /// Per-line gain coefficients encoding RT60.
    gains: Vec<f32>,
    /// FDN state vector (size = order).
    state: Vec<f32>,
    rt60: f32,
    _damping: f32,
    _size_scale: f32,
    sample_rate: f32,
}

impl FeedbackDelayNetwork {
    /// Initializes a new instance of the associated type.
    pub fn new(
        order: FdnOrder,
        rt60: f32,
        damping: f32,
        size_scale: f32,
        sample_rate: f32,
    ) -> Self {
        let n = order.n();
        let sr_ratio = sample_rate / 44100.0;

        let delay_lengths: Vec<usize> = PRIME_DELAYS_44100
            .iter()
            .take(n)
            .map(|&d| ((d as f32 * sr_ratio * size_scale) as usize).max(2))
            .collect();

        let delays: Vec<Vec<f32>> = delay_lengths.iter().map(|&len| vec![0.0f32; len]).collect();

        let write_pos = vec![0usize; n];

        let gains: Vec<f32> = delay_lengths
            .iter()
            .map(|&d| compute_gain(d as f32, rt60, sample_rate))
            .collect();

        let absorb = vec![AbsorbFilter::new(damping); n];
        let state = vec![0.0f32; n];

        Self {
            order,
            delays,
            write_pos,
            delay_lengths,
            absorb,
            gains,
            state,
            rt60,
            _damping: damping,
            _size_scale: size_scale,
            sample_rate,
        }
    }

    /// Recalculate gains for new RT60 value.
    pub fn set_rt60(&mut self, rt60: f32) {
        self.rt60 = rt60;
        for (i, &d) in self.delay_lengths.iter().enumerate() {
            self.gains[i] = compute_gain(d as f32, rt60, self.sample_rate);
        }
    }

    /// Update damping coefficient for all absorb filters.
    pub fn set_damping(&mut self, damping: f32) {
        for f in self.absorb.iter_mut() {
            *f = AbsorbFilter::new(damping);
        }
    }

    /// Process one stereo pair through the FDN, returning the decorrelated output pair.
    ///
    /// Input is mixed into the FDN as a mono sum. Outputs are taken from
    /// the first and last delay lines for stereo decorrelation.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input_l: f32, input_r: f32) -> (f32, f32) {
        let n = self.order.n();
        let input_mono = (input_l + input_r) * 0.5;

        // Read current state from each delay line
        for i in 0..n {
            let read_pos = self.write_pos[i]
                .wrapping_sub(self.delay_lengths[i])
                .wrapping_add(self.delays[i].len())
                % self.delays[i].len();
            self.state[i] = self.delays[i][read_pos];
        }

        // Apply Hadamard mixing matrix to state vector
        let mixed = hadamard_mix(&self.state, n);

        // Write back through absorb + gain, injecting input
        for i in 0..n {
            let x = mixed[i] * self.gains[i] + input_mono;
            let damped = self.absorb[i].process(x);
            self.delays[i][self.write_pos[i]] = damped;
            self.write_pos[i] = (self.write_pos[i] + 1) % self.delays[i].len();
        }

        // Stereo output: odd/even decorrelation
        let out_l = self.state.iter().step_by(2).sum::<f32>() / (n / 2) as f32;
        let out_r = self.state.iter().skip(1).step_by(2).sum::<f32>() / (n / 2) as f32;
        (out_l, out_r)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for d in self.delays.iter_mut() {
            for s in d.iter_mut() {
                *s = 0.0;
            }
        }
        for w in self.write_pos.iter_mut() {
            *w = 0;
        }
        for a in self.absorb.iter_mut() {
            a.reset();
        }
        for s in self.state.iter_mut() {
            *s = 0.0;
        }
    }
}

/// `g = 10^(-3 · delay_samples / (RT60 · sample_rate))`
fn compute_gain(delay_samples: f32, rt60: f32, sample_rate: f32) -> f32 {
    if rt60 < 1e-4 {
        return 0.0;
    }
    let exponent = -3.0 * delay_samples / (rt60 * sample_rate);
    smoothie_core::math::exp_approx(exponent * 2.302585_f32) // log(10) ≈ 2.302585
}

/// In-place Hadamard butterfly mixing for vectors of size N (must be power of 2).
fn hadamard_mix(v: &[f32], n: usize) -> Vec<f32> {
    let mut out = v.to_vec();
    let mut step = 1;
    while step < n {
        let mut i = 0;
        while i < n {
            for j in i..i + step {
                let a = out[j];
                let b = out[j + step];
                out[j] = (a + b) * core::f32::consts::FRAC_1_SQRT_2;
                out[j + step] = (a - b) * core::f32::consts::FRAC_1_SQRT_2;
            }
            i += step * 2;
        }
        step <<= 1;
    }
    out
}
