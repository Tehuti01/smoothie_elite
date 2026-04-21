/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x8cfeda06 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_drums.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Physical modeling of drum heads, shells, snare wires, and kick drum.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticDrums structure.
pub struct AcousticDrums {
    kick: KickDrum,
    snare: SnareDrum,
    tom_high: Tom,
    tom_mid: Tom,
    floor_tom: Tom,
    hi_hat: HiHat,
    sample_rate: f32,
    output: f32,
}

#[repr(align(64))]
/// Technical implementation of the KickDrum structure.
pub struct KickDrum {
    head: DrumHead,
    shell: f32,
    mass: f32,
    port: f32,
    output: f32,
}

#[repr(align(64))]
/// Technical implementation of the DrumHead structure.
pub struct DrumHead {
    frequency: f32,
    tension: f32,
    diameter: f32,
    amplitude: f32,
    phase: f32,
    decay: f32,
    overtone_ratio: f32,
}

#[repr(align(64))]
/// Technical implementation of the SnareDrum structure.
pub struct SnareDrum {
    top_head: DrumHead,
    bottom_head: DrumHead,
    wires: Vec<SnareWire>,
    shell: f32,
    side: f32,
    output: f32,
}

#[repr(align(64))]
/// Technical implementation of the SnareWire structure.
pub struct SnareWire {
    frequency: f32,
    tension: f32,
    amplitude: f32,
    decay: f32,
}

#[repr(align(64))]
/// Technical implementation of the Tom structure.
pub struct Tom {
    head: DrumHead,
    shell: f32,
    diameter: f32,
    depth: f32,
    output: f32,
}

#[repr(align(64))]
/// Technical implementation of the HiHat structure.
pub struct HiHat {
    cymbal_top: Cymbal,
    cymbal_bottom: Cymbal,
    pedal: f32,
    open: bool,
    output: f32,
}

#[repr(align(64))]
/// Technical implementation of the Cymbal structure.
pub struct Cymbal {
    frequency: f32,
    amplitude: f32,
    decay: f32,
    overtone_count: usize,
}

impl AcousticDrums {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            kick: KickDrum {
                head: DrumHead {
                    frequency: 60.0,
                    tension: 5000.0,
                    diameter: 20.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.95,
                    overtone_ratio: 1.0,
                },
                shell: 0.8,
                mass: 5.0,
                port: 0.3,
                output: 0.0,
            },
            snare: SnareDrum {
                top_head: DrumHead {
                    frequency: 180.0,
                    tension: 4000.0,
                    diameter: 14.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.97,
                    overtone_ratio: 1.5,
                },
                bottom_head: DrumHead {
                    frequency: 200.0,
                    tension: 3500.0,
                    diameter: 14.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.96,
                    overtone_ratio: 1.3,
                },
                wires: vec![
                    SnareWire {
                        frequency: 3000.0,
                        tension: 1000.0,
                        amplitude: 0.0,
                        decay: 0.92,
                    },
                    SnareWire {
                        frequency: 4000.0,
                        tension: 1200.0,
                        amplitude: 0.0,
                        decay: 0.90,
                    },
                    SnareWire {
                        frequency: 5000.0,
                        tension: 1400.0,
                        amplitude: 0.0,
                        decay: 0.88,
                    },
                    SnareWire {
                        frequency: 6000.0,
                        tension: 1600.0,
                        amplitude: 0.0,
                        decay: 0.86,
                    },
                ],
                shell: 0.7,
                side: 0.5,
                output: 0.0,
            },
            tom_high: Tom {
                head: DrumHead {
                    frequency: 200.0,
                    tension: 4500.0,
                    diameter: 12.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.965,
                    overtone_ratio: 1.4,
                },
                shell: 0.6,
                diameter: 12.0,
                depth: 8.0,
                output: 0.0,
            },
            tom_mid: Tom {
                head: DrumHead {
                    frequency: 170.0,
                    tension: 4200.0,
                    diameter: 13.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.97,
                    overtone_ratio: 1.35,
                },
                shell: 0.65,
                diameter: 13.0,
                depth: 10.0,
                output: 0.0,
            },
            floor_tom: Tom {
                head: DrumHead {
                    frequency: 100.0,
                    tension: 4000.0,
                    diameter: 16.0,
                    amplitude: 0.0,
                    phase: 0.0,
                    decay: 0.975,
                    overtone_ratio: 1.25,
                },
                shell: 0.7,
                diameter: 16.0,
                depth: 14.0,
                output: 0.0,
            },
            hi_hat: HiHat {
                cymbal_top: Cymbal {
                    frequency: 4000.0,
                    amplitude: 0.0,
                    decay: 0.85,
                    overtone_count: 6,
                },
                cymbal_bottom: Cymbal {
                    frequency: 4500.0,
                    amplitude: 0.0,
                    decay: 0.83,
                    overtone_count: 5,
                },
                pedal: 1.0,
                open: false,
                output: 0.0,
            },
            sample_rate,
            output: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_head(&mut self, head: &mut DrumHead) -> f32 {
        if head.amplitude > 0.0001 {
            let dt = 1.0 / self.sample_rate;
            head.phase += 2.0 * PI * head.frequency * dt;
            if head.phase > 2.0 * PI {
                head.phase -= 2.0 * PI;
            }

            let fundamental = head.amplitude * head.phase.sin();
            let overtone = head.amplitude * 0.3 * (head.phase * head.overtone_ratio).sin();

            head.amplitude *= head.decay;

            fundamental + overtone
        } else {
            0.0
        }
    }

