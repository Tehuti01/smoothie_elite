/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x235f1fda | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/reverb.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::{fdn::FdnOrder, AllPassDiffuser, EarlyReflections, FeedbackDelayNetwork, PreDelay};

/// Complete reverb parameter set.
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the ReverbParams structure.
pub struct ReverbParams {
    /// Room size scale [0.1, 4.0]. Scales all delay lengths linearly.
    pub size: f32,
    /// Reverb decay time in seconds [0.1, 60.0].
    pub rt60_s: f32,
    /// High-frequency damping [0.0, 1.0]. Higher = darker tail.
    pub damping: f32,
    /// Pre-delay in milliseconds [0, 500].
    pub predelay_ms: f32,
    /// Early / late blend [0.0 = early only, 1.0 = late only].
    pub early_late_mix: f32,
    /// Wet/dry mix [0.0 = dry, 1.0 = full wet].
    pub wet_dry: f32,
    /// Stereo width of the output [0.0 = mono, 1.0 = full stereo].
    pub width: f32,
}

impl Default for ReverbParams {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            size: 1.0,
            rt60_s: 2.0,
            damping: 0.5,
            predelay_ms: 20.0,
            early_late_mix: 0.5,
            wet_dry: 0.3,
            width: 1.0,
        }
    }
}

/// Technical implementation of the Reverb structure.
pub struct Reverb {
    params: ReverbParams,
    predelay: PreDelay,
    early: EarlyReflections,
    diffuser: AllPassDiffuser,
    fdn: FeedbackDelayNetwork,
    sample_rate: f32,
}

impl Reverb {
    /// Initializes a new instance of the associated type.
    pub fn new(params: ReverbParams, sample_rate: f32) -> Self {
        Self {
            predelay: PreDelay::new(500.0, sample_rate),
            early: EarlyReflections::with_default_taps(params.size, sample_rate),
            diffuser: AllPassDiffuser::new(sample_rate, params.size),
            fdn: FeedbackDelayNetwork::new(
                FdnOrder::N8,
                params.rt60_s,
                params.damping,
                params.size,
                sample_rate,
            ),
            params,
            sample_rate,
        }
    }

    /// Updates a framework parameter value.
    pub fn set_params(&mut self, params: ReverbParams) {
        self.params = params;
        self.predelay
            .set_delay_ms(params.predelay_ms, self.sample_rate);
        self.fdn.set_rt60(params.rt60_s);
        self.fdn.set_damping(params.damping);
    }

    /// Process one stereo sample through the full reverb chain.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let p = &self.params;

        // Pre-delay
        let (pd_l, pd_r) = self.predelay.process(in_l, in_r);

        // Early reflections
        let (er_l, er_r) = self.early.process(pd_l, pd_r);

        // All-pass diffusion
        let (diff_l, diff_r) = self.diffuser.process(pd_l, pd_r);

        // FDN late tail
        let (fdn_l, fdn_r) = self.fdn.process(diff_l, diff_r);

        // Blend early + late
        let mix = p.early_late_mix;
        let late_l = er_l * (1.0 - mix) + fdn_l * mix;
        let late_r = er_r * (1.0 - mix) + fdn_r * mix;

        // Stereo width: M/S processing
        let mid = (late_l + late_r) * 0.5;
        let side = (late_l - late_r) * 0.5 * p.width;
        let wet_l = mid + side;
        let wet_r = mid - side;

        // Wet/dry mix
        let wd = p.wet_dry;
        (
            in_l * (1.0 - wd) + wet_l * wd,
            in_r * (1.0 - wd) + wet_r * wd,
        )
    }

    /// Primary real-time signal processing execution block.
    pub fn process_block(&mut self, left: &mut [f32], right: &mut [f32]) {
        for (l, r) in left.iter_mut().zip(right.iter_mut()) {
            let (ol, or_) = self.process(*l, *r);
            *l = ol;
            *r = or_;
        }
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.predelay.reset();
        self.early.reset();
        self.diffuser.reset();
        self.fdn.reset();
    }
}
