/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2c80128a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_piano.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Physical modeling of piano hammer, string, soundboard, and damper physics.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticPiano structure.
pub struct AcousticPiano {
    strings: Vec<PianoString>,
    hammer: Hammer,
    soundboard: ModalResonator,
    dampers: Vec<Damper>,
    sample_rate: f32,
}

struct PianoString {
    frequency: f32,
    decay: f32,
    tension: f32,
    length: f32,
    inharmonicity: f32,
    amplitude: f32,
    phase: f32,
}

struct Hammer {
    mass: f32,
    stiffness: f32,
    position: f32,
    velocity: f32,
    strike_time: f32,
}

struct ModalResonator {
    modes: Vec<ModalMode>,
    output: f32,
}

struct ModalMode {
    frequency: f32,
    damping: f32,
    amplitude: f32,
}

struct Damper {
    position: f32,
    lifted: bool,
    damping_coef: f32,
}

impl AcousticPiano {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut strings = Vec::with_capacity(88);
        for i in 0..88 {
            let freq = 440.0 * 2.0_f32.powf((i as f32 - 69.0) / 12.0);
            let length = 1.0 / freq;
            let tension = 800.0;
            let inharmonicity = 0.1 * (i as f32 + 1.0) / 88.0;
            strings.push(PianoString {
                frequency: freq,
                decay: 0.9995,
                tension,
                length,
                inharmonicity,
                amplitude: 0.0,
                phase: 0.0,
            });
        }

        let modes = vec![
            ModalMode {
                frequency: 220.0,
                damping: 0.98,
                amplitude: 0.0,
            },
            ModalMode {
                frequency: 440.0,
                damping: 0.985,
                amplitude: 0.0,
            },
            ModalMode {
                frequency: 660.0,
                damping: 0.975,
                amplitude: 0.0,
            },
            ModalMode {
                frequency: 880.0,
                damping: 0.97,
                amplitude: 0.0,
            },
        ];

        let mut dampers = Vec::with_capacity(88);
        for _ in 0..88 {
            dampers.push(Damper {
                position: 0.0,
                lifted: false,
                damping_coef: 0.95,
            });
        }

        Self {
            strings,
            hammer: Hammer {
                mass: 0.005,
                stiffness: 1e8,
                position: 0.0,
                velocity: 0.0,
                strike_time: 0.0,
            },
            soundboard: ModalResonator { modes, output: 0.0 },
            dampers,
            sample_rate,
        }
    }

    /// Technical implementation of the note_on logic.
    pub fn note_on(&mut self, note: usize, velocity: f32) {
        if note < self.strings.len() {
            let string = &mut self.strings[note];
            string.amplitude = velocity.clamp(0.0, 1.0);
            string.phase = 0.0;

            let imp = velocity * 5.0;
            for mode in &mut self.soundboard.modes {
                mode.amplitude += imp * 0.1;
            }

            if note < self.dampers.len() {
                self.dampers[note].lifted = true;
            }
        }
    }

    /// Technical implementation of the note_off logic.
    pub fn note_off(&mut self, note: usize) {
        if note < self.dampers.len() {
            self.dampers[note].lifted = false;
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let mut output = 0.0;
        let dt = 1.0 / self.sample_rate;

        for (i, string) in self.strings.iter_mut().enumerate() {
            if string.amplitude > 0.001 {
                let mut freq = string.frequency;
                for n in 1..8 {
                    let partial = n as f32;
                    let offset = string.inharmonicity * partial * partial;
                    freq = string.frequency * (1.0 + offset);
                    string.phase += 2.0 * PI * freq * dt;
                    if string.phase > 2.0 * PI {
                        string.phase -= 2.0 * PI;
                    }
                }

                let sample = string.amplitude * (string.phase * 0.5).sin();
                output += sample;

                let damping = if i < self.dampers.len() && self.dampers[i].lifted {
                    string.decay
                } else {
                    string.decay * self.dampers.get(i).map(|d| d.damping_coef).unwrap_or(0.9)
                };
                string.amplitude *= damping;
            }
        }

        let mut board_out = 0.0;
        for mode in &mut self.soundboard.modes {
            mode.amplitude *= mode.damping;
            board_out += mode.amplitude * (mode.frequency * 2.0 * PI * dt).sin();
            mode.amplitude *= 0.999;
        }
        self.soundboard.output = board_out;

        output * 0.3 + board_out * 0.15
    }

    /// Technical implementation of the set_hammer_hardness logic.
    pub fn set_hammer_hardness(&mut self, hardness: f32) {
        self.hammer.stiffness = 1e8 * hardness.clamp(0.1, 2.0);
    }

    /// Technical implementation of the set_pedal logic.
    pub fn set_pedal(&mut self, pedaled: bool) {
        for damper in &mut self.dampers {
            damper.lifted = pedaled;
        }
    }

    /// Technical implementation of the set_string_tension logic.
    pub fn set_string_tension(&mut self, note: usize, tension: f32) {
        if note < self.strings.len() {
            self.strings[note].tension = tension.clamp(100.0, 2000.0);
        }
    }

    /// Technical implementation of the get_amplitude logic.
    pub fn get_amplitude(&self, note: usize) -> f32 {
        self.strings.get(note).map(|s| s.amplitude).unwrap_or(0.0)
    }

    /// Technical implementation of the set_sample_rate logic.
    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
    }
}

impl Default for AcousticPiano {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
