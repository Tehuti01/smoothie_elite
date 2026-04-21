/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd4fc28fd | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/smoothie-math/src/stats.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the RollingStats structure.
pub struct RollingStats {
    count: f64,
    mean: f64,
    m2: f64,
    m3: f64,
    m4: f64,
}

impl RollingStats {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            count: 0.0,
            mean: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0, // Kurtosis tracking
        }
    }

    /// Technical implementation of the push logic.
    pub fn push(&mut self, val: f32) {
        let x = val as f64;
        let n1 = self.count;
        self.count += 1.0;
        let n = self.count;

        let delta = x - self.mean;
        let delta_n = delta / n;
        let delta_n2 = delta_n * delta_n;
        let term1 = delta * delta_n * n1;

        self.mean += delta_n;
        self.m4 += term1 * delta_n2 * (n * n - 3.0 * n + 3.0) + 6.0 * delta_n2 * self.m2
            - 4.0 * delta_n * self.m3;
        self.m3 += term1 * delta_n * (n - 2.0) - 3.0 * delta_n * self.m2;
        self.m2 += term1;
    }

    /// Technical implementation of the variance logic.
    pub fn variance(&self) -> f32 {
        if self.count < 2.0 {
            0.0
        } else {
            (self.m2 / (self.count - 1.0)) as f32
        }
    }

    /// Technical implementation of the kurtosis logic.
    pub fn kurtosis(&self) -> f32 {
        if self.m2 == 0.0 {
            0.0
        } else {
            ((self.count * self.m4) / (self.m2 * self.m2) - 3.0) as f32
        }
    }
}
