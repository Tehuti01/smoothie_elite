/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa1c74fde | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-reverb/src/algorithmic/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Schroeder, Moorer, and hybrid reverb algorithms.

use super::{EarlyReflections, FeedbackDelayNetwork, PreDelay, Reverb};

/// Technical implementation of the SchroederReverb structure.
pub struct SchroederReverb {
    allpasses: [AllPass; 4],
    comb_buffers: [[f32; 1600]; 8],
    comb_positions: [usize; 8],
    comb_gains: [f32; 8],
    comb_filtered: [f32; 8],
    sample_rate: f32,
    room_size: f32,
    damping: f32,
    wet_level: f32,
    dry_level: f32,
    width: f32,
}

struct AllPass {
    buffer: [f32; 1100],
    position: usize,
    feedback: f32,
}

impl AllPass {
    /// Initializes a new instance of the associated type.
    fn new(feedback: f32) -> Self {
        Self {
            buffer: [0.0; 1100],
            position: 0,
            feedback,
        }
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        let buf = self.buffer[self.position];
        let output = -input + buf;
        self.buffer[self.position] = input + buf * self.feedback;
        self.position = (self.position + 1) % 1100;
        output
    }

    /// Resets the internal state of the component.
    fn reset(&mut self) {
        self.buffer = [0.0; 1100];
        self.position = 0;
    }
}

impl SchroederReverb {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let comb_delays = [1557, 1617, 1491, 1422, 1277, 1356, 1188, 1116];
        let mut comb_gains = [0.0; 8];
        for (i, _delay) in comb_delays.iter().enumerate() {
            comb_gains[i] = 0.84;
        }

        Self {
            allpasses: [
                AllPass::new(0.7),
                AllPass::new(0.7),
                AllPass::new(0.7),
                AllPass::new(0.7),
            ],
            comb_buffers: [[0.0; 1600]; 8],
            comb_positions: [0; 8],
            comb_gains,
            comb_filtered: [0.0; 8],
            sample_rate,
            room_size: 0.5,
            damping: 0.5,
            wet_level: 0.33,
            dry_level: 0.7,
            width: 1.0,
        }
    }

    /// Technical implementation of the set_room_size logic.
    pub fn set_room_size(&mut self, size: f32) {
        self.room_size = size.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_damping logic.
    pub fn set_damping(&mut self, damp: f32) {
        self.damping = damp.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_wet logic.
    pub fn set_wet(&mut self, wet: f32) {
        self.wet_level = wet.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_dry logic.
    pub fn set_dry(&mut self, dry: f32) {
        self.dry_level = dry.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_width logic.
    pub fn set_width(&mut self, width: f32) {
        self.width = width.clamp(0.0, 1.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = input;

        for allpass in self.allpasses.iter_mut() {
            output = allpass.process(output);
        }

        let wet_mix = output * self.wet_level;
        let dry_mix = input * self.dry_level;

        let mut comb_output = 0.0;
        for i in 0..8 {
            let pos = self.comb_positions[i];
            let delay = match i {
                0 => 1557,
                1 => 1617,
                2 => 1491,
                3 => 1422,
                4 => 1277,
                5 => 1356,
                6 => 1188,
                7 => 1116,
                _ => 1557,
            };

            let read_pos = (pos + self.comb_buffers[0].len() - delay) % delay;
            let delayed = self.comb_buffers[i][read_pos];

            self.comb_filtered[i] =
                delayed * (1.0 - self.damping) + self.comb_filtered[i] * self.damping;

            self.comb_buffers[i][pos] = wet_mix + self.comb_filtered[i];
            self.comb_positions[i] = (pos + 1) % delay;

            comb_output += delayed;
        }

        let stereo = if self.width != 1.0 {
            let mid = comb_output;
            let side = comb_output * (self.width - 1.0);
            (mid + side, mid - side)
        } else {
            (comb_output, comb_output)
        };

        dry_mix + stereo.0
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: f32) -> (f32, f32) {
        let mut output = input;

        for allpass in self.allpasses.iter_mut() {
            output = allpass.process(output);
        }

        let wet_mix = output * self.wet_level;
        let dry_mix = input * self.dry_level;

        let mut comb_output = 0.0;
        let mut comb_out_r = 0.0;

        for i in 0..8 {
            let delay = match i {
                0 => 1557,
                1 => 1617,
                2 => 1491,
                3 => 1422,
                4 => 1277,
                5 => 1356,
                6 => 1188,
                7 => 1116,
                _ => 1557,
            };

            let pos = self.comb_positions[i];
            let read_pos = (pos + self.comb_buffers[0].len() - delay) % delay;
            let delayed = self.comb_buffers[i][read_pos];

            self.comb_filtered[i] =
                delayed * (1.0 - self.damping) + self.comb_filtered[i] * self.damping;

            self.comb_buffers[i][pos] = wet_mix + self.comb_filtered[i];
            self.comb_positions[i] = (pos + 1) % delay;

            if i < 4 {
                comb_output += delayed;
            } else {
                comb_out_r += delayed;
            }
        }

        let stereo = if self.width != 1.0 {
            let mid = comb_output;
            let side = comb_output * (self.width - 1.0);
            (mid + side, mid - side)
        } else {
            (comb_output, comb_out_r)
        };

        (dry_mix + stereo.0, dry_mix + stereo.1)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        for ap in self.allpasses.iter_mut() {
            ap.reset();
        }
        for buf in self.comb_buffers.iter_mut() {
            *buf = [0.0; 1600];
        }
        self.comb_positions = [0; 8];
        self.comb_filtered = [0.0; 8];
    }
}

/// Technical implementation of the MoorerReverb structure.
pub struct MoorerReverb {
    schroeder: SchroederReverb,
    diffusers: [Diffuser; 3],
    input_filter: SimpleBandpass,
}

struct Diffuser {
    buffer: [f32; 500],
    position: usize,
    feedback: f32,
}

impl Diffuser {
    /// Initializes a new instance of the associated type.
    fn new(feedback: f32) -> Self {
        Self {
            buffer: [0.0; 500],
            position: 0,
            feedback,
        }
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        let output = self.buffer[self.position];
        self.buffer[self.position] = input + output * self.feedback;
        self.position = (self.position + 1) % 500;
        output
    }
}

struct SimpleBandpass {
    low_state: f32,
    high_state: f32,
    low_coeff: f32,
    high_coeff: f32,
}

impl SimpleBandpass {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            low_state: 0.0,
            high_state: 0.0,
            low_coeff: 0.5,
            high_coeff: 0.5,
        }
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        self.low_state += (input - self.low_state) * self.low_coeff;
        self.high_state = input - self.high_state + self.high_state * self.high_coeff;
        self.low_state + self.high_state
    }
}

impl MoorerReverb {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            schroeder: SchroederReverb::new(sample_rate),
            diffusers: [Diffuser::new(0.7), Diffuser::new(0.7), Diffuser::new(0.7)],
            input_filter: SimpleBandpass::new(),
        }
    }

    /// Updates a framework parameter value.
    pub fn set_parameters(&mut self, room_size: f32, damping: f32, wet: f32, dry: f32) {
        self.schroeder.set_room_size(room_size);
        self.schroeder.set_damping(damping);
        self.schroeder.set_wet(wet);
        self.schroeder.set_dry(dry);
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, input: f32) -> f32 {
        let filtered = self.input_filter.process(input);
        let diffused = filtered;

        for diffuser in self.diffusers.iter_mut() {
            let _ = diffuser.process(diffused);
        }

        self.schroeder.process(input)
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: f32) -> (f32, f32) {
        self.schroeder.process_stereo(input)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.schroeder.reset();
    }
}

