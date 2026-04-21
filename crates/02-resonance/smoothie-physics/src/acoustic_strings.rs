/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x1f730ad7 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_strings.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Violin body modes, bow friction, bridge transmission, and string vibration.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticStrings structure.
pub struct AcousticStrings {
    strings: Vec<ViolinString>,
    body: ViolinBody,
    bridge: Bridge,
    bow: Bow,
    output: f32,
}

struct ViolinString {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    decay: f32,
    finger_position: f32,
    stopped: bool,
}

struct ViolinBody {
    modes: Vec<BodyMode>,
    top_amp: f32,
    back_amp: f32,
    f_hole_amp: f32,
}

struct BodyMode {
    frequency: f32,
    damping: f32,
    amplitude: f32,
    type_: ModeType,
}

enum ModeType {
    Top,
    Back,
    Air,
}

struct Bridge {
    transmission: f32,
    height: f32,
    mass: f32,
    position: f32,
}

struct Bow {
    position: f32,
    velocity: f32,
    pressure: f32,
    rosin: f32,
}

impl AcousticStrings {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let strings = vec![
            ViolinString {
                frequency: 196.0,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.999,
                finger_position: 0.0,
                stopped: false,
            },
            ViolinString {
                frequency: 293.66,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.999,
                finger_position: 0.0,
                stopped: false,
            },
            ViolinString {
                frequency: 392.0,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.999,
                finger_position: 0.0,
                stopped: false,
            },
            ViolinString {
                frequency: 523.25,
                amplitude: 0.0,
                phase: 0.0,
                decay: 0.999,
                finger_position: 0.0,
                stopped: false,
            },
        ];

        let modes = vec![
            BodyMode {
                frequency: 280.0,
                damping: 0.96,
                amplitude: 0.0,
                type_: ModeType::Top,
            },
            BodyMode {
                frequency: 470.0,
                damping: 0.94,
                amplitude: 0.0,
                type_: ModeType::Top,
            },
            BodyMode {
                frequency: 560.0,
                damping: 0.93,
                amplitude: 0.0,
                type_: ModeType::Back,
            },
            BodyMode {
                frequency: 720.0,
                damping: 0.92,
                amplitude: 0.0,
                type_: ModeType::Air,
            },
            BodyMode {
                frequency: 900.0,
                damping: 0.90,
                amplitude: 0.0,
                type_: ModeType::Top,
            },
            BodyMode {
                frequency: 1100.0,
                damping: 0.88,
                amplitude: 0.0,
                type_: ModeType::Back,
            },
            BodyMode {
                frequency: 1400.0,
                damping: 0.86,
                amplitude: 0.0,
                type_: ModeType::Air,
            },
        ];

        Self {
            strings,
            body: ViolinBody {
                modes,
                top_amp: 0.0,
                back_amp: 0.0,
                f_hole_amp: 0.0,
            },
            bridge: Bridge {
                transmission: 0.6,
                height: 3.0,
                mass: 0.002,
                position: 0.85,
            },
            bow: Bow {
                position: 0.15,
                velocity: 0.0,
                pressure: 0.0,
                rosine: 0.8,
            },
            output: 0.0,
            sample_rate,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_string(&mut self, string: &mut ViolinString) -> f32 {
        if string.amplitude > 0.0001 {
            let dt = 1.0 / self.sample_rate;
            let k = 2.0 * PI * string.frequency * dt;

            string.phase += k;
            if string.phase > 2.0 * PI {
                string.phase -= 2.0 * PI;
            }

            let envelope = string.amplitude * string.decay.powf(1000.0 * dt);
            envelope * string.phase.sin()
        } else {
            0.0
        }
    }

    /// Technical implementation of the bow logic.
    pub fn bow(&mut self, string: usize, velocity: f32, pressure: f32) {
        if string < self.strings.len() {
            let s = &mut self.strings[string];
            s.amplitude = velocity.clamp(0.0, 1.0);

            let friction = pressure.clamp(0.0, 1.0) * self.bow.rosin;
            s.amplitude *= 0.5 + friction * 0.5;

            for mode in &mut self.body.modes {
                mode.amplitude += s.amplitude * 0.05;
            }
        }
    }

    /// Technical implementation of the pluck logic.
    pub fn pluck(&mut self, string: usize, position: f32, velocity: f32) {
        if string < self.strings.len() {
            let s = &mut self.strings[string];
            s.amplitude = velocity.clamp(0.0, 1.0);
            s.finger_position = position.clamp(0.0, 1.0);
        }
    }

    /// Technical implementation of the finger logic.
    pub fn finger(&mut self, string: usize, position: f32) {
        if string < self.strings.len() {
            let s = &mut self.strings[string];
            let base = [196.0, 293.66, 392.0, 523.25];
            let open = base[string];
            let pos = position.clamp(0.0, 1.0);
            s.frequency = open * 2.0_f32.powf(pos * 4.0);
            s.stopped = pos > 0.05;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;
        let mut string_out = 0.0;
        let mut bridge_force = 0.0;

        for string in &mut self.strings {
            let sample = self.process_string(string);
            string_out += sample;
            bridge_force += sample;

            if self.bow.pressure > 0.1 && string.amplitude < 0.01 {
                string.amplitude += self.bow.velocity * 0.1 * self.bow.pressure;
            }

            let damping = if string.stopped { 0.999 } else { 0.9995 };
            string.amplitude *= damping;
        }

        let mut body_out = 0.0;
        for mode in &mut self.body.modes {
            mode.amplitude *= mode.damping;
            let osc = (mode.frequency * 2.0 * PI * dt).sin();
            body_out += mode.amplitude * osc;
            mode.amplitude *= 0.9999;
        }

        match &mut self.body.modes[0].type_ {
            ModeType::Top => self.body.top_amp = body_out,
            ModeType::Back => self.body.back_amp = body_out,
            ModeType::Air => self.body.f_hole_amp = body_out,
        }

        let bridge_out = bridge_force * self.bridge.transmission;
        let body_vib = body_out * 0.4;
        let f_holes = body_out * self.body.f_hole_amp * 0.1;

        self.output = string_out * 0.3 + bridge_out * 0.3 + body_vib * 0.25 + f_holes * 0.15;
        self.output *= 0.75;

        self.output
    }

    /// Technical implementation of the set_bow_pressure logic.
    pub fn set_bow_pressure(&mut self, pressure: f32) {
        self.bow.pressure = pressure.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_bow_velocity logic.
    pub fn set_bow_velocity(&mut self, velocity: f32) {
        self.bow.velocity = velocity.clamp(-1.0, 1.0);
    }

    /// Technical implementation of the set_bridge_position logic.
    pub fn set_bridge_position(&mut self, position: f32) {
        self.bridge.position = position.clamp(0.5, 0.95);
    }

    /// Technical implementation of the set_rosin_level logic.
    pub fn set_rosin_level(&mut self, level: f32) {
        self.bow.rosin = level.clamp(0.0, 1.0);
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

impl Default for AcousticStrings {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
