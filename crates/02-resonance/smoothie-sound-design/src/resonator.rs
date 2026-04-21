/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xe458c87b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/resonator.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Modal resonator for physical modeling synthesis.

#[repr(align(64))]
/// Technical implementation of the Resonator structure.
pub struct Resonator {
    pub modes: [ResonanceMode; 16],
    pub mode_count: usize,
    pub decay: f32,
    pub brightness: f32,
}

#[derive(Clone, Copy)]
#[repr(align(64))]
/// Technical implementation of the ResonanceMode structure.
pub struct ResonanceMode {
    pub frequency: f32,
    pub bandwidth: f32,
    pub amplitude: f32,
    pub phase: f32,
    pub envelope: f32,
}

impl Resonator {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            modes: [ResonanceMode::default(); 16],
            mode_count: 0,
            decay: 0.5,
            brightness: 1.0,
        }
    }

    /// Performs vector addition logic.
    pub fn add_mode(&mut self, freq: f32, bw: f32, amp: f32) {
        if self.mode_count < 16 {
            self.modes[self.mode_count] = ResonanceMode::new(freq, bw, amp);
            self.mode_count += 1;
        }
    }

    /// Performs vector addition logic.
    pub fn add_frequencies(&mut self, base_freq: f32, num_overtones: usize) {
        for i in 1..=num_overtones.min(16) {
            let ratio = i as f32;
            let freq = base_freq * ratio;
            let bw = base_freq * 0.01 / ratio;
            let amp = 1.0 / ratio;
            self.add_mode(freq, bw, amp);
        }
    }

    /// Technical implementation of the set_decay logic.
    pub fn set_decay(&mut self, decay: f32) {
        self.decay = decay.clamp(0.0, 1.0);
    }

    /// Technical implementation of the trigger logic.
    pub fn trigger(&mut self) {
        for i in 0..self.mode_count {
            self.modes[i].envelope = self.modes[i].amplitude;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self, input: f32) -> f32 {
        let mut output = 0.0;

        for i in 0..self.mode_count {
            let mode = &mut self.modes[i];
            let omega = 2.0 * core::f32::consts::PI * mode.frequency;
            let sample = (omega * mode.phase).sin() * mode.envelope;

            output += sample;

            mode.envelope *= (1.0 - self.decay * 0.001);
            mode.phase += 0.0001;
        }

        output * input * self.brightness
    }

    /// Technical implementation of the impulse logic.
    pub fn impulse(&mut self) -> f32 {
        let mut output = 0.0;

        for i in 0..self.mode_count {
            output += self.modes[i].envelope;
            self.modes[i].phase = 0.0;
        }

        output
    }
}

impl ResonanceMode {
    /// Technical implementation of the default logic.
    pub const fn default() -> Self {
        Self {
            frequency: 440.0,
            bandwidth: 10.0,
            amplitude: 1.0,
            phase: 0.0,
            envelope: 0.0,
        }
    }

    /// Initializes a new instance of the associated type.
    pub const fn new(frequency: f32, bandwidth: f32, amplitude: f32) -> Self {
        Self {
            frequency,
            bandwidth,
            amplitude,
            phase: 0.0,
            envelope: 0.0,
        }
    }
}

impl Default for Resonator {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}
