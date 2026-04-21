/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x3e958f56 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/nodes/source.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::math::FloatExt;
///
/// Defines different node behaviors in the processing graph.

extern crate alloc;

use crate::port::PortDirection;
use core::sync::atomic::{AtomicU32, Ordering};

pub const MAX_PORTS: usize = 8;

pub trait NodeType: Send {
    /// Technical implementation of the node_type logic.
    fn node_type(&self) -> NodeClassification;
    /// Primary real-time signal processing execution block.
    fn process(&mut self, inputs: &[&[f32]], outputs: &mut [&mut [f32]]);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the NodeClassification enumeration.
pub enum NodeClassification {
    Source,
    Process,
    Sink,
    Modifier,
    Router,
    Splitter,
}

/// Technical implementation of the SourceNode structure.
pub struct SourceNode {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
    amplitude: f32,
}

impl SourceNode {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            frequency: smoothie_core::constants::STANDARD_PITCH,
            phase: 0.0,
            amplitude: 1.0,
        }
    }

    /// Technical implementation of the with_frequency logic.
    pub fn with_frequency(mut self, freq: f32) -> Self {
        self.frequency = freq;
        self
    }

    /// Technical implementation of the with_amplitude logic.
    pub fn with_amplitude(mut self, amp: f32) -> Self {
        self.amplitude = amp;
        self
    }

    /// Technical implementation of the set_frequency logic.
    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    /// Technical implementation of the set_amplitude logic.
    pub fn set_amplitude(&mut self, amp: f32) {
        self.amplitude = amp;
    }
}

impl NodeType for SourceNode {
    /// Technical implementation of the node_type logic.
    fn node_type(&self) -> NodeClassification {
        NodeClassification::Source
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, _inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        let phase_increment = self.frequency / self.sample_rate;
        for output in outputs.iter_mut() {
            for sample in output.iter_mut() {
                *sample = self.amplitude * (self.phase * core::f32::consts::TAU).sin();
                self.phase += phase_increment;
                if self.phase >= 1.0 {
                    self.phase -= 1.0;
                }
            }
        }
    }
}

impl Default for SourceNode {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the ProcessNode structure.
pub struct ProcessNode {
    sample_rate: f32,
    gain: f32,
    mix: f32,
}

impl ProcessNode {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            gain: 1.0,
            mix: 1.0,
        }
    }

    /// Technical implementation of the with_gain logic.
    pub fn with_gain(mut self, gain: f32) -> Self {
        self.gain = gain;
        self
    }

    /// Technical implementation of the with_mix logic.
    pub fn with_mix(mut self, mix: f32) -> Self {
        self.mix = mix;
        self
    }

    /// Technical implementation of the set_gain logic.
    pub fn set_gain(&mut self, gain: f32) {
        self.gain = gain;
    }

    /// Technical implementation of the set_mix logic.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }
}

impl NodeType for ProcessNode {
    /// Technical implementation of the node_type logic.
    fn node_type(&self) -> NodeClassification {
        NodeClassification::Process
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
        let wet = self.mix;
        let dry = 1.0 - wet;
        for (input_ch, output_ch) in inputs.iter().zip(outputs.iter_mut()) {
            for (i, o) in input_ch.iter().zip(output_ch.iter_mut()) {
                *o = *i * self.gain;
            }
        }
    }
}

impl Default for ProcessNode {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

/// Technical implementation of the SinkNode structure.
pub struct SinkNode {
    sample_rate: f32,
    peak: AtomicU32,
    rms: AtomicU32,
    peak_hold: AtomicU32,
    peak_decay: f32,
}

impl SinkNode {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            sample_rate: 44100.0,
            peak: AtomicU32::new(0.0_f32.to_bits()),
            rms: AtomicU32::new(0.0_f32.to_bits()),
            peak_hold: AtomicU32::new(0.0_f32.to_bits()),
            peak_decay: 0.999,
        }
    }

    /// Technical implementation of the get_peak logic.
    pub fn get_peak(&self) -> f32 {
        f32::from_bits(self.peak.load(Ordering::Relaxed))
    }

    /// Technical implementation of the get_rms logic.
    pub fn get_rms(&self) -> f32 {
        f32::from_bits(self.rms.load(Ordering::Relaxed))
    }

    /// Technical implementation of the get_peak_hold logic.
    pub fn get_peak_hold(&self) -> f32 {
        f32::from_bits(self.peak_hold.load(Ordering::Relaxed))
    }
}

impl NodeType for SinkNode {
    /// Technical implementation of the node_type logic.
    fn node_type(&self) -> NodeClassification {
        NodeClassification::Sink
    }

    /// Primary real-time signal processing execution block.
    fn process(&mut self, inputs: &[&[f32]], _outputs: &mut [&mut [f32]]) {
        let mut peak = 0.0f32;
        let mut sum = 0.0f32;
        let mut count = 0;

        for input in inputs {
            for &sample in input.iter() {
                let abs = sample.abs();
                if abs > peak {
                    peak = abs;
                }
                sum += sample * sample;
                count += 1;
            }
        }

        let rms = if count > 0 {
            (sum / count as f32).sqrt()
        } else {
            0.0
        };

        self.peak.store(peak.to_bits(), Ordering::Relaxed);
        self.rms.store(rms.to_bits(), Ordering::Relaxed);
    }
}

impl Default for SinkNode {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_source_node logic.
    fn test_source_node() {
        let mut node = SourceNode::new();
        let mut output = vec![0.0f32; 128];
        node.process(&[], &mut [&mut output]);
        assert!(output.iter().any(|&s| s != 0.0));
    }
}
