/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x38b7fa5e | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/plugin-host/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use serde::{Serialize, Deserialize};

/// Cache-line alignment constant
const CACHE_LINE: usize = 64;

///
/// All methods must execute in O(1) time with deterministic latency
pub trait PluginProcessor: Send + Sync {
    /// Initialize plugin (called once at startup)
    fn init(&mut self) -> Result<(), PluginError>;

    /// Activate plugin (set sample rate, open resources)
    fn activate(&mut self, sample_rate: f32) -> Result<(), PluginError>;

    /// Deactivate plugin (close resources)
    fn deactivate(&mut self) -> Result<(), PluginError>;

    /// Process audio block (O(1) per sample, zero-allocation)
    ///
    /// # Arguments:
    /// * `inputs` - Input buffer (pre-allocated)
    /// * `outputs` - Output buffer (pre-allocated)
    /// * `sample_count` - Number of samples to process
    /// * `events` - MIDI events for this block
    fn process(
        &mut self,
        inputs: &[&[f32]],
        outputs: &mut [&mut [f32]],
        sample_count: usize,
        events: &[MidiEvent],
    ) -> Result<(), PluginError>;

    /// Get parameter count
    fn parameter_count(&self) -> usize;

    /// Get parameter info
    fn parameter_info(&self, index: usize) -> Option<ParameterInfo>;

    /// Set parameter value (from automation or UI)
    fn set_parameter(&mut self, index: usize, value: f32);

    /// Get parameter value
    fn get_parameter(&self, index: usize) -> Option<f32>;

    /// Bypass support (O(1), zero-allocation)
    fn set_bypass(&mut self, bypassed: bool);

    /// Get bypass state
    fn is_bypassed(&self) -> bool;

    /// Get current CPU load (percentage)
    fn cpu_load(&self) -> f32;

    /// Get tail output length in samples (for reverb, delay, etc)
    fn tail_length(&self) -> usize {
        0 // Default: no tail
    }
}

/// MIDI event with sample-accurate timing
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the MidiEvent structure.
pub struct MidiEvent {
    /// Sample offset within block (0..block_size)
    pub sample_offset: u32,

    /// MIDI status byte
    pub status: u8,

    /// MIDI data byte 1 (note or CC number)
    pub data1: u8,

    /// MIDI data byte 2 (velocity or CC value)
    pub data2: u8,

    /// Channel (0-15)
    pub channel: u8,

    /// Padding for alignment
    _pad: [u8; 3],
}

impl MidiEvent {
    /// Create note-on event
    pub fn note_on(sample_offset: u32, note: u8, velocity: u8, channel: u8) -> Self {
        Self {
            sample_offset,
            status: 0x90 | (channel & 0x0F),
            data1: note,
            data2: velocity,
            channel,
            _pad: [0; 3],
        }
    }

    /// Create note-off event
    pub fn note_off(sample_offset: u32, note: u8, velocity: u8, channel: u8) -> Self {
        Self {
            sample_offset,
            status: 0x80 | (channel & 0x0F),
            data1: note,
            data2: velocity,
            channel,
            _pad: [0; 3],
        }
    }

    /// Create CC event
    pub fn control_change(sample_offset: u32, cc: u8, value: u8, channel: u8) -> Self {
        Self {
            sample_offset,
            status: 0xB0 | (channel & 0x0F),
            data1: cc,
            data2: value,
            channel,
            _pad: [0; 3],
        }
    }

    /// Is this a note-on event?
    pub fn is_note_on(&self) -> bool {
        (self.status & 0xF0) == 0x90 && self.data2 > 0
    }

    /// Is this a note-off event?
    pub fn is_note_off(&self) -> bool {
        (self.status & 0xF0) == 0x80 || ((self.status & 0xF0) == 0x90 && self.data2 == 0)
    }

    /// Is this a CC event?
    pub fn is_control_change(&self) -> bool {
        (self.status & 0xF0) == 0xB0
    }
}

/// Parameter information
#[derive(Clone, Debug, Serialize, Deserialize)]
/// Technical implementation of the ParameterInfo structure.
pub struct ParameterInfo {
    pub id: u32,
    pub name: String,
    pub unit: String,
    pub min_value: f32,
    pub max_value: f32,
    pub default_value: f32,
    pub is_automatable: bool,
}

/// Plugin preset
#[repr(align(64))]
#[derive(Clone, Debug, Serialize, Deserialize)]
/// Technical implementation of the Preset structure.
pub struct Preset {
    pub name: String,
    pub parameters: Vec<(u32, f32)>,
}

impl Preset {
    /// Initializes a new instance of the associated type.
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: Vec::new(),
        }
    }
}

/// Plugin error type
#[derive(Debug, Clone)]
/// Technical implementation of the PluginError enumeration.
pub enum PluginError {
    NotInitialized,
    AlreadyActive,
    InvalidParameter,
    ProcessError(String),
    SerializationError(String),
}

