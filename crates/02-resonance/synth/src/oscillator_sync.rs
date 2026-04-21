/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc10c16dc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/synth/src/oscillator_sync.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use smoothie_core::constants::TAU;
use smoothie_core::math::sine_approx;
use smoothie_core::primitives::Sample;

/// Sync type selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SyncType enumeration.
pub enum SyncType {
    Hard,
    Soft,
}

/// Sync oscillator configuration.
#[derive(Clone, Copy, Debug)]
#[repr(align(64))]
/// Technical implementation of the SyncConfig structure.
pub struct SyncConfig {
    pub sync_type: SyncType,
    pub slave_ratio: f32,
    pub hard_reset_level: f32,
}

impl Default for SyncConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            sync_type: SyncType::Hard,
            slave_ratio: 1.0,
            hard_reset_level: 0.5,
        }
    }
}

/// Synced oscillator pair (master → slave).
#[repr(align(64))]
/// Technical implementation of the SyncedOscillator structure.
pub struct SyncedOscillator {
    pub master_phase: f32,
    pub slave_phase: f32,
    pub master_phase_inc: f32,
    config: SyncConfig,
    last_slave_reset: f32,
}

impl SyncedOscillator {
    /// Initializes a new instance of the associated type.
    pub fn new(config: SyncConfig, _sample_rate: f32) -> Self {
        Self {
            master_phase: 0.0,
            slave_phase: 0.0,
            master_phase_inc: 0.0,
            config,
            last_slave_reset: 0.0,
        }
    }

    /// Technical implementation of the set_master_freq logic.
    pub fn set_master_freq(&mut self, freq: f32, sample_rate: f32) {
        self.master_phase_inc = freq / sample_rate;
    }

    /// Technical implementation of the next logic.
    pub fn next(&mut self) -> Sample {
        self.master_phase += self.master_phase_inc;
        if self.master_phase >= 1.0 {
            self.master_phase -= 1.0;
        }

        let master_frac = self.master_phase;
        let slave_inc = self.master_phase_inc * self.config.slave_ratio;

        match self.config.sync_type {
            SyncType::Hard => {
                if master_frac < self.last_slave_reset {
                    self.slave_phase = 0.0;
                }
                self.last_slave_reset = master_frac;
            }
            SyncType::Soft => {
                if master_frac < 0.01 && self.last_slave_reset > 0.99 {
                    self.slave_phase = 0.0;
                }
                self.last_slave_reset = master_frac;
            }
        }

        self.slave_phase += slave_inc;
        if self.slave_phase >= 1.0 {
            self.slave_phase -= 1.0;
        }

        sine_approx(self.slave_phase * TAU)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.master_phase = 0.0;
        self.slave_phase = 0.0;
    }
}
