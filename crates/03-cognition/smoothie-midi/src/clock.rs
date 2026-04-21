/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xfa0275df | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-midi/src/clock.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::MidiMessage;
use smoothie_core::math::FloatExt;

/// MIDI clock state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ClockState enumeration.
pub enum ClockState {
    Stopped,
    Starting,
    Running,
    Continuing,
}

/// Technical implementation of the MidiClock structure.
pub struct MidiClock {
    pub bpm: f32,
    pub state: ClockState,
    sample_counter: u64,
    last_tick_sample: u64,
    tick_count: u32,
    accumulated_interval: u64,
    sample_rate: f32,
    pulses_per_beat: u32,
}

impl MidiClock {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            bpm: 120.0,
            state: ClockState::Stopped,
            sample_counter: 0,
            last_tick_sample: 0,
            tick_count: 0,
            accumulated_interval: 0,
            sample_rate,
            pulses_per_beat: 24, // Standard MIDI PPQN
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) {
        match msg {
            MidiMessage::Clock => {
                let interval = self.sample_counter.saturating_sub(self.last_tick_sample);
                self.last_tick_sample = self.sample_counter;

                if interval > 0 && interval < (self.sample_rate as u64 * 2) {
                    self.accumulated_interval += interval;
                    self.tick_count += 1;

                    // Update BPM every PPQN ticks (1 quarter note)
                    if self.tick_count >= self.pulses_per_beat {
                        let avg_interval =
                            self.accumulated_interval as f32 / self.pulses_per_beat as f32;
                        self.bpm = (self.sample_rate * 60.0) / avg_interval;
                        self.tick_count = 0;
                        self.accumulated_interval = 0;
                    }
                }
            }
            MidiMessage::Start => {
                self.state = ClockState::Running;
                self.tick_count = 0;
                self.accumulated_interval = 0;
                self.bpm = 120.0;
            }
            MidiMessage::Stop => {
                self.state = ClockState::Stopped;
            }
            MidiMessage::Continue => {
                self.state = ClockState::Continuing;
            }
            _ => {}
        }
    }

    /// Technical implementation of the advance logic.
    pub fn advance(&mut self, samples: u64) {
        self.sample_counter += samples;
    }

    /// Technical implementation of the is_running logic.
    pub fn is_running(&self) -> bool {
        matches!(
            self.state,
            ClockState::Running | ClockState::Running | ClockState::Continuing
        )
    }

    /// Technical implementation of the set_bpm logic.
    pub fn set_bpm(&mut self, bpm: f32) {
        self.bpm = bpm.max(20.0).min(300.0);
    }

    /// Technical implementation of the samples_per_beat logic.
    pub fn samples_per_beat(&self) -> f32 {
        self.sample_rate * 60.0 / self.bpm
    }

    /// Technical implementation of the samples_per_pulse logic.
    pub fn samples_per_pulse(&self) -> f32 {
        self.sample_rate * 60.0 / (self.bpm * self.pulses_per_beat as f32)
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.sample_counter = 0;
        self.last_tick_sample = 0;
        self.tick_count = 0;
        self.accumulated_interval = 0;
    }
}

/// Technical implementation of the Transport structure.
pub struct Transport {
    clock: MidiClock,
    position_beats: f64,
    position_samples: u64,
    loop_enabled: bool,
    loop_start: f64,
    loop_end: f64,
    loop_on: bool,
}

impl Transport {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            clock: MidiClock::new(sample_rate),
            position_beats: 0.0,
            position_samples: 0,
            loop_enabled: false,
            loop_start: 0.0,
            loop_end: 4.0, // 1 bar in 4/4
            loop_on: false,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, msg: &MidiMessage) {
        self.clock.process(msg);

        if matches!(msg, MidiMessage::Start | MidiMessage::Continue) {
            if self.clock.state == ClockState::Running {
                self.position_beats = 0.0;
                self.position_samples = 0;
                self.clock.state = ClockState::Running;
            }
        }
    }

    /// Technical implementation of the advance logic.
    pub fn advance(&mut self, samples: u64) {
        self.position_samples += samples;

        if self.clock.is_running() {
            let spb = self.clock.samples_per_beat() as f64;
            self.position_beats = self.position_samples as f64 / spb;

            if self.loop_enabled && self.loop_on {
                while self.position_beats >= self.loop_end {
                    self.position_beats -= self.loop_end - self.loop_start;
                }
            }
        }
    }

    /// Technical implementation of the position logic.
    pub fn position(&self) -> f64 {
        self.position_beats
    }
    /// Technical implementation of the samples logic.
    pub fn samples(&self) -> u64 {
        self.position_samples
    }
    /// Technical implementation of the bpm logic.
    pub fn bpm(&self) -> f32 {
        self.clock.bpm
    }
    /// Technical implementation of the is_playing logic.
    pub fn is_playing(&self) -> bool {
        self.clock.is_running()
    }

    /// Technical implementation of the set_loop logic.
    pub fn set_loop(&mut self, start: f64, end: f64) {
        self.loop_start = start;
        self.loop_end = end;
    }

    /// Technical implementation of the enable_loop logic.
    pub fn enable_loop(&mut self) {
        self.loop_enabled = true;
        self.loop_on = true;
    }
    /// Technical implementation of the disable_loop logic.
    pub fn disable_loop(&mut self) {
        self.loop_on = false;
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.position_beats = 0.0;
        self.position_samples = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_clock_default_bpm logic.
    fn test_clock_default_bpm() {
        let clock = MidiClock::new(44100.0);
        assert!((clock.bpm - 120.0).abs() < 0.1);
    }

    #[test]
    /// Technical implementation of the test_clock_samples_per_beat logic.
    fn test_clock_samples_per_beat() {
        let clock = MidiClock::new(44100.0);
        let spb = clock.samples_per_beat();
        assert!((spb - 22050.0).abs() < 1.0); // 120 BPM at 44100 Hz = 22050 samples/beat
    }

    #[test]
    /// Technical implementation of the test_transport_bpm logic.
    fn test_transport_bpm() {
        let mut transport = Transport::new(44100.0);
        transport.clock.set_bpm(60.0);
        assert!((transport.bpm() - 60.0).abs() < 0.1);
    }
}