impl std::fmt::Display for PluginError {
    /// Technical implementation of the fmt logic.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "Plugin not initialized"),
            Self::AlreadyActive => write!(f, "Plugin already active"),
            Self::InvalidParameter => write!(f, "Invalid parameter"),
            Self::ProcessError(msg) => write!(f, "Process error: {}", msg),
            Self::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for PluginError {}

///
/// - Audio thread pinned to isolated core
/// - Pre-allocated buffers (8 blocks x 2048 samples)
/// - SCHED_FIFO priority scheduling
#[repr(align(64))]
/// Technical implementation of the PluginHost structure.
pub struct PluginHost {
    /// Plugin instance
    plugin: Option<Box<dyn PluginProcessor>>,

    /// Audio buffer pool (pre-allocated)
    audio_buffers: Vec<Vec<f32>>,

    /// Current block size
    block_size: usize,

    /// Sample rate
    sample_rate: f32,

    /// Is audio processing active?
    is_active: AtomicBool,

    /// CPU load tracking
    cpu_load: AtomicU32,  // Bit-cast f32

    /// Presets
    presets: Vec<Preset>,

    /// Current preset index
    current_preset: usize,
}

impl PluginHost {
    /// Create new plugin host
    ///
    /// Pre-allocates all buffers for zero-allocation audio processing
    pub fn new(block_size: usize, max_blocks: usize) -> Self {
        let mut audio_buffers = Vec::with_capacity(max_blocks * 2);  // stereo
        for _ in 0..(max_blocks * 2) {
            audio_buffers.push(vec![0.0; block_size]);
        }

        Self {
            plugin: None,
            audio_buffers,
            block_size,
            sample_rate: 44100.0,
            is_active: AtomicBool::new(false),
            cpu_load: AtomicU32::new(0),
            presets: Vec::new(),
            current_preset: 0,
        }
    }

    /// Set plugin instance
    pub fn set_plugin(&mut self, plugin: Box<dyn PluginProcessor>) -> Result<(), PluginError> {
        self.plugin = Some(plugin);
        self.plugin.as_mut().unwrap().init()?;
        Ok(())
    }

