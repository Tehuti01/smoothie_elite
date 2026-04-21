/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xebcad3b1 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/learn.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::{cc, MidiMessage};
use smoothie_core::math::FloatExt;

/// Learn state for a single parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the LearnState enumeration.
pub enum LearnState {
    Inactive,
    Waiting,  // Waiting for first CC message
    Armed,    // Received first CC, waiting for release
    Assigned, // CC assigned to parameter
}

/// Learn entry for a parameter
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the LearnEntry structure.
pub struct LearnEntry {
    pub state: LearnState,
    pub cc_number: u8,
    pub channel: u8,
    pub min_value: u8,
    pub max_value: u8,
}

impl Default for LearnEntry {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            state: LearnState::Inactive,
            cc_number: 0,
            channel: 0,
            min_value: 0,
            max_value: 127,
        }
    }
}

impl LearnEntry {
    /// Technical implementation of the const_default logic.
    pub const fn const_default() -> Self {
        Self {
            state: LearnState::Inactive,
            cc_number: 0,
            channel: 0,
            min_value: 0,
            max_value: 127,
        }
    }
}

/// Technical implementation of the MidiLearn structure.
pub struct MidiLearn {
    /// Learn state per parameter (index = parameter index)
    entries: [LearnEntry; 64],
    /// Current parameter being learned
    learning_param: Option<usize>,
    /// Timeout for learn mode (in samples)
    timeout_samples: u64,
    /// Sample counter
    sample_counter: u64,
    /// Global enabled flag
    enabled: bool,
}

