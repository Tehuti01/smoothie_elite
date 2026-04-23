/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbd923863 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-dynamics/src/gain_computer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[repr(align(64))]
/// Technical implementation of the GainComputer structure.
pub struct GainComputer {
    /// Signal level at which compression begins (dBFS).
    pub threshold_db: f32,
    /// The compression slope ratio (e.g., 4.0 for 4:1).
    pub ratio: f32,
    /// Width of the soft-knee transition zone (dB).
    pub knee_db: f32,
    /// Post-computation makeup gain (dB).
    pub makeup_db: f32,
}

impl GainComputer {
    /// Initializes a new instance of the associated type.
    pub fn new(threshold_db: f32, ratio: f32, knee_db: f32, makeup_db: f32) -> Self {
        Self {
            threshold_db,
            ratio,
            knee_db,
            makeup_db,
        }
    }

    /// Compute gain reduction for an input level `x_db` (in dBFS).
    /// Returns the output gain reduction in **linear** scale (< 1.0 = attenuation).
    #[inline(always)]
    /// Technical implementation of the compute logic.
    pub fn compute(&self, x_db: f32) -> f32 {
        let gain_db = self.compute_db(x_db) + self.makeup_db;
        db_to_linear(gain_db)
    }

    /// Compute the gain reduction for `x_db`, returning the result in dB.
    #[inline(always)]
    /// Technical implementation of the compute_db logic.
    pub fn compute_db(&self, x_db: f32) -> f32 {
        let t = self.threshold_db;
        let r = self.ratio;
        let kw = self.knee_db;
        let half_kw = kw * 0.5;

        if kw <= 0.0 {
            // Hard knee
            if x_db <= t {
                0.0
            } else {
                (x_db - t) * (1.0 / r - 1.0)
            }
        } else {
            // Soft knee via quadratic interpolant
            let below = x_db <= t - half_kw;
            let above = x_db >= t + half_kw;

            if below {
                0.0
            } else if above {
                (x_db - t) * (1.0 / r - 1.0)
            } else {
                // Knee zone: smooth quadratic transition
                let kz = x_db - t + half_kw;
                kz * kz * (1.0 / r - 1.0) / (2.0 * kw)
            }
        }
    }
}

#[inline(always)]
/// Technical implementation of the db_to_linear logic.
fn db_to_linear(db: f32) -> f32 {
    // 10^(dB/20) via exp approximation
    smoothie_core::math::exp_approx(db * 0.115_129_255_f32) // ln(10)/20
}
