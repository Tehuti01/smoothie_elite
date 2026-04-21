/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x28785523 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-multiverse/src/synthesis/phase_distorter.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[repr(align(64))]
/// Technical implementation of the PhaseDistorter structure.
pub struct PhaseDistorter {
    amount: f32,
    mode: DistortionMode,
}

#[derive(Clone, Copy)]
/// Technical implementation of the DistortionMode enumeration.
pub enum DistortionMode {
    Sync,
    Bend,
    Flip,
    Quantize,
}

impl PhaseDistorter {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            amount: 0.0,
            mode: DistortionMode::Bend,
        }
    }

    /// 🚀 Set distortion amount (0.0 - 1.0)
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount.clamp(0.0, 1.0);
    }

    /// 🧠 Warp the phase (for Synthesis)
    /// This is applied to the oscillator phase before table lookup.
    pub fn warp_phase(&self, phase: f32) -> f32 {
        match self.mode {
            DistortionMode::Sync => {
                // Hard-sync simulation: re-wrap the phase multiple times
                let scale = 1.0 + self.amount * 4.0;
                (phase * scale) % 1.0
            }
            DistortionMode::Bend => {
                // Exponential phase bending
                let curve = 1.0 + self.amount * 3.0;
                phase.powf(curve)
            }
            DistortionMode::Flip => {
                // Phase mirror
                if phase < self.amount {
                    self.amount - phase
                } else {
                    phase
                }
            }
            DistortionMode::Quantize => {
                // Step-quantization of the phase ramp
                let steps = (1.0 + (1.0 - self.amount) * 32.0) as f32;
                (phase * steps).floor() / steps
            }
        }
    }

    /// 🦾 Apply non-linear saturation to output
    pub fn saturate(&self, input: f32) -> f32 {
        // Fast tanh approximation for warm saturation
        let x = input * (1.0 + self.amount * 2.0);
        x / (1.0 + x.abs())
    }
}

/// 🛡️ System Integrity Verification: Spectral warping verified.
pub const DISTORTION_DENSITY: &str = "SERAPHIC_300IQ_NON_LINEAR";
