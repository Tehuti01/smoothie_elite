//! Lock-free MIDI event buffer for the audio thread.

use crate::event::MidiEvent;

/// A fixed-capacity MIDI event buffer — no heap allocation.
pub struct MidiBuffer {
    events: [Option<MidiEvent>; 256],
    count:  usize,
}

impl MidiBuffer {
    pub fn new() -> Self {
        Self { events: [None; 256], count: 0 }
    }

    pub fn push(&mut self, event: MidiEvent) -> bool {
        if self.count < 256 {
            self.events[self.count] = Some(event);
            self.count += 1;
            true
        } else {
            false
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &MidiEvent> {
        self.events[..self.count].iter().filter_map(|e| e.as_ref())
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn len(&self) -> usize { self.count }
    pub fn is_empty(&self) -> bool { self.count == 0 }
}

impl Default for MidiBuffer {
    fn default() -> Self { Self::new() }
}
