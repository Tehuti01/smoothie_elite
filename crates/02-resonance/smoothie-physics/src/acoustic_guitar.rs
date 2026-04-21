/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5755f01e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_guitar.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// and plucking excitation.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticGuitar structure.
pub struct AcousticGuitar {
    strings: Vec<GuitarString>,
    body: GuitarBody,
    sample_rate: f32,
    output: f32,
}

struct GuitarString {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    decay: f32,
    position: f32,
    is_nylon: bool,
}

struct GuitarBody {
    modes: Vec<BodyMode>,
    soundhole_radiation: f32,
    bridge_yield: f32,
    top_displacement: f32,
    back_displacement: f32,
}

struct BodyMode {
    frequency: f32,
    damping: f32,
    amplitude: f32,
    is_ring: bool,
}

impl AcousticGuitar {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let strings = vec![
            GuitarString {
                frequency: 329.63,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.999,
                position: 0.0,
                is_nylon: false,
            },
            GuitarString {
                frequency: 246.94,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9992,
                position: 0.0,
                is_nylon: false,
            },
            GuitarString {
                frequency: 196.00,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9993,
                position: 0.0,
                is_nylon: false,
            },
            GuitarString {
                frequency: 146.83,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9994,
                position: 0.0,
                is_nylon: false,
            },
            GuitarString {
                frequency: 110.00,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                is_nylon: false,
            },
            GuitarString {
                frequency: 82.41,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9996,
                position: 0.0,
                is_nylon: false,
            },
        ];

        let modes = vec![
            BodyMode {
                frequency: 180.0,
                damping: 0.96,
                amplitude: 0.0,
                is_ring: true,
            },
            BodyMode {
                frequency: 280.0,
                damping: 0.965,
                amplitude: 0.0,
                is_ring: true,
            },
            BodyMode {
                frequency: 420.0,
                damping: 0.95,
                amplitude: 0.0,
                is_ring: false,
            },
            BodyMode {
                frequency: 540.0,
                damping: 0.94,
                amplitude: 0.0,
                is_ring: false,
            },
            BodyMode {
                frequency: 680.0,
                damping: 0.93,
                amplitude: 0.0,
                is_ring: false,
            },
            BodyMode {
                frequency: 820.0,
                damping: 0.92,
                amplitude: 0.0,
                is_ring: false,
            },
            BodyMode {
                frequency: 950.0,
                damping: 0.91,
                amplitude: 0.0,
                is_ring: false,
            },
            BodyMode {
                frequency: 1100.0,
                damping: 0.90,
                amplitude: 0.0,
                is_ring: false,
            },
        ];

        Self {
            strings,
            body: GuitarBody {
                modes,
                soundhole_radiation: 0.5,
                bridge_yield: 0.3,
                top_displacement: 0.0,
                back_displacement: 0.0,
            },
            sample_rate,
            output: 0.0,
        }
    }

    /// Technical implementation of the pluck logic.
    pub fn pluck(&mut self, string: usize, position: f32, velocity: f32) {
        if string < self.strings.len() {
            let s = &mut self.strings[string];
            s.amplitude = velocity.clamp(0.0, 1.0);
            s.position = position.clamp(0.0, 1.0);
            s.phase = 0.0;

            let pickup = 1.0 - (position - 0.5).abs();
            let mut body_excite = velocity * pickup * 0.3;

            for mode in &mut self.body.modes {
                mode.amplitude += body_excite * (mode.frequency / 500.0).min(1.0);
            }
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let mut string_out = 0.0;
        let mut bridge_force = 0.0;

        for string in &mut self.strings {
            if string.amplitude > 0.0001 {
                let harmonic_factor = if string.is_nylon { 1.0 } else { 1.5 };
                let k = (2.0 * PI * string.frequency * dt) % (2.0 * PI);

                string.phase += k;
                if string.phase > 2.0 * PI {
                    string.phase -= 2.0 * PI;
                }

                let envelope = string.amplitude * (string.decay.powf(1000.0 * dt));
                let pluck_shape = 1.0 - string.position;

                let sample = envelope * (string.phase * harmonic_factor).sin() * pluck_shape;
                string_out += sample;

                bridge_force += sample * self.body.bridge_yield;
                string.amplitude *= string.decay;
            }
        }

        let mut body_out = 0.0;
        for mode in &mut self.body.modes {
            mode.amplitude *= mode.damping;
            let osc = (mode.frequency * 2.0 * PI * dt).sin();
            body_out += mode.amplitude * osc * if mode.is_ring { 0.5 } else { 1.0 };
            mode.amplitude *= 0.9999;
        }

        let soundhole = body_out * self.body.soundhole_radiation;
        let body_vibration = body_out * (1.0 - self.body.soundhole_radiation);

        self.body.top_displacement = body_vibration;
        self.body.back_displacement = body_vibration * 0.3;

        self.output = string_out * 0.5 + body_out * 0.35 + soundhole * 0.15;
        self.output += bridge_force * 0.1;
        self.output *= 0.8;

        self.output
    }

    /// Technical implementation of the set_tone_knob logic.
    pub fn set_tone_knob(&mut self, tone: f32) {
        self.body.soundhole_radiation = tone.clamp(0.1, 0.9);
    }

    /// Technical implementation of the set_body_size logic.
    pub fn set_body_size(&mut self, size: f32) {
        for mode in &mut self.body.modes {
            mode.frequency *= size.clamp(0.8, 1.2);
        }
    }

    /// Technical implementation of the set_string_type logic.
    pub fn set_string_type(&mut self, string: usize, is_nylon: bool) {
        if string < self.strings.len() {
            self.strings[string].is_nylon = is_nylon;
            self.strings[string].decay = if is_nylon { 0.999 } else { 0.9995 };
        }
    }

    /// Technical implementation of the get_output logic.
    pub fn get_output(&self) -> f32 {
        self.output
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl Default for AcousticGuitar {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
