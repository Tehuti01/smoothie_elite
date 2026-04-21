/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0f4a027a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_bowed.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// and string-body coupling.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticBowed structure.
pub struct AcousticBowed {
    strings: Vec<BowedString>,
    body: BowedBody,
    output: f32,
    sample_rate: f32,
}

struct BowedString {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    decay: f32,
    position: f32,
    bowing: bool,
    bow_pos: f32,
    bow_vel: f32,
    bow_pressure: f32,
}

struct BowedBody {
    modes: Vec<BodyMode>,
    radiation: f32,
}

struct BodyMode {
    frequency: f32,
    damping: f32,
    amplitude: f32,
}

impl AcousticBowed {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let strings = vec![
            BowedString {
                frequency: 110.0,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
            BowedString {
                frequency: 146.83,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
            BowedString {
                frequency: 196.0,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
            BowedString {
                frequency: 261.63,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
            BowedString {
                frequency: 329.63,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
            BowedString {
                frequency: 392.0,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.9995,
                position: 0.0,
                bowing: false,
                bow_pos: 0.2,
                bow_vel: 0.0,
                bow_pressure: 0.5,
            },
        ];

        let modes = vec![
            BodyMode {
                frequency: 180.0,
                damping: 0.97,
                amplitude: 0.0,
            },
            BodyMode {
                frequency: 320.0,
                damping: 0.96,
                amplitude: 0.0,
            },
            BodyMode {
                frequency: 480.0,
                damping: 0.94,
                amplitude: 0.0,
            },
            BodyMode {
                frequency: 640.0,
                damping: 0.92,
                amplitude: 0.0,
            },
            BodyMode {
                frequency: 820.0,
                damping: 0.90,
                amplitude: 0.0,
            },
        ];

        Self {
            strings,
            body: BowedBody {
                modes,
                radiation: 0.6,
            },
            output: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the friction_model logic.
    fn friction_model(relative_vel: f32, pressure: f32, rosin: f32) -> f32 {
        let static_fric = 0.4 * rosin;
        let dynamic_fric = 0.2 * rosin;
        let velocity_threshold = 0.01;

        if relative_vel.abs() < velocity_threshold {
            static_fric * pressure
        } else {
            dynamic_fric * pressure * (1.0 + velocity_threshold / relative_vel.abs())
        }
    }

    /// Technical implementation of the bow logic.
    pub fn bow(&mut self, string: usize, position: f32, velocity: f32, pressure: f32) {
        if string < self.strings.len() {
            let s = &mut self.strings[string];
            s.bowing = true;
            s.bow_pos = position.clamp(0.05, 0.5);
            s.bow_vel = velocity.clamp(-1.0, 1.0);
            s.bow_pressure = pressure.clamp(0.0, 1.0);
        }
    }

    /// Technical implementation of the release logic.
    pub fn release(&mut self, string: usize) {
        if string < self.strings.len() {
            self.strings[string].bowing = false;
        }
    }

    /// Technical implementation of the stop_bowing logic.
    pub fn stop_bowing(&mut self) {
        for s in &mut self.strings {
            s.bowing = false;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_string(&mut self, string: &mut BowedString) -> f32 {
        let dt = 1.0 / self.sample_rate;

        if string.bowing {
            let relative_vel = string.bow_vel - string.amplitude * string.frequency;
            let friction = Self::friction_model(relative_vel, string.bow_pressure, 0.7);

            string.amplitude += friction * string.bow_vel.abs() * 0.1;
            string.amplitude = string.amplitude.min(1.0);
        }

        if string.amplitude > 0.0001 {
            let k = 2.0 * PI * string.frequency * dt;
            string.phase += k;

            while string.phase > 2.0 * PI {
                string.phase -= 2.0 * PI;
            }

            let envelope = string.amplitude * string.decay.powf(1000.0 * dt);
            let sample = envelope * string.phase.sin();

            string.amplitude *= string.decay;

            sample
        } else {
            0.0
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let mut string_out = 0.0;

        for string in &mut self.strings {
            let sample = self.process_string(string);
            string_out += sample;

            if string.bowing {
                for mode in &mut self.body.modes {
                    mode.amplitude += sample * 0.02;
                }
            }
        }

        let mut body_out = 0.0;
        for mode in &mut self.body.modes {
            mode.amplitude *= mode.damping;
            body_out += mode.amplitude * (mode.frequency * 2.0 * PI * dt).sin();
            mode.amplitude *= 0.999;
        }

        let radiation = body_out * self.body.radiation;

        self.output = string_out * 0.5 + body_out * 0.35 + radiation * 0.15;
        self.output *= 0.7;

        self.output
    }

    /// Technical implementation of the set_string_frequency logic.
    pub fn set_string_frequency(&mut self, string: usize, freq: f32) {
        if string < self.strings.len() {
            self.strings[string].frequency = freq.clamp(20.0, 5000.0);
        }
    }

    /// Technical implementation of the set_bow_pressure logic.
    pub fn set_bow_pressure(&mut self, string: usize, pressure: f32) {
        if string < self.strings.len() {
            self.strings[string].bow_pressure = pressure.clamp(0.0, 1.0);
        }
    }

    /// Technical implementation of the set_bow_position logic.
    pub fn set_bow_position(&mut self, string: usize, position: f32) {
        if string < self.strings.len() {
            self.strings[string].bow_pos = position.clamp(0.05, 0.5);
        }
    }

    /// Technical implementation of the set_rosin logic.
    pub fn set_rosin(&mut self, level: f32) {}

    /// Technical implementation of the get_output logic.
    pub fn get_output(&self) -> f32 {
        self.output
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl Default for AcousticBowed {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
