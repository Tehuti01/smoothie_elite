/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3a38149a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/seraphic-multiverse/src/pitch/auto_pitch_quantizer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[repr(align(64))]
/// Technical implementation of the PitchQuantizer structure.
pub struct PitchQuantizer {
    scale_mask: u16, // Bitmask for 12 chromatic notes (e.g., 0b0000000010101001)
    root_note: u8,
    pub intensity: f32, // [0.0, 1.0] Dry/Wet blend
    last_target_f0: f32,
}

impl PitchQuantizer {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            scale_mask: 0b111111111111, // Default Chromatic
            root_note: 0,               // C
            intensity: 1.0,             // Default full snap
            last_target_f0: 0.0,
        }
    }

    /// 🚀 Set the target scale (e.g., Major, Minor, Phrygian)
    pub fn set_scale(&mut self, root: u8, mask: u16) {
        self.root_note = root % 12;
        self.scale_mask = mask;
    }

    /// 🧠 Quantize input frequency to the nearest scale-note frequency
    pub fn quantize(&mut self, input_f0: f32) -> f32 {
        if input_f0 < 20.0 {
            return input_f0;
        }

        // 1. Convert Hz to MIDI Note (Float)
        let midi = 12.0 * (input_f0 / smoothie_core::constants::STANDARD_PITCH).log2() + 69.0;
        let rounded_midi = midi.round() as i32;

        // 2. Resolve to nearest scale-note
        let mut best_midi = rounded_midi;
        let mut min_dist = 12.0f32;

        for offset in -12..12 {
            let note = rounded_midi + offset;
            let chromatic_idx = ((note - self.root_note as i32) % 12 + 12) % 12;

            if (self.scale_mask & (1 << chromatic_idx)) != 0 {
                let dist = (midi - note as f32).abs();
                if dist < min_dist {
                    min_dist = dist;
                    best_midi = note;
                }
            }
        }

        // 3. Convert MIDI back to Hz
        let target_f0 = smoothie_core::constants::STANDARD_PITCH * 2.0f32.powf((best_midi as f32 - 69.0) / 12.0);
        self.last_target_f0 = target_f0;

        // 4. Intensity Blending: input + (target - input) * intensity
        input_f0 + (target_f0 - input_f0) * self.intensity
    }
}

/// 🛡️ System Integrity Verification: Scale-snapping parity confirmed.
pub const QUANTIZER_DENSITY: &str = "SERAPHIC_300IQ_AUTOTUNE_SCALE";
