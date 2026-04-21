/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x4b7e02dc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-params/src/automation.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;
use smoothie_core::math::FloatExt;

use alloc::vec::Vec;

pub const MAX_AUTOMATION_POINTS: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the AutomationPoint structure.
pub struct AutomationPoint {
    pub time: f64,
    pub value: f32,
}

impl AutomationPoint {
    /// Initializes a new instance of the associated type.
    pub const fn new(time: f64, value: f32) -> Self {
        Self { time, value }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the AutomationState enumeration.
pub enum AutomationState {
    Stopped,
    Recording,
    Playing,
    Writing,
}

/// Technical implementation of the AutomationLane structure.
pub struct AutomationLane {
    pub param_index: usize,
    points: Vec<AutomationPoint>,
    state: AutomationState,
    record_time: f64,
    loop_start: f64,
    loop_end: f64,
    loop_enabled: bool,
}

impl AutomationLane {
    /// Initializes a new instance of the associated type.
    pub fn new(param_index: usize) -> Self {
        Self {
            param_index,
            points: Vec::with_capacity(MAX_AUTOMATION_POINTS),
            state: AutomationState::Stopped,
            record_time: 0.0,
            loop_start: 0.0,
            loop_end: 0.0,
            loop_enabled: false,
        }
    }

    /// Technical implementation of the start_recording logic.
    pub fn start_recording(&mut self, time: f64) {
        self.state = AutomationState::Recording;
        self.record_time = time;
    }

    /// Technical implementation of the stop_recording logic.
    pub fn stop_recording(&mut self) {
        if self.state == AutomationState::Recording {
            self.state = AutomationState::Stopped;
            self.points
                .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        }
    }

    /// Technical implementation of the start_playback logic.
    pub fn start_playback(&mut self, time: f64) {
        self.record_time = time;
        self.state = AutomationState::Playing;
    }

    /// Technical implementation of the stop_playback logic.
    pub fn stop_playback(&mut self) {
        self.state = AutomationState::Stopped;
    }

    /// Performs vector addition logic.
    pub fn add_point(&mut self, time: f64, value: f32) {
        if self.points.len() < MAX_AUTOMATION_POINTS {
            self.points.push(AutomationPoint::new(time, value));
        }
    }

    /// Technical implementation of the set_loop logic.
    pub fn set_loop(&mut self, start: f64, end: f64) {
        self.loop_start = start;
        self.loop_end = end;
        self.loop_enabled = true;
    }

    /// Technical implementation of the get_value_at logic.
    pub fn get_value_at(&self, time: f64) -> f32 {
        if self.points.is_empty() {
            return 0.5;
        }

        let t = if self.loop_enabled && time > self.loop_end {
            let loop_len = self.loop_end - self.loop_start;
            if loop_len > 0.0 {
                let offset = (time - self.loop_start) % loop_len;
                self.loop_start + offset
            } else {
                time
            }
        } else {
            time
        };

        let idx = self
            .points
            .binary_search_by(|p| p.time.partial_cmp(&t).unwrap());

        match idx {
            Ok(i) => self.points[i].value,
            Err(i) if i == 0 => self.points[0].value,
            Err(i) if i >= self.points.len() => self.points[self.points.len() - 1].value,
            Err(i) => {
                let prev = &self.points[i - 1];
                let next = &self.points[i];
                let t_range = next.time - prev.time;
                if t_range == 0.0 {
                    prev.value
                } else {
                    let t_factor = (t - prev.time) / t_range;
                    prev.value + (next.value - prev.value) * t_factor as f32
                }
            }
        }
    }

    /// Technical implementation of the clear logic.
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Technical implementation of the point_count logic.
    pub fn point_count(&self) -> usize {
        self.points.len()
    }
}

impl Default for AutomationLane {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0)
    }
}

pub const MAX_PARAM_AUTOMATION: usize = 128;

/// Technical implementation of the ParameterAutomation structure.
pub struct ParameterAutomation {
    lanes: [Option<AutomationLane>; MAX_PARAM_AUTOMATION],
    active_lanes: usize,
}

impl ParameterAutomation {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            lanes: [None; MAX_PARAM_AUTOMATION],
            active_lanes: 0,
        }
    }

    /// Technical implementation of the get_or_create_lane logic.
    pub fn get_or_create_lane(&mut self, param_index: usize) -> &mut AutomationLane {
        if self.lanes[param_index].is_none() {
            self.lanes[param_index] = Some(AutomationLane::new(param_index));
            self.active_lanes += 1;
        }
        self.lanes[param_index].as_mut().unwrap()
    }

    /// Technical implementation of the get_lane logic.
    pub fn get_lane(&self, param_index: usize) -> Option<&AutomationLane> {
        self.lanes[param_index].as_ref()
    }

    /// Technical implementation of the all_stopped logic.
    pub fn all_stopped(&self) -> bool {
        for lane in &self.lanes {
            if let Some(ref l) = lane {
                if l.state != AutomationState::Stopped {
                    return false;
                }
            }
        }
        true
    }
}

impl Default for ParameterAutomation {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_automation_lane logic.
    fn test_automation_lane() {
        let mut lane = AutomationLane::new(0);
        lane.add_point(0.0, 0.0);
        lane.add_point(1.0, 1.0);
        assert_eq!(lane.point_count(), 2);
        assert!((lane.get_value_at(0.5) - 0.5).abs() < 0.01);
    }
}