impl MidiLearn {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            entries: [LearnEntry::const_default(); 64],
            learning_param: None,
            timeout_samples: 0,
            sample_counter: 0,
            enabled: true,
        }
    }

    /// Start learning for a parameter
    pub fn start_learning(&mut self, param_index: usize) {
        if param_index < 64 {
            self.learning_param = Some(param_index);
            self.entries[param_index].state = LearnState::Waiting;
            self.timeout_samples = self.sample_counter + (44100 * 10); // 10 seconds default
        }
    }

    /// Cancel learning
    pub fn cancel_learning(&mut self) {
        if let Some(idx) = self.learning_param {
            self.entries[idx].state = LearnState::Inactive;
        }
        self.learning_param = None;
    }

    /// Stop learning and assign CC
    pub fn assign_cc(&mut self, param_index: usize, cc: u8, channel: u8) {
        if param_index < 64 {
            self.entries[param_index].state = LearnState::Assigned;
            self.entries[param_index].cc_number = cc;
            self.entries[param_index].channel = channel;
        }
        self.learning_param = None;
    }

    /// Process a MIDI message for learn
    pub fn process(&mut self, msg: &MidiMessage) -> Option<LearnEvent> {
        if !self.enabled {
            return None;
        }

        match msg {
            MidiMessage::ControlChange {
                channel,
                controller,
                value,
            } => {
                // Check if we're learning for this CC
                if let Some(param_idx) = self.learning_param {
                    let entry = &mut self.entries[param_idx];

                    match entry.state {
                        LearnState::Waiting => {
                            // First CC received
                            entry.state = LearnState::Armed;
                            entry.cc_number = *controller;
                            entry.channel = *channel;
                            entry.min_value = *value;
                            entry.max_value = *value;
                            return Some(LearnEvent::FirstValue {
                                param: param_idx,
                                cc: *controller,
                                value: *value,
                            });
                        }
                        LearnState::Armed => {
                            // Update min/max
                            entry.min_value = entry.min_value.min(*value);
                            entry.max_value = entry.max_value.max(*value);
                            return Some(LearnEvent::ValueUpdate {
                                param: param_idx,
                                value: *value,
                            });
                        }
                        _ => {}
                    }
                }

                // Check for existing assignments
                for (idx, entry) in self.entries.iter().enumerate() {
                    if entry.state == LearnState::Assigned
                        && entry.cc_number == *controller
                        && entry.channel == *channel
                    {
                        // Normalize value to 0.0-1.0 range
                        let normalized = if entry.max_value > entry.min_value {
                            (*value as f32 - entry.min_value as f32)
                                / (entry.max_value - entry.min_value) as f32
                        } else {
                            *value as f32 / 127.0
                        };
                        return Some(LearnEvent::MappedValue {
                            param: idx,
                            value: normalized,
                            raw: *value,
                        });
                    }
                }
            }
            _ => {}
        }

        None
    }

    /// Check for learn timeout
    pub fn check_timeout(&mut self) -> Option<LearnEvent> {
        if let Some(idx) = self.learning_param {
            if self.sample_counter > self.timeout_samples {
                self.entries[idx].state = LearnState::Inactive;
                let evt = LearnEvent::Timeout { param: idx };
                self.learning_param = None;
                return Some(evt);
            }
        }
        None
    }

    /// Technical implementation of the advance logic.
    pub fn advance(&mut self, samples: u64) {
        self.sample_counter += samples;
    }

    /// Get the assigned CC for a parameter
    pub fn get_cc(&self, param_index: usize) -> Option<(u8, u8)> {
        if param_index < 64 {
            let entry = &self.entries[param_index];
            if entry.state == LearnState::Assigned {
                Some((entry.cc_number, entry.channel))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Clear assignment for a parameter
    pub fn clear_assignment(&mut self, param_index: usize) {
        if param_index < 64 {
            self.entries[param_index] = LearnEntry::default();
        }
    }

    /// Clear all assignments
    pub fn clear_all(&mut self) {
        self.entries = [LearnEntry::default(); 64];
        self.learning_param = None;
    }

    /// Technical implementation of the is_enabled logic.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Technical implementation of the set_enabled logic.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

/// MIDI Learn events
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the LearnEvent enumeration.
pub enum LearnEvent {
    FirstValue { param: usize, cc: u8, value: u8 },
    ValueUpdate { param: usize, value: u8 },
    MappedValue { param: usize, value: f32, raw: u8 },
    Timeout { param: usize },
}

/// Technical implementation of the SustainPedal structure.
pub struct SustainPedal {
    /// Current sustain state (pressed = true)
    is_on: bool,
    /// Pending note-offs (to be sent when sustain released)
    pending_notes: [bool; 128],
    pending_count: usize,
}

impl SustainPedal {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            is_on: false,
            pending_notes: [false; 128],
            pending_count: 0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) -> Option<MidiMessage> {
        match msg {
            MidiMessage::ControlChange {
                controller, value, ..
            } => {
                if *controller == cc::SUSTAIN {
                    let was_on = self.is_on;
                    self.is_on = *value >= 64;

                    // Sustain released - release pending notes
                    if was_on && !self.is_on {
                        let mut output = None;
                        for (i, pending) in self.pending_notes.iter_mut().enumerate() {
                            if *pending {
                                *pending = false;
                                output = Some(MidiMessage::NoteOff {
                                    channel: 0,
                                    note: i as u8,
                                    velocity: 0,
                                });
                                // Only return one note-off per call
                                break;
                            }
                        }
                        return output;
                    }
                } else if *controller == cc::ALL_NOTES_OFF {
                    // All notes off - clear pending
                    self.clear();
                }
            }
            MidiMessage::NoteOff { note, .. } if self.is_on => {
                // Hold note-off until sustain released
                let n = *note as usize;
                if n < 128 && !self.pending_notes[n] {
                    self.pending_notes[n] = true;
                    self.pending_count += 1;
                }
            }
            MidiMessage::NoteOn { note, .. } => {
                // Note on while sustaining - remove from pending if there
                let n = *note as usize;
                if n < 128 && self.pending_notes[n] {
                    self.pending_notes[n] = false;
                    self.pending_count = self.pending_count.saturating_sub(1);
                }
            }
            _ => {}
        }
        None
    }

    /// Technical implementation of the is_active logic.
    pub fn is_active(&self) -> bool {
        self.is_on
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.pending_notes = [false; 128];
        self.pending_count = 0;
    }
}

/// Technical implementation of the FootController structure.
pub struct FootController {
    value: u8,
    target: u8,
}

impl FootController {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            value: 0,
            target: 0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) {
        if let MidiMessage::ControlChange {
            controller, value, ..
        } = msg
        {
            if *controller == cc::FOOT {
                self.target = *value;
            }
        }
    }

    /// Technical implementation of the value logic.
    pub fn value(&self) -> u8 {
        self.value
    }
    /// Technical implementation of the target logic.
    pub fn target(&self) -> u8 {
        self.target
    }

    /// Get smoothed value (call once per block)
    pub fn advance(&mut self) {
        if self.value < self.target {
            self.value = self.value.saturating_add(1);
        } else if self.value > self.target {
            self.value = self.value.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_midi_learn_new logic.
    fn test_midi_learn_new() {
        let learn = MidiLearn::new();
        assert!(learn.learning_param.is_none());
    }

    #[test]
    /// Technical implementation of the test_midi_learn_start logic.
    fn test_midi_learn_start() {
        let mut learn = MidiLearn::new();
        learn.start_learning(5);
        assert_eq!(learn.learning_param, Some(5));
    }

    #[test]
    /// Technical implementation of the test_sustain_pedal logic.
    fn test_sustain_pedal() {
        let mut sustain = SustainPedal::new();

        // Press sustain
        sustain.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::SUSTAIN,
            value: 127,
        });
        assert!(sustain.is_active());

        // Note off while sustained
        sustain.process(&MidiMessage::NoteOff {
            channel: 0,
            note: 60,
            velocity: 0,
        });

        // Release sustain - should get note off back
        let msg = sustain.process(&MidiMessage::ControlChange {
            channel: 0,
            controller: cc::SUSTAIN,
            value: 0,
        });
        assert!(msg.is_some());
    }
}
