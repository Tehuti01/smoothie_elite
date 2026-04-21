/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0b6bdaec | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/diffuser/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec;
///
/// energy into a dense, time-smeared reverb buildup. This is the critical
/// professional-sounding reverb tail.
/// The diffuser chain uses Schroeder's classic nested all-pass topology:
/// ```text
/// ```
/// Each `APF(delay, gain)` implements:
/// y[n] = -g·x[n] + x[n-d] + g·y[n-d]
/// which has unit magnitude response (`|H(e^jω)| = 1`) at all frequencies,
/// but non-uniform group delay — introducing the desired phase-scattering.
use alloc::vec::Vec;
use smoothie_core::math::FloatExt;

/// A single all-pass filter element using a circular delay buffer.
struct AllPassElement {
    buffer: Vec<f32>,
    delay_samples: usize,
    write_pos: usize,
    gain: f32,
}

impl AllPassElement {
    /// Initializes a new instance of the associated type.
    fn new(delay_samples: usize, gain: f32) -> Self {
        Self {
            buffer: vec![0.0; delay_samples + 1],
            delay_samples,
            write_pos: 0,
            gain,
        }
    }

    #[inline(always)]
    /// Primary real-time signal processing execution block.
    fn process(&mut self, x: f32) -> f32 {
        let len = self.buffer.len();
        let read_pos = (self.write_pos + len - self.delay_samples) % len;
        let delayed = self.buffer[read_pos];
        let output = -self.gain * x + delayed;
        self.buffer[self.write_pos] = x + self.gain * delayed;
        self.write_pos = (self.write_pos + 1) % len;
        output
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        for s in self.buffer.iter_mut() {
            *s = 0.0;
        }
        self.write_pos = 0;
    }
}

/// mutually prime delay lengths to maximise diffusion without resonance.
/// Technical implementation of the AllPassDiffuser structure.
pub struct AllPassDiffuser {
    stages_l: [AllPassElement; 4],
    stages_r: [AllPassElement; 4],
}

impl AllPassDiffuser {
    /// Construct a diffuser at the given sample rate and size scale [0.5, 2.0].
    pub fn new(sample_rate: f32, size: f32) -> Self {
        let sr_scale = sample_rate / 44100.0 * size;
        let delays = [
            (142.0 * sr_scale) as usize,
            (107.0 * sr_scale) as usize,
            (379.0 * sr_scale) as usize,
            (277.0 * sr_scale) as usize,
        ];
        let gains = [0.75, 0.70, 0.625, 0.625_f32];

        Self {
            stages_l: [
                AllPassElement::new(delays[0].max(2), gains[0]),
                AllPassElement::new(delays[1].max(2), gains[1]),
                AllPassElement::new(delays[2].max(2), gains[2]),
                AllPassElement::new(delays[3].max(2), gains[3]),
            ],
            stages_r: [
                AllPassElement::new(delays[0].max(2) + 7, gains[0]),
                AllPassElement::new(delays[1].max(2) + 5, gains[1]),
                AllPassElement::new(delays[2].max(2) + 11, gains[2]),
                AllPassElement::new(delays[3].max(2) + 3, gains[3]),
            ],
        }
    }

    /// Process one stereo sample through all 4 stages.
    #[inline(always)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, in_l: f32, in_r: f32) -> (f32, f32) {
        let mut l = in_l;
        let mut r = in_r;
        for i in 0..4 {
            l = self.stages_l[i].process(l);
            r = self.stages_r[i].process(r);
        }
        (l, r)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for s in self.stages_l.iter_mut() {
            s.reset();
        }
        for s in self.stages_r.iter_mut() {
            s.reset();
        }
    }
}
