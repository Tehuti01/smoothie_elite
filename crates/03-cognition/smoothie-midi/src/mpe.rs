/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5b117f0a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/mpe.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::{MidiMessage, NoteTracker, NUM_NOTES};
use smoothie_core::math::FloatExt;

/// MPE configuration
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the MpeConfig structure.
pub struct MpeConfig {
    /// Chief Engineer channel (where global controls are received)
    pub master_channel: u8,
    /// Number of per-note channels (max 15)
    pub zone_size: u8,
    /// Per-note pitch bend range in semitones
    pub pitch_bend_range: u8,
    /// Enable MPE mode
    pub enabled: bool,
}

impl Default for MpeConfig {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            master_channel: 0,
            zone_size: 15,
            pitch_bend_range: 48,
            enabled: false,
        }
    }
}

/// Per-note MPE state (pitch bend, pressure, timbre)
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the MpeNoteState structure.
pub struct MpeNoteState {
    /// Per-note pitch bend (14-bit, centered at 8192)
    pub pitch_bend: u16,
    /// Per-note aftertouch/pressure (0-127)
    pub pressure: u8,
    /// Per-note velocity (initial attack velocity)
    pub velocity: u8,
    /// Timbre (CC1 - modulation wheel)
    pub timbre: u8,
}

impl Default for MpeNoteState {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            pitch_bend: 8192,
            pressure: 0,
            velocity: 0,
            timbre: 0,
        }
    }
}

/// Technical implementation of the MpeZone structure.
pub struct MpeZone {
    config: MpeConfig,
    /// Per-note state (index = note number)
    note_states: [MpeNoteState; NUM_NOTES],
    /// Active notes tracker
    note_tracker: NoteTracker,
}

impl MpeZone {
    /// Initializes a new instance of the associated type.
    pub fn new(config: MpeConfig) -> Self {
        Self {
            config,
            note_states: [MpeNoteState::default(); NUM_NOTES],
            note_tracker: NoteTracker::new(),
        }
    }

    /// Technical implementation of the config logic.
    pub fn config(&self) -> MpeConfig {
        self.config
    }

    /// Technical implementation of the enable logic.
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }
    /// Technical implementation of the disable logic.
    pub fn disable(&mut self) {
        self.config.enabled = false;
    }
    /// Technical implementation of the is_enabled logic.
    pub fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) {
        if !self.config.enabled {
            return;
        }

        match msg {
            MidiMessage::NoteOn {
                channel,
                note,
                velocity,
            } => {
                if *channel == self.config.master_channel {
                    // Chief Engineer channel note on - allocate to first free per-note channel
                    let n = *note as usize;
                    if n < NUM_NOTES {
                        self.note_tracker.process(msg);
                        self.note_states[n].velocity = *velocity;
                        self.note_states[n].pitch_bend = 8192;
                        self.note_states[n].pressure = 0;
                    }
                } else if *channel > self.config.master_channel
                    && *channel <= self.config.master_channel + self.config.zone_size
                {
                    // Per-note channel - this is the "note number" for MPE
                    self.note_tracker.process(msg);
                }
            }
            MidiMessage::NoteOff { channel, note, .. } => {
                let n = *note as usize;
                if n < NUM_NOTES {
                    self.note_tracker.process(msg);
                    if *channel == self.config.master_channel {
                        self.note_states[n] = MpeNoteState::default();
                    }
                }
            }
            MidiMessage::PitchBend { channel, value } => {
                if *channel == self.config.master_channel {
                    // Global pitch bend - apply to all notes
                    for note_state in self.note_states.iter_mut() {
                        note_state.pitch_bend = *value;
                    }
                }
            }
            MidiMessage::Aftertouch {
                channel,
                note,
                value,
            } => {
                let n = *note as usize;
                if n < NUM_NOTES && *channel != self.config.master_channel {
                    self.note_states[n].pressure = *value;
                }
            }
            MidiMessage::ControlChange {
                channel,
                controller,
                value,
            } => {
                // CC1 (modulation) = timbre, CC74 = volume-like
                if *controller == 1 && *channel != self.config.master_channel {
                    // Need to track note-channel mapping; for now use last_note
                    let n = self.note_tracker.last_note() as usize;
                    if n < NUM_NOTES {
                        self.note_states[n].timbre = *value;
                    }
                }
            }
            _ => {}
        }
    }

    /// Get the pitch bend ratio for a specific note
    pub fn note_pitch_ratio(&self, note: u8) -> f32 {
        let n = note as usize;
        if n >= NUM_NOTES {
            return 1.0;
        }
        let state = &self.note_states[n];
        let semitones =
            (state.pitch_bend as f32 - 8192.0) / 8192.0 * self.config.pitch_bend_range as f32;
        mpe_pow2_approx(semitones / 12.0)
    }

    /// Get the pressure for a specific note
    pub fn note_pressure(&self, note: u8) -> u8 {
        if (note as usize) < NUM_NOTES {
            self.note_states[note as usize].pressure
        } else {
            0
        }
    }

    /// Get the timbre for a specific note
    pub fn note_timbre(&self, note: u8) -> u8 {
        if (note as usize) < NUM_NOTES {
            self.note_states[note as usize].timbre
        } else {
            0
        }
    }

    /// Clear all note states (all notes off)
    pub fn clear(&mut self) {
        self.note_states = [MpeNoteState::default(); NUM_NOTES];
        self.note_tracker.clear();
    }
}

