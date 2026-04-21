/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xbbaa0857 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_organ.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// and stop control.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticOrgan structure.
pub struct AcousticOrgan {
    ranks: Vec<OrganRank>,
    wind_supply: WindSupply,
    wind_chest: WindChest,
    console: OrganConsole,
    output: f32,
    sample_rate: f32,
}

struct OrganRank {
    name: String,
    pipes: Vec<OrganPipe>,
    division: Division,
    attenuation: f32,
    mix_level: f32,
}

enum Division {
    Pedal,
    Great,
    Swell,
    Choir,
}

struct OrganPipe {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    decay: f32,
    harmonic: f32,
    pipe_type: PipeType,
    open: bool,
}

enum PipeType {
    Principal,
    Flute,
    String,
    Reed,
}

struct WindSupply {
    bellows_position: f32,
    pressure: f32,
    ripple: f32,
    noise: f32,
}

struct WindChest {
    pressure: f32,
    capacity: f32,
    current_load: f32,
}

struct OrganConsole {
    master_volume: f32,
    tremulant: bool,
    tremulant_speed: f32,
    tremulant_depth: f32,
    tremulant_phase: f32,
}

impl AcousticOrgan {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let mut ranks = Vec::new();

        ranks.push(OrganRank {
            name: "Principal".into(),
            pipes: Self::create_rank(8, PipeType::Principal),
            division: Division::Great,
            attenuation: 1.0,
            mix_level: 0.8,
        });

        ranks.push(OrganRank {
            name: "Flute".into(),
            pipes: Self::create_rank(8, PipeType::Flute),
            division: Division::Great,
            attenuation: 1.0,
            mix_level: 0.6,
        });

        ranks.push(OrganRank {
            name: "String".into(),
            pipes: Self::create_rank(8, PipeType::String),
            division: Division::Swell,
            attenuation: 1.0,
            mix_level: 0.5,
        });

        ranks.push(OrganRank {
            name: "Reed".into(),
            pipes: Self::create_rank(8, PipeType::Reed),
            division: Division::Pedal,
            attenuation: 1.0,
            mix_level: 0.7,
        });

        Self {
            ranks,
            wind_supply: WindSupply {
                bellows_position: 0.8,
                pressure: 1.0,
                ripple: 0.02,
                noise: 0.005,
            },
            wind_chest: WindChest {
                pressure: 1.0,
                capacity: 10.0,
                current_load: 0.0,
            },
            console: OrganConsole {
                master_volume: 0.8,
                tremulant: false,
                tremulant_speed: 4.0,
                tremulant_depth: 0.3,
                tremulant_phase: 0.0,
            },
            output: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the create_rank logic.
    fn create_rank(count: usize, pipe_type: PipeType) -> Vec<OrganPipe> {
        let mut pipes = Vec::with_capacity(count);
        let base_freq = 55.0;

        for i in 0..count {
            let freq = base_freq * 2.0_f32.powf(i as f32 / 12.0);
            let harmonic = match pipe_type {
                PipeType::Principal => 1.0,
                PipeType::Flute => 0.3,
                PipeType::String => 2.0,
                PipeType::Reed => 3.0,
            };
            let decay = match pipe_type {
                PipeType::Principal => 0.998,
                PipeType::Flute => 0.999,
                PipeType::String => 0.997,
                PipeType::Reed => 0.996,
            };

            pipes.push(OrganPipe {
                frequency: freq,
                amplitude: 0.0,
                phase: 0.0,
                decay,
                harmonic,
                pipe_type,
                open: false,
            });
        }

        pipes
    }

    /// Technical implementation of the key_on logic.
    pub fn key_on(&mut self, rank: usize, note: usize) {
        if rank < self.ranks.len() {
            let rank = &mut self.ranks[rank];
            if note < rank.pipes.len() {
                let pipe = &mut rank.pipes[note];
                pipe.open = true;
                pipe.amplitude = 1.0;

                self.wind_chest.current_load += 0.1;
            }
        }
    }

    /// Technical implementation of the key_off logic.
    pub fn key_off(&mut self, rank: usize, note: usize) {
        if rank < self.ranks.len() {
            let rank = &mut self.ranks[rank];
            if note < rank.pipes.len() {
                rank.pipes[note].open = false;

                self.wind_chest.current_load = (self.wind_chest.current_load - 0.1).max(0.0);
            }
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_pipe(&mut self, pipe: &mut OrganPipe) -> f32 {
        if pipe.open && pipe.amplitude > 0.0001 {
            let dt = 1.0 / self.sample_rate;

            pipe.phase += 2.0 * PI * pipe.frequency * dt;
            if pipe.phase > 2.0 * PI {
                pipe.phase -= 2.0 * PI;
            }

            let fundamental = pipe.amplitude * pipe.phase.sin();
            let overtone = pipe.amplitude * 0.3 * (pipe.phase * pipe.harmonic).sin();

            let wind_pressure = self.wind_supply.pressure
                * (1.0 + self.wind_supply.ripple * (self.wind_supply.ripple * 10.0 * dt).sin());
            let pressure_mod = fundamental * wind_pressure;

            pipe.amplitude *= pipe.decay;

            pressure_mod + overtone
        } else {
            0.0
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        self.wind_supply.ripple = 0.02 + self.wind_chest.current_load * 0.01;

        self.wind_chest.pressure =
            self.wind_supply.pressure * (1.0 - self.wind_chest.current_load * 0.05);

        let mut rank_outputs = Vec::new();

        for rank in &mut self.ranks {
            let mut rank_out = 0.0;

            for pipe in &mut rank.pipes {
                if pipe.open {
                    rank_out += self.process_pipe(pipe);
                }
            }

            rank_out *= rank.attenuation * rank.mix_level;
            rank_outputs.push(rank_out);
        }

        let trem = if self.console.tremulant {
            self.console.tremulant_phase += 2.0 * PI * self.console.tremulant_speed * dt;
            if self.console.tremulant_phase > 2.0 * PI {
                self.console.tremulant_phase -= 2.0 * PI;
            }
            1.0 - self.console.tremulant_depth * (1.0 + self.console.tremulant_phase.sin()) * 0.5
        } else {
            1.0
        };

        let mut sum: f32 = rank_outputs.iter().sum();
        sum *= trem;
        sum *= self.console.master_volume;

        self.output = sum * 0.3;

        self.output
    }

    /// Technical implementation of the set_stop logic.
    pub fn set_stop(&mut self, rank: usize, value: f32) {
        if rank < self.ranks.len() {
            self.ranks[rank].attenuation = value.clamp(0.0, 1.0);
        }
    }

    /// Technical implementation of the set_tremulant logic.
    pub fn set_tremulant(&mut self, enabled: bool) {
        self.console.tremulant = enabled;
    }

    /// Technical implementation of the set_wind_pressure logic.
    pub fn set_wind_pressure(&mut self, pressure: f32) {
        self.wind_supply.pressure = pressure.clamp(0.5, 1.5);
    }

    /// Technical implementation of the set_master_volume logic.
    pub fn set_master_volume(&mut self, volume: f32) {
        self.console.master_volume = volume.clamp(0.0, 1.0);
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

impl Default for AcousticOrgan {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
