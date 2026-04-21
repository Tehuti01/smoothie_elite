/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x89fdd669 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_brass.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// bell radiation, and valve simulation.

use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticBrass structure.
pub struct AcousticBrass {
    lip_osc: LipOscillator,
    mouthpiece: Mouthpiece,
    bore: Bore,
    bell: Bell,
    valves: [Valve; 3],
    output: f32,
    sample_rate: f32,
}

struct LipOscillator {
    frequency: f32,
    amplitude: f32,
    pressure: f32,
    stiffness: f32,
    separation: f32,
    phase: f32,
    drive: f32,
}

struct Mouthpiece {
    volume: f32,
    back_pressure: f32,
    resistance: f32,
}

struct Bore {
    length: f32,
    radius: f32,
    curvature: f32,
    losses: f32,
    delay_line: [f32; 1024],
    write_pos: usize,
}

struct Bell {
    flare: f32,
    radiation: f32,
    cutoff: f32,
    amplitude: f32,
    modes: [f32; 4],
}

struct Valve {
    position: f32,
    open: bool,
    length_add: f32,
}

impl AcousticBrass {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut bore = Bore {
            length: 2.0,
            radius: 0.01,
            curvature: 0.5,
            losses: 0.98,
            delay_line: [0.0; 1024],
            write_pos: 0,
        };
        for i in 0..1024 {
            bore.delay_line[i] = 0.0;
        }

        Self {
            lip_osc: LipOscillator {
                frequency: 220.0,
                amplitude: 0.0,
                pressure: 0.5,
                stiffness: 0.3,
                separation: 0.002,
                phase: 0.0,
                drive: 0.5,
            },
            mouthpiece: Mouthpiece {
                volume: 5.0,
                back_pressure: 0.0,
                resistance: 0.1,
            },
            bore,
            bell: Bell {
                flare: 2.5,
                radiation: 0.8,
                cutoff: 2000.0,
                amplitude: 0.0,
                modes: [1.0, 0.5, 0.25, 0.125],
            },
            valves: [
                Valve {
                    position: 0.0,
                    open: false,
                    length_add: 0.0,
                },
                Valve {
                    position: 0.0,
                    open: false,
                    length_add: 0.0,
                },
                Valve {
                    position: 0.0,
                    open: false,
                    length_add: 0.0,
                },
            ],
            output: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the blow logic.
    pub fn blow(&mut self, pressure: f32, breath_noise: f32) {
        let p = pressure.clamp(0.0, 1.0);
        let noise = breath_noise.clamp(0.0, 0.1);

        self.lip_osc.pressure = p;
        self.lip_osc.amplitude = p * 0.8 + noise;

        let base_freq = self.get_tube_length();
        self.lip_osc.frequency = base_freq * (1.0 + p * 0.1);

        let drive = self.lip_osc.drive * p;
        self.lip_osc.separation = 0.001 + drive * 0.002;
    }

    /// Technical implementation of the get_tube_length logic.
    fn get_tube_length(&self) -> f32 {
        let mut length = self.bore.length;
        for valve in &self.valves {
            length += valve.length_add;
        }
        220.0 / length.max(0.1)
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_lips(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        self.lip_osc.phase += 2.0 * PI * self.lip_osc.frequency * dt;
        if self.lip_osc.phase > 2.0 * PI {
            self.lip_osc.phase -= 2.0 * PI;
        }

        let oscillation = self.lip_osc.amplitude * self.lip_osc.phase.sin();
        let stiffness_mod = 1.0 + self.lip_osc.stiffness * oscillation;

        self.lip_osc.frequency *= stiffness_mod;

        oscillation
    }

    /// Technical implementation of the press_valve logic.
    pub fn press_valve(&mut self, valve: usize, position: f32) {
        if valve < 3 {
            self.valves[valve].position = position.clamp(0.0, 1.0);
            self.valves[valve].open = position > 0.5;

            self.valves[valve].length_add = match valve {
                0 => 0.0,
                1 => 0.05,
                2 => 0.15,
                _ => 0.0,
            } * position;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let lip_signal = self.process_lips();

        self.bore.delay_line[self.bore.write_pos] = lip_signal;
        self.bore.write_pos = (self.bore.write_pos + 1) % 1024;
        let delayed = self.bore.delay_line[self.bore.write_pos];

        let bore_out = lip_signal * 0.6 + delayed * 0.4;
        let bore_filtered = bore_out * self.bore.losses;

        let mut bell_out = 0.0;
        let fundamental = bore_filtered;

        for (i, &mode) in self.bell.modes.iter().enumerate() {
            let harm = (i + 1) as f32;
            let freq = self.lip_osc.frequency * harm;
            if freq < self.bell.cutoff {
                bell_out += fundamental * mode / harm;
            }
        }

        bell_out *= self.bell.flare.sqrt();
        self.bell.amplitude = self.bell.amplitude * 0.99 + bell_out * 0.01;

        let radiation = self.bell.amplitude * self.bell.radiation;

        self.output = lip_signal * 0.2 + bore_filtered * 0.3 + radiation * 0.5;
        self.output *= 0.7;

        self.output
    }

    /// Technical implementation of the set_mouthpiece logic.
    pub fn set_mouthpiece(&mut self, volume: f32, resistance: f32) {
        self.mouthpiece.volume = volume.clamp(1.0, 20.0);
        self.mouthpiece.resistance = resistance.clamp(0.01, 1.0);
    }

    /// Technical implementation of the set_bell_flare logic.
    pub fn set_bell_flare(&mut self, flare: f32) {
        self.bell.flare = flare.clamp(1.0, 5.0);
    }

    /// Technical implementation of the set_lip_stiffness logic.
    pub fn set_lip_stiffness(&mut self, stiffness: f32) {
        self.lip_osc.stiffness = stiffness.clamp(0.1, 1.0);
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

impl Default for AcousticBrass {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
