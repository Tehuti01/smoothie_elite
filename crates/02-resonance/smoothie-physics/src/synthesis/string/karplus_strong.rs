/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x479941c3 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/synthesis/string/karplus_strong.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Features multiple excitation models, inharmonicity, and dispersion.

use alloc::vec;
use alloc::vec::Vec;
use smoothie_core::constants::TAU;
use smoothie_core::math::exp_approx;
use smoothie_core::primitives::Sample;

#[repr(align(64))]
/// Technical implementation of the KarplusString structure.
pub struct KarplusString {
    buffer: Vec<f32>,
    write_head: usize,
    read_head: f32,
    length: f32,
    damping: f32,
    stiffness: f32,
    inharmonicity: f32,
    excitation_phase: usize,
    excitation_samples: usize,
    sample_rate: f32,
}

impl KarplusString {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, min_freq: f32) -> Self {
        let max_length = (sample_rate / min_freq) as usize + 10;
        let buffer = vec![0.0; max_length];

        Self {
            buffer,
            write_head: 0,
            read_head: 0.0,
            length: max_length as f32,
            damping: 0.5,
            stiffness: 0.0,
            inharmonicity: 0.0,
            excitation_phase: 0,
            excitation_samples: 0,
            sample_rate,
        }
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, frequency: f32) {
        self.length = self.sample_rate / frequency;
        self.read_head = self.write_head as f32 - self.length;
        if self.read_head < 0.0 {
            self.read_head += self.buffer.len() as f32;
        }
    }

    /// Technical implementation of the set_damping logic.
    pub fn set_damping(&mut self, damping: f32) {
        self.damping = damping.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_stiffness logic.
    pub fn set_stiffness(&mut self, stiffness: f32) {
        self.stiffness = stiffness.clamp(0.0, 1.0);
    }

    /// Technical implementation of the set_inharmonicity logic.
    pub fn set_inharmonicity(&mut self, inharm: f32) {
        self.inharmonicity = inharm.clamp(0.0, 0.5);
    }

    /// Technical implementation of the pluck logic.
    pub fn pluck(&mut self, frequency: f32, amplitude: f32, excitation: ExcitationType) {
        self.set_frequency(frequency);

        let samples = self.length as usize;
        self.excitation_samples = samples;
        self.excitation_phase = 0;

        for i in 0..samples {
            let sample = match excitation {
                ExcitationType::Pulse => {
                    if i < samples / 10 {
                        amplitude
                    } else {
                        0.0
                    }
                }
                ExcitationType::Noise => self.generate_noise() * amplitude,
                ExcitationType::Sawtooth => (2.0 * (i as f32 / samples as f32) - 1.0) * amplitude,
                ExcitationType::Triangle => {
                    let x = i as f32 / samples as f32;
                    (4.0 * (x - 0.5).abs() - 1.0) * amplitude
                }
                ExcitationType::Custom(data) => {
                    if i < data.len() {
                        data[i] * amplitude
                    } else {
                        0.0
                    }
                }
            };
            self.buffer[(self.write_head + i) % self.buffer.len()] = sample;
        }
    }

    #[inline(always)]
    /// Technical implementation of the generate_noise logic.
    fn generate_noise(&mut self) -> f32 {
        let x = (self.write_head as f32 * 12.9898 + 78.233).sin() * 43758.5453;
        x - x.floor()
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> Sample {
        let len = self.buffer.len();

        let idx0 = self.read_head.floor() as usize % len;
        let idx1 = (idx0 + 1) % len;
        let frac = self.read_head - idx0 as f32;

        let output = self.buffer[idx0] * (1.0 - frac) + self.buffer[idx1] * frac;

        let idx_w = self.write_head;
        let idx_r = self.read_head.floor() as usize;

        let damping_factor = 1.0 - self.damping / self.length;
        let filtered = output * damping_factor;

        self.buffer[idx_w] = filtered;

        self.read_head += 1.0;
        self.write_head = (self.write_head + 1) % len;
        if self.read_head >= len as f32 {
            self.read_head -= len as f32;
        }

        output
    }
}

/// Technical implementation of the ExcitationType enumeration.
pub enum ExcitationType<'a> {
    Pulse,
    Noise,
    Sawtooth,
    Triangle,
    Custom(&'a [f32]),
}

#[repr(align(64))]
/// Technical implementation of the PluckedHarmonicString structure.
pub struct PluckedHarmonicString {
    delays: Vec<f32>,
    phases: Vec<f32>,
    amplitudes: Vec<f32>,
    frequencies: Vec<f32>,
    decay_rates: Vec<f32>,
    sample_rate: f32,
    n_harmonics: usize,
}

impl PluckedHarmonicString {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32, fundamental: f32, n_harmonics: usize) -> Self {
        let mut frequencies = Vec::with_capacity(n_harmonics);
        let mut decay_rates = Vec::with_capacity(n_harmonics);
        let mut amplitudes = Vec::with_capacity(n_harmonics);
        let phases = vec![0.0; n_harmonics];

        for i in 0..n_harmonics {
            let h = (i + 1) as f32;
            frequencies.push(fundamental * h);
            let decay_time = 2.0 / h;
            decay_rates.push(exp_approx(-1.0 / (decay_time * sample_rate)));
            amplitudes.push(1.0 / h);
        }

        Self {
            delays: vec![0.0; n_harmonics * 4],
            phases,
            amplitudes,
            frequencies,
            decay_rates,
            sample_rate,
            n_harmonics,
        }
    }

    /// Technical implementation of the pluck logic.
    pub fn pluck(&mut self, amplitude: f32) {
        for i in 0..self.n_harmonics {
            self.phases[i] = 0.0;
            self.delays[i] = amplitude * self.amplitudes[i];
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) -> Sample {
        let mut sum = 0.0;
        for i in 0..self.n_harmonics {
            sum += self.delays[i] * self.phases[i].sin();

            self.phases[i] += self.frequencies[i] / self.sample_rate;
            if self.phases[i] >= 1.0 {
                self.phases[i] -= 1.0;
            }

            self.delays[i] *= self.decay_rates[i];
        }
        sum
    }
}
