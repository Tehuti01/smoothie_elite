//! Frequency shifter using Hilbert transform approximation (single-sideband AM).
//! Single-sideband modulation shifts all frequencies by a fixed Hz amount.

use smoothie_math::trig::Phasor;

/// Single-sideband frequency shifter.
/// Uses a 12-stage allpass Hilbert pair approximation.
pub struct FreqShifter {
    // Hilbert allpass pairs (in-phase and quadrature)
    ap_i: [f32; 12],
    ap_q: [f32; 12],
    coeffs_i: [f32; 6],
    coeffs_q: [f32; 6],
    carrier:  Phasor,
    /// Shift amount in Hz (can be negative for downward shift).
    pub shift_hz: f32,
    /// Wet/dry [0, 1].
    pub mix: f32,
    sample_rate: f32,
}

impl FreqShifter {
    /// Allpass coefficients for wideband 90° phase split (Regalia-Mitra).
    const COEFFS_I: [f32; 6] = [0.6923878, 0.9360654, 0.9882295, 0.9987488, 0.9998744, 0.9999896];
    const COEFFS_Q: [f32; 6] = [0.4021921, 0.8561862, 0.9722678, 0.9952884, 0.9993776, 0.9999380];

    pub fn new(sample_rate: f32) -> Self {
        Self {
            ap_i:    [0.0; 12],
            ap_q:    [0.0; 12],
            coeffs_i: Self::COEFFS_I,
            coeffs_q: Self::COEFFS_Q,
            carrier:  Phasor::new(0.0, sample_rate),
            shift_hz: 100.0,
            mix:      1.0,
            sample_rate,
        }
    }

    pub fn set_shift(&mut self, hz: f32) {
        self.shift_hz = hz;
        self.carrier.set_freq(hz.abs(), self.sample_rate);
    }

    #[inline]
    fn allpass_6stage(x: f32, state: &mut [f32; 12], coeffs: &[f32; 6], offset: usize) -> f32 {
        let mut y = x;
        for k in 0..6 {
            let idx = offset + k;
            let a = coeffs[k];
            let tmp = -a * y + state[idx];
            state[idx] = a * tmp + y;
            y = tmp;
        }
        y
    }

    #[inline]
    pub fn process(&mut self, x: f32) -> f32 {
        // Hilbert pair: I (in-phase) and Q (90° phase shift)
        let i = Self::allpass_6stage(x, &mut self.ap_i, &self.coeffs_i, 0);
        // Second set uses state slots 6..12 from ap_q re-used as ap_i[6..12]
        let mut state_q = self.ap_q;
        let q = Self::allpass_6stage(x, &mut state_q, &self.coeffs_q, 0);
        self.ap_q = state_q;

        // Carrier sinusoid pair
        let phase = self.carrier.tick();
        let tau = std::f32::consts::TAU;
        let cos_c = (phase * tau).cos();
        let sin_c = (phase * tau).sin();

        // SSB modulation: up-shift or down-shift
        let wet = if self.shift_hz >= 0.0 {
            i * cos_c - q * sin_c
        } else {
            i * cos_c + q * sin_c
        };

        x * (1.0 - self.mix) + wet * self.mix
    }
}