/// Technical implementation of the ShimmerReverb structure.
pub struct ShimmerReverb {
    base: SchroederReverb,
    pitch_shift: PitchShift,
}

struct PitchShift {
    input_buffer: [f32; 4096],
    output_buffer: [f32; 4096],
    write_pos: usize,
    read_pos: usize,
    pitch_ratio: f32,
}

impl PitchShift {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            input_buffer: [0.0; 4096],
            output_buffer: [0.0; 4096],
            write_pos: 0,
            read_pos: 0,
            pitch_ratio: 1.0,
        }
    }

    /// Technical implementation of the set_octave logic.
    fn set_octave(&mut self, semitones: f32) {
        self.pitch_ratio = 2.0_f32.powf(semitones / 12.0);
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, input: f32) -> f32 {
        self.input_buffer[self.write_pos] = input;
        self.write_pos = (self.write_pos + 1) & 4095;

        let read_index = self.read_pos as f32;
        let read_idx = read_index as usize;
        let frac = read_index - read_idx as f32;

        let s0 = self.input_buffer[read_idx & 4095];
        let s1 = self.input_buffer[(read_idx + 1) & 4095];

        let interp = s0 * (1.0 - frac) + s1 * frac;

        self.output_buffer[self.write_pos] = interp;

        self.read_pos = (self.read_pos as f32 + self.pitch_ratio) as usize;
        if self.read_pos >= 4096 {
            self.read_pos -= 4096;
        }

        interp
    }
}

impl ShimmerReverb {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            base: SchroederReverb::new(sample_rate),
            pitch_shift: PitchShift::new(),
        }
    }

    /// Technical implementation of the set_octaves logic.
    pub fn set_octaves(&mut self, octaves: i32) {
        self.pitch_shift.set_octave(octaves as f32 * 12.0);
    }

    /// Primary real-time signal processing execution block.
    pub fn process_stereo(&mut self, input: f32) -> (f32, f32) {
        let shifted = self.pitch_shift.process(input);
        let dry = self.base.process(input);
        let wet = self.base.process(shifted);
        (dry + wet * 0.5, dry + wet * 0.5)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.base.reset();
    }
}
