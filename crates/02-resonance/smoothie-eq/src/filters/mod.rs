/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5be429a5 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-eq/src/filters/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[derive(Clone, Copy, Debug)]
/// Technical implementation of the BiquadCoeffs structure.
pub struct BiquadCoeffs {
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    /// Feedback coefficient for y[n-1] (sign already negated for DF2T).
    pub a1: f64,
    /// Feedback coefficient for y[n-2] (sign already negated for DF2T).
    pub a2: f64,
}

impl BiquadCoeffs {
    /// Identity (unity gain, all-pass) coefficients.
    pub const IDENTITY: Self = Self {
        b0: 1.0,
        b1: 0.0,
        b2: 0.0,
        a1: 0.0,
        a2: 0.0,
    };

    /// Construct from the standard `[b0, b1, b2, 1, a1, a2]` form used in
    /// textbook representations by dividing all terms by `a0`.
    pub fn from_standard(b0: f64, b1: f64, b2: f64, a0: f64, a1: f64, a2: f64) -> Self {
        debug_assert!(a0.abs() > 1e-30, "BiquadCoeffs: a0 must be non-zero");
        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            // Negate for DF2T inner loop: y += -a1*s1 becomes y += a1_stored*s1
            a1: -a1 / a0,
            a2: -a2 / a0,
        }
    }
}

///
/// Must be one instance per channel × per filter stage.
#[derive(Clone, Copy, Debug, Default)]
/// Technical implementation of the BiquadState structure.
pub struct BiquadState {
    s1: f64,
    s2: f64,
}

impl BiquadState {
    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
    }
}

///
/// instances but shared `BiquadCoeffs`, guaranteeing perfect phase matching
///
///
/// `f32` buffers, biquad feedback loops accumulate significant rounding error
/// `f32`. The f64 intermediate values are silently downcast at the output boundary.
/// Technical implementation of the BiquadFilter structure.
pub struct BiquadFilter {
    pub coeffs: BiquadCoeffs,
    state_l: BiquadState,
    state_r: BiquadState,
}

impl Default for BiquadFilter {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::identity()
    }
}

impl BiquadFilter {
    /// Initializes a new instance of the associated type.
    pub fn new(coeffs: BiquadCoeffs) -> Self {
        Self {
            coeffs,
            state_l: BiquadState::default(),
            state_r: BiquadState::default(),
        }
    }

    /// Generates a numerical identity representation.
    pub fn identity() -> Self {
        Self::new(BiquadCoeffs::IDENTITY)
    }

    /// Technical implementation of the const_default logic.
    pub const fn const_default() -> Self {
        Self {
            coeffs: BiquadCoeffs::IDENTITY,
            state_l: BiquadState { s1: 0.0, s2: 0.0 },
            state_r: BiquadState { s1: 0.0, s2: 0.0 },
        }
    }

    /// Process a single sample (mono).
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process_single(&mut self, x: f32) -> f32 {
        self.process_left(x as f64) as f32
    }

    /// Update coefficients. Safe to call from the audio thread only when
    /// the non-realtime thread has finished writing the new `BiquadCoeffs`.
    /// Use an atomic swap pattern (e.g. `AtomicU64` coefficient cells) for
    /// true lock-free updates across threads.
    #[inline(always)]
    /// Technical implementation of the set_coeffs logic.
    pub fn set_coeffs(&mut self, coeffs: BiquadCoeffs) {
        self.coeffs = coeffs;
    }

    /// Process a single left-channel sample. Zero allocation, 5 MACs.
    ///
    /// The `#[inline(always)]` annotation is critical: it allows the compiler
    /// to hoist the coefficient loads into registers for the surrounding block loop,
    /// avoiding repeated memory access on each call.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process_left(&mut self, x: f64) -> f64 {
        let c = &self.coeffs;
        let y = c.b0 * x + self.state_l.s1;
        self.state_l.s1 = c.b1 * x + c.a1 * y + self.state_l.s2;
        self.state_l.s2 = c.b2 * x + c.a2 * y;
        y
    }

    /// Process a single right-channel sample.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process_right(&mut self, x: f64) -> f64 {
        let c = &self.coeffs;
        let y = c.b0 * x + self.state_r.s1;
        self.state_r.s1 = c.b1 * x + c.a1 * y + self.state_r.s2;
        self.state_r.s2 = c.b2 * x + c.a2 * y;
        y
    }

    /// Process a complete stereo block in-place.
    ///
    /// The coefficients are loaded into local variables once per call,
    /// preventing repeated cache loads across the length-N inner loops.
    pub fn process_block_stereo(&mut self, left: &mut [f32], right: &mut [f32]) {
        debug_assert_eq!(left.len(), right.len());

        // Load coeffs to stack — keeps them in L1 cache for the duration of the loop.
        let b0 = self.coeffs.b0;
        let b1 = self.coeffs.b1;
        let b2 = self.coeffs.b2;
        let a1 = self.coeffs.a1;
        let a2 = self.coeffs.a2;

        let (mut s1l, mut s2l) = (self.state_l.s1, self.state_l.s2);
        let (mut s1r, mut s2r) = (self.state_r.s1, self.state_r.s2);

        for (xl, xr) in left.iter_mut().zip(right.iter_mut()) {
            // Left channel — DF2T
            let x = *xl as f64;
            let y = b0 * x + s1l;
            s1l = b1 * x + a1 * y + s2l;
            s2l = b2 * x + a2 * y;
            *xl = y as f32;

            // Right channel — DF2T
            let x = *xr as f64;
            let y = b0 * x + s1r;
            s1r = b1 * x + a1 * y + s2r;
            s2r = b2 * x + a2 * y;
            *xr = y as f32;
        }

        // Write state back
        self.state_l.s1 = s1l;
        self.state_l.s2 = s2l;
        self.state_r.s1 = s1r;
        self.state_r.s2 = s2r;
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.state_l.reset();
        self.state_r.reset();
    }
}
