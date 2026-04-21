/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6dcc3c4a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/acoustic_woodwind.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// and tone hole interactions.

use alloc::vec::Vec;
use core::f32::consts::PI;

#[repr(align(64))]
/// Technical implementation of the AcousticWoodwind structure.
pub struct AcousticWoodwind {
    reed: Reed,
    bore: WoodwindBore,
    finger_holes: Vec<FingerHole>,
    output: f32,
    sample_rate: f32,
}

struct Reed {
    stiffness: f32,
    opening: f32,
    pressure: f32,
    amplitude: f32,
    phase: f32,
    latency: f32,
}

struct WoodwindBore {
    length: f32,
    radius: f32,
    profile: BoreProfile,
    delay_line: [f32; 512],
    write_pos: usize,
    reflection: f32,
}

enum BoreProfile {
    Cylindrical,
    Conical,
    Tapered,
}

struct FingerHole {
    position: f32,
    open: bool,
    radius: f32,
    venting: f32,
}

impl AcousticWoodwind {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        let bore = WoodwindBore {
            length: 0.6,
            radius: 0.008,
            profile: BoreProfile::Cylindrical,
            delay_line: [0.0; 512],
            write_pos: 0,
            reflection: 0.8,
        };

        let mut finger_holes = Vec::with_capacity(12);
        for i in 0..12 {
            finger_holes.push(FingerHole {
                position: (i as f32 + 1.0) / 13.0,
                open: false,
                radius: 0.005,
                venting: 0.0,
            });
        }

        Self {
            reed: Reed {
                stiffness: 0.5,
                opening: 0.5,
                pressure: 0.5,
                amplitude: 0.0,
                phase: 0.0,
                latency: 0.0,
            },
            bore,
            finger_holes,
            output: 0.0,
            sample_rate,
        }
    }

    /// Technical implementation of the get_open_hole_count logic.
    fn get_open_hole_count(&self) -> usize {
        self.finger_holes.iter().filter(|h| h.open).count()
    }

    /// Technical implementation of the get_effective_length logic.
    fn get_effective_length(&self) -> f32 {
        let mut effective = self.bore.length;
        let open_holes = self.get_open_hole_count();

        if open_holes > 0 {
            let hole_factor = open_holes as f32 / self.finger_holes.len() as f32;
            effective *= 1.0 - hole_factor * 0.3;
        }

        effective
    }

    /// Technical implementation of the blow logic.
    pub fn blow(&mut self, pressure: f32) {
        let p = pressure.clamp(0.0, 1.0);
        self.reed.pressure = p;

        let effective_len = self.get_effective_length();
        let base_freq = 440.0 / effective_len;

        self.reed.amplitude = p * (1.0 - self.reed.stiffness * 0.5);
        self.reed.opening = 0.3 + p * 0.4;
    }

    /// Technical implementation of the finger logic.
    pub fn finger(&mut self, hole: usize, open: bool) {
        if hole < self.finger_holes.len() {
            self.finger_holes[hole].open = open;

            let idx = hole as isize - 1;
            if idx >= 0 && (idx as usize) < self.finger_holes.len() {
                self.finger_holes[idx as usize].venting = if open { 0.0 } else { 0.5 };
            }
        }
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    fn process_reed(&mut self) -> f32 {
        let dt = 1.0 / self.sample_rate;

        let effective_len = self.get_effective_length();
        let freq = 440.0 / effective_len;

        self.reed.phase += 2.0 * PI * freq * dt;
        if self.reed.phase > 2.0 * PI {
            self.reed.phase -= 2.0 * PI;
        }

        let reed_vibration = self.reed.amplitude * self.reed.phase.sin();
        let stiffness_effect = 1.0 + self.reed.stiffness * reed_vibration;

        let pressure_osc = self.reed.pressure * (1.0 + reed_vibration * 0.3);

        pressure_osc * stiffness_effect
    }

    /// Primary real-time signal processing execution block.
    #[inline(always)]
    pub fn process(&mut self) -> f32 {
        let reed_signal = self.process_reed();

        self.bore.delay_line[self.bore.write_pos] = reed_signal;
        self.bore.write_pos = (self.bore.write_pos + 1) % 512;
        let delayed = self.bore.delay_line[self.bore.write_pos];

        let bore_out = reed_signal * 0.7 + delayed * 0.3;

        let mut hole_venting = 0.0;
        for hole in &self.finger_holes {
            if hole.open {
                hole_venting += hole.venting * 0.1;
            }
        }

        let bore_filtered = bore_out * (1.0 - hole_venting);

        let mut radiation = 0.0;
        let open_count = self.get_open_hole_count() as f32;
        if open_count > 0 {
            let freq = 440.0 / self.get_effective_length();
            let radiation_factor = open_count / self.finger_holes.len() as f32;
            radiation = bore_filtered * radiation_factor;
        }

        self.output = bore_filtered * 0.6 + radiation * 0.4;
        self.output *= 0.6;

        self.output
    }

    /// Technical implementation of the set_reed_stiffness logic.
    pub fn set_reed_stiffness(&mut self, stiffness: f32) {
        self.reed.stiffness = stiffness.clamp(0.1, 1.0);
    }

    /// Technical implementation of the set_bore_profile logic.
    pub fn set_bore_profile(&mut self, profile: usize) {
        self.bore.profile = match profile {
            0 => BoreProfile::Cylindrical,
            1 => BoreProfile::Conical,
            2 => BoreProfile::Tapered,
            _ => BoreProfile::Cylindrical,
        };
    }

    /// Technical implementation of the set_finger_hole_radius logic.
    pub fn set_finger_hole_radius(&mut self, hole: usize, radius: f32) {
        if hole < self.finger_holes.len() {
            self.finger_holes[hole].radius = radius.clamp(0.002, 0.01);
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

impl Default for AcousticWoodwind {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(44100.0)
    }
}