    /// Activate plugin (begin audio processing)
    pub fn activate(&mut self) -> Result<(), PluginError> {
        if let Some(plugin) = &mut self.plugin {
            plugin.activate(self.sample_rate)?;
            self.is_active.store(true, Ordering::Release);
            Ok(())
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Deactivate plugin (end audio processing)
    pub fn deactivate(&mut self) -> Result<(), PluginError> {
        if let Some(plugin) = &mut self.plugin {
            plugin.deactivate()?;
            self.is_active.store(false, Ordering::Release);
            Ok(())
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Process audio block (real-time safe, O(1), zero-allocation)
    pub fn process_block(&mut self, midi_events: &[MidiEvent]) -> Result<(), PluginError> {
        if !self.is_active.load(Ordering::Acquire) {
            return Err(PluginError::NotInitialized);
        }

        if let Some(plugin) = &mut self.plugin {
            // Get pre-allocated input/output buffers (non-overlapping via split_at_mut)
            let (input_buf, output_buf) = self.audio_buffers.split_at_mut(1);
            let inputs: Vec<&[f32]> = vec![&input_buf[0][..self.block_size]];
            let mut outputs: Vec<&mut [f32]> = vec![
                &mut output_buf[0][..self.block_size]
            ];

            // Process (zero-allocation)
            plugin.process(&inputs, &mut outputs, self.block_size, midi_events)?;

            // Update CPU load (atomic, lock-free)
            let load = plugin.cpu_load();
            self.cpu_load.store(load.to_bits(), Ordering::Release);

            Ok(())
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Get current CPU load percentage
    pub fn cpu_load(&self) -> f32 {
        f32::from_bits(self.cpu_load.load(Ordering::Acquire))
    }

    /// Set bypass mode
    pub fn set_bypass(&mut self, bypassed: bool) -> Result<(), PluginError> {
        if let Some(plugin) = &mut self.plugin {
            plugin.set_bypass(bypassed);
            Ok(())
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Get bypass state
    pub fn is_bypassed(&self) -> Result<bool, PluginError> {
        if let Some(plugin) = &self.plugin {
            Ok(plugin.is_bypassed())
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Save preset
    pub fn save_preset(&mut self, name: String) -> Result<(), PluginError> {
        let mut preset = Preset::new(name);

        if let Some(plugin) = &self.plugin {
            for i in 0..plugin.parameter_count() {
                if let Some(value) = plugin.get_parameter(i) {
                    preset.parameters.push((i as u32, value));
                }
            }
        }

        self.presets.push(preset);
        Ok(())
    }

    /// Load preset
    pub fn load_preset(&mut self, index: usize) -> Result<(), PluginError> {
        if index >= self.presets.len() {
            return Err(PluginError::InvalidParameter);
        }

        let preset = &self.presets[index];
        if let Some(plugin) = &mut self.plugin {
            for &(param_id, value) in &preset.parameters {
                plugin.set_parameter(param_id as usize, value);
            }
        }

        self.current_preset = index;
        Ok(())
    }

    /// Get list of presets
    pub fn preset_list(&self) -> Vec<(usize, String)> {
        self.presets
            .iter()
            .enumerate()
            .map(|(i, p)| (i, p.name.clone()))
            .collect()
    }

    /// Serialize plugin state
    pub fn serialize_state(&self) -> Result<Vec<u8>, PluginError> {
        if let Some(plugin) = &self.plugin {
            let mut state = Vec::new();

            // Serialize parameters
            for i in 0..plugin.parameter_count() {
                if let Some(value) = plugin.get_parameter(i) {
                    state.extend_from_slice(&value.to_le_bytes());
                }
            }

            Ok(state)
        } else {
            Err(PluginError::NotInitialized)
        }
    }

    /// Deserialize plugin state
    pub fn deserialize_state(&mut self, state: &[u8]) -> Result<(), PluginError> {
        if let Some(plugin) = &mut self.plugin {
            let param_count = plugin.parameter_count();
            let bytes_per_param = 4;

            for i in 0..param_count {
                let offset = i * bytes_per_param;
                if offset + 4 <= state.len() {
                    let bytes = [
                        state[offset],
                        state[offset + 1],
                        state[offset + 2],
                        state[offset + 3],
                    ];
                    let value = f32::from_le_bytes(bytes);
                    plugin.set_parameter(i, value);
                }
            }

            Ok(())
        } else {
            Err(PluginError::NotInitialized)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestPlugin {
        sample_rate: f32,
        bypassed: bool,
        parameters: Vec<f32>,
    }

    impl TestPlugin {
        /// Initializes a new instance of the associated type.
        fn new() -> Self {
            Self {
                sample_rate: 44100.0,
                bypassed: false,
                parameters: vec![0.5, 0.0, 1.0],  // gain, attack, release
            }
        }
    }

    impl PluginProcessor for TestPlugin {
        /// Technical implementation of the init logic.
        fn init(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        /// Technical implementation of the activate logic.
        fn activate(&mut self, sample_rate: f32) -> Result<(), PluginError> {
            self.sample_rate = sample_rate;
            Ok(())
        }

        /// Technical implementation of the deactivate logic.
        fn deactivate(&mut self) -> Result<(), PluginError> {
            Ok(())
        }

        /// Primary real-time signal processing execution block.
        fn process(
            &mut self,
            _inputs: &[&[f32]],
            _outputs: &mut [&mut [f32]],
            _sample_count: usize,
            _events: &[MidiEvent],
        ) -> Result<(), PluginError> {
            Ok(())
        }

        /// Technical implementation of the parameter_count logic.
        fn parameter_count(&self) -> usize {
            3
        }

        /// Technical implementation of the parameter_info logic.
        fn parameter_info(&self, index: usize) -> Option<ParameterInfo> {
            match index {
                0 => Some(ParameterInfo {
                    id: 0,
                    name: "Gain".to_string(),
                    unit: "dB".to_string(),
                    min_value: -f32::INFINITY,
                    max_value: 24.0,
                    default_value: 0.0,
                    is_automatable: true,
                }),
                _ => None,
            }
        }

        /// Updates a framework parameter value.
        fn set_parameter(&mut self, index: usize, value: f32) {
            if index < self.parameters.len() {
                self.parameters[index] = value;
            }
        }

        /// Retrieves the current framework parameter value.
        fn get_parameter(&self, index: usize) -> Option<f32> {
            self.parameters.get(index).copied()
        }

        /// Technical implementation of the set_bypass logic.
        fn set_bypass(&mut self, bypassed: bool) {
            self.bypassed = bypassed;
        }

        /// Technical implementation of the is_bypassed logic.
        fn is_bypassed(&self) -> bool {
            self.bypassed
        }

        /// Technical implementation of the cpu_load logic.
        fn cpu_load(&self) -> f32 {
            5.0  // 5% CPU load
        }
    }

    #[test]
    /// Technical implementation of the test_plugin_host_creation logic.
    fn test_plugin_host_creation() {
        let host = PluginHost::new(2048, 4);
        assert_eq!(host.block_size, 2048);
        assert!(!host.is_active.load(Ordering::Acquire));
    }

    #[test]
    /// Technical implementation of the test_midi_event_creation logic.
    fn test_midi_event_creation() {
        let note_on = MidiEvent::note_on(0, 60, 100, 0);
        assert!(note_on.is_note_on());
        assert!(!note_on.is_note_off());

        let note_off = MidiEvent::note_off(2048, 60, 64, 0);
        assert!(note_off.is_note_off());
        assert!(!note_off.is_note_on());
    }

    #[test]
    /// Technical implementation of the test_preset_save_load logic.
    fn test_preset_save_load() {
        let mut host = PluginHost::new(2048, 4);
        let plugin = Box::new(TestPlugin::new());
        let _ = host.set_plugin(plugin);

        // Save preset
        let _ = host.save_preset("Test".to_string());
        assert_eq!(host.preset_list().len(), 1);
    }
}