/// Technical implementation of the MpeManager structure.
pub struct MpeManager {
    zones: [Option<MpeZone>; 2], // Lower and upper zones
    /// Current active zone
    active_zone: usize,
}

impl MpeManager {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            zones: [None, None],
            active_zone: 0,
        }
    }

    /// Create a new MPE zone (0 = lower, 1 = upper)
    pub fn create_zone(&mut self, zone_id: usize, config: MpeConfig) {
        if zone_id < 2 {
            self.zones[zone_id] = Some(MpeZone::new(config));
        }
    }

    /// Process a message through MPE
    pub fn process(&mut self, msg: &MidiMessage) {
        if let Some(ref mut zone) = self.zones[self.active_zone] {
            zone.process(msg);
        }
        // Also process through other zone if message matches
        let other = if self.active_zone == 0 { 1 } else { 0 };
        if let Some(ref mut zone) = self.zones[other] {
            if let Some(ch) = msg.channel() {
                if ch == zone.config().master_channel
                    || (ch > zone.config().master_channel
                        && ch <= zone.config().master_channel + zone.config().zone_size)
                {
                    zone.process(msg);
                }
            }
        }
    }

    /// Get current MPE zone
    pub fn active_zone_mut(&mut self) -> Option<&mut MpeZone> {
        self.zones[self.active_zone].as_mut()
    }

    /// Get current MPE zone
    pub fn active_zone(&self) -> Option<&MpeZone> {
        self.zones[self.active_zone].as_ref()
    }

    /// Switch active zone
    pub fn set_active_zone(&mut self, zone_id: usize) {
        if zone_id < 2 {
            self.active_zone = zone_id;
        }
    }
}

impl Default for MpeManager {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
/// Technical implementation of the mpe_pow2_approx logic.
pub fn mpe_pow2_approx(x: f32) -> f32 {
    x.exp() // Simplified for now
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_mpe_config_default logic.
    fn test_mpe_config_default() {
        let config = MpeConfig::default();
        assert_eq!(config.master_channel, 0);
        assert_eq!(config.zone_size, 15);
        assert_eq!(config.pitch_bend_range, 48);
        assert!(!config.enabled);
    }

    #[test]
    /// Technical implementation of the test_mpe_zone_enable logic.
    fn test_mpe_zone_enable() {
        let mut zone = MpeZone::new(MpeConfig::default());
        assert!(!zone.is_enabled());
        zone.enable();
        assert!(zone.is_enabled());
    }

    #[test]
    /// Technical implementation of the test_mpe_note_state_default logic.
    fn test_mpe_note_state_default() {
        let state = MpeNoteState::default();
        assert_eq!(state.pitch_bend, 8192);
        assert_eq!(state.pressure, 0);
    }

    #[test]
    /// Technical implementation of the test_mpe_manager logic.
    fn test_mpe_manager() {
        let mut manager = MpeManager::new();
        manager.create_zone(
            0,
            MpeConfig {
                enabled: true,
                ..Default::default()
            },
        );
        assert!(manager.active_zone().is_some());
    }
}