    /// Technical implementation of the kick logic.
    pub fn kick(&mut self, velocity: f32) {
        self.kick.head.amplitude = velocity.clamp(0.0, 1.0);
        self.kick.head.phase = 0.0;
    }

    /// Technical implementation of the snare logic.
    pub fn snare(&mut self, velocity: f32) {
        let v = velocity.clamp(0.0, 1.0);
        self.snare.top_head.amplitude = v;
        self.snare.top_head.phase = 0.0;
        self.snare.bottom_head.amplitude = v * 0.7;
        self.snare.bottom_head.phase = 0.0;

        for wire in &mut self.snare.wires {
            wire.amplitude = v * 0.5;
        }
    }

    /// Technical implementation of the tom logic.
    pub fn tom(&mut self, tom: usize, velocity: f32) {
        let t = match tom {
            0 => &mut self.tom_high,
            1 => &mut self.tom_mid,
            2 => &mut self.floor_tom,
            _ => return,
        };
        t.head.amplitude = velocity.clamp(0.0, 1.0);
        t.head.phase = 0.0;
    }

    /// Technical implementation of the hit_hat logic.
    pub fn hit_hat(&mut self, velocity: f32) {
        let v = velocity.clamp(0.0, 1.0);
        self.hi_hat.cymbal_top.amplitude = v;
        self.hi_hat.cymbal_bottom.amplitude = v * 0.8;

        if !self.hi_hat.open {
            self.hi_hat.cymbal_top.decay = 0.85;
            self.hi_hat.cymbal_bottom.decay = 0.83;
        } else {
            self.hi_hat.cymbal_top.decay = 0.95;
            self.hi_hat.cymbal_bottom.decay = 0.94;
        }
    }

    /// Technical implementation of the set_hat_pedal logic.
    pub fn set_hat_pedal(&mut self, position: f32) {
        self.hi_hat.pedal = position.clamp(0.0, 1.0);
        self.hi_hat.open = position > 0.5;
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        let kick_out = {
            let k = &mut self.kick;
            if k.head.amplitude > 0.0001 {
                k.head.phase += 2.0 * PI * k.head.frequency * dt;
                if k.head.phase > 2.0 * PI {
                    k.head.phase -= 2.0 * PI;
                }
                let out = k.head.amplitude * k.head.phase.sin();
                k.head.amplitude *= k.head.decay;
                out
            } else {
                0.0
            }
        };
        self.kick.output = kick_out;

        let snare_head = self.process_head(&mut self.snare.top_head);
        let snare_bottom = self.process_head(&mut self.snare.bottom_head);
        let mut wire_out = 0.0;
        for wire in &mut self.snare.wires {
            if wire.amplitude > 0.0001 {
                wire.amplitude *= wire.decay;
                wire_out += wire.amplitude * (wire.frequency * 2.0 * PI * dt).sin();
            }
        }
        let snare_out = snare_head * 0.4 + snare_bottom * 0.3 + wire_out * 0.3;
        self.snare.output = snare_out;

        let tom_out = self.process_head(&mut self.tom_high.head) * 0.33
            + self.process_head(&mut self.tom_mid.head) * 0.33
            + self.process_head(&mut self.floor_tom.head) * 0.34;
        self.tom_high.output = self.process_head(&mut self.tom_high.head);
        self.tom_mid.output = self.process_head(&mut self.tom_mid.head);
        self.floor_tom.output = self.process_head(&mut self.floor_tom.head);

        let hat_decay = if self.hi_hat.open { 0.95 } else { 0.85 };
        let mut hat_out = 0.0;
        self.hi_hat.cymbal_top.amplitude *= hat_decay;
        self.hi_hat.cymbal_bottom.amplitude *= hat_decay;

        for i in 0..self.hi_hat.cymbal_top.overtone_count {
            let freq = self.hi_hat.cymbal_top.frequency * (1.0 + i as f32 * 0.5);
            hat_out +=
                self.hi_hat.cymbal_top.amplitude * (freq * 2.0 * PI * dt).sin() / (1.0 + i as f32);
        }
        self.hi_hat.output = hat_out * 0.5;

        self.output = self.kick.output * 0.3
            + self.snare.output * 0.3
            + (self.tom_high.output + self.tom_mid.output + self.floor_tom.output) * 0.15
            + self.hi_hat.output * 0.25;

        self.output * 0.7
    }

    /// Technical implementation of the set_kick_tuning logic.
    pub fn set_kick_tuning(&mut self, freq: f32) {
        self.kick.head.frequency = freq.clamp(30.0, 120.0);
    }

    /// Technical implementation of the set_snare_tension logic.
    pub fn set_snare_tension(&mut self, tension: f32) {
        let t = tension.clamp(0.1, 2.0);
        self.snare.top_head.tension = 4000.0 * t;
        self.snare.top_head.frequency = 180.0 * t.sqrt();
    }

    /// Technical implementation of the set_hat_tension logic.
    pub fn set_hat_tension(&mut self, tension: f32) {
        let t = tension.clamp(0.1, 2.0);
        self.hi_hat.cymbal_top.frequency = 4000.0 * t.sqrt();
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

impl Default for AcousticDrums {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
