/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x19851b3f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/spatial-audio/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicU32, Ordering};
use core::f32::consts::PI;

/// Cache line size
const CACHE_LINE: usize = 64;

/// Source position in spherical coordinates
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
/// Technical implementation of the SourcePosition structure.
pub struct SourcePosition {
    /// Azimuth angle (0..2π, 0° = front, π/2 = left, π = back)
    pub azimuth: f32,

    /// Elevation angle (-π/2..π/2, 0° = horizontal, π/2 = above)
    pub elevation: f32,

    /// Distance in meters (typically 0.5..20.0)
    pub distance: f32,

    _pad: u32,
}

impl SourcePosition {
    /// Initializes a new instance of the associated type.
    pub fn new(azimuth: f32, elevation: f32, distance: f32) -> Self {
        Self {
            azimuth,
            elevation,
            distance,
            _pad: 0,
        }
    }

    /// Front center
    pub fn front() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Left
    pub fn left() -> Self {
        Self::new(PI / 2.0, 0.0, 1.0)
    }

    /// Right
    pub fn right() -> Self {
        Self::new(-PI / 2.0, 0.0, 1.0)
    }

    /// Back
    pub fn back() -> Self {
        Self::new(PI, 0.0, 1.0)
    }

    /// Above
    pub fn above() -> Self {
        Self::new(0.0, PI / 2.0, 1.0)
    }
}

/// HRTF spatial audio processor (64-byte aligned)
#[repr(align(64))]
/// Technical implementation of the SpatialAudioProcessor structure.
pub struct SpatialAudioProcessor {
    /// Current source position
    position: SourcePosition,

    /// Previous position (for Doppler calculation)
    prev_position: SourcePosition,

    /// Left channel HRTF buffer (pre-allocated)
    hrtf_left: Vec<f32>,

    /// Right channel HRTF buffer (pre-allocated)
    hrtf_right: Vec<f32>,

    /// HRTF length in samples
    hrtf_length: usize,

    /// Sample rate (Hz)
    sample_rate: f32,

    /// Listener forward vector (normalized)
    listener_forward: [f32; 3],

    /// Listener right vector (normalized)
    listener_right: [f32; 3],

    /// Listener up vector (normalized)
    listener_up: [f32; 3],

    /// Lock-free position update flag
    position_dirty: AtomicU32,

    /// Distance attenuation gain
    distance_gain: AtomicU32, // Bit-cast f32
}

impl SpatialAudioProcessor {
    /// Create new spatial audio processor
    ///
    /// # Arguments
    /// * `hrtf_length` - HRTF filter length (typical: 512-2048)
    /// * `sample_rate` - Sample rate in Hz
    pub fn new(hrtf_length: usize, sample_rate: f32) -> Self {
        Self {
            position: SourcePosition::front(),
            prev_position: SourcePosition::front(),
            hrtf_left: vec![0.0; hrtf_length],
            hrtf_right: vec![0.0; hrtf_length],
            hrtf_length,
            sample_rate,
            listener_forward: [0.0, 0.0, 1.0],
            listener_right: [1.0, 0.0, 0.0],
            listener_up: [0.0, 1.0, 0.0],
            position_dirty: AtomicU32::new(1),
            distance_gain: AtomicU32::new(0x3f800000), // f32::from_bits(0x3f800000) = 1.0
        }
    }

    /// Set source position (lock-free)
    pub fn set_position(&mut self, position: SourcePosition) {
        self.prev_position = self.position;
        self.position = position;
        self.position_dirty.store(1, Ordering::Release);
    }

    /// Get current position
    pub fn position(&self) -> SourcePosition {
        self.position
    }

    /// Update HRTF based on current position
    pub fn update_hrtf(&mut self) {
        if self.position_dirty.load(Ordering::Acquire) == 0 {
            return;
        }

        // Compute distance attenuation (inverse square law)
        let distance_clipped = self.position.distance.max(0.5).min(20.0);
        let ref_distance = 1.0;
        let gain = (ref_distance / distance_clipped).powf(2.0);
        self.distance_gain
            .store(gain.to_bits(), Ordering::Release);

        // Compute ITD (Interaural Time Difference)
        let itd_samples = self.compute_itd();

        // Generate HRTF filters based on azimuth/elevation
        self.generate_hrtf_filters(itd_samples);

        self.position_dirty.store(0, Ordering::Release);
    }

    /// Compute interaural time difference (ITD) in samples
    ///
    /// ITD ≈ (head_radius / speed_of_sound) * sin(azimuth)
    fn compute_itd(&self) -> f32 {
        let head_radius = 0.0875; // ~8.75 cm average head radius
        let speed_of_sound = 343.0; // m/s at 20°C

        let itd_sec = (head_radius / speed_of_sound) * self.position.azimuth.sin();
        itd_sec * self.sample_rate
    }

    /// Generate HRTF impulse responses for current position
    fn generate_hrtf_filters(&mut self, _itd_samples: f32) {
        // Simplified HRTF generation based on azimuth/elevation
        // In production, load from measured HRTF database (e.g., CIPIC, ARI)

        // Left channel HRTF
        for i in 0..self.hrtf_length {
            let phase = self.position.azimuth + (i as f32 / self.hrtf_length as f32) * PI;
            self.hrtf_left[i] = (1.0 + self.position.azimuth.cos()) * 0.5 * phase.sin();
        }

        // Right channel HRTF
        for i in 0..self.hrtf_length {
            let phase = -self.position.azimuth + (i as f32 / self.hrtf_length as f32) * PI;
            self.hrtf_right[i] = (1.0 - self.position.azimuth.cos()) * 0.5 * phase.sin();
        }
    }

    /// Process audio with HRTF convolution (simplified for this example)
    pub fn process(&self, mono_in: f32) -> (f32, f32) {
        let gain = f32::from_bits(self.distance_gain.load(Ordering::Acquire));

        // Apply HRTF filtering (simplified: weighted averaging for demo)
        let left_out = mono_in * self.hrtf_left[0] * gain;
        let right_out = mono_in * self.hrtf_right[0] * gain;

        (left_out, right_out)
    }

    /// Compute Doppler shift factor for moving source
    pub fn doppler_shift_factor(&self) -> f32 {
        let listener_vel = 0.0; // Static listener
        let source_vel = self.compute_source_velocity();

        let v_source_radial = source_vel * self.position.azimuth.cos();

        // Doppler formula: f' = f * (v_sound - v_listener) / (v_sound - v_source)
        let speed_of_sound = 343.0;
        (speed_of_sound - listener_vel) / (speed_of_sound - v_source_radial).max(0.1)
    }

    /// Compute source velocity from position change
    fn compute_source_velocity(&self) -> f32 {
        let delta_dist = self.position.distance - self.prev_position.distance;
        delta_dist * self.sample_rate // Simplified
    }

    /// Get distance attenuation gain
    pub fn distance_gain(&self) -> f32 {
        f32::from_bits(self.distance_gain.load(Ordering::Acquire))
    }

    /// Set listener orientation
    pub fn set_listener_orientation(&mut self, forward: [f32; 3], up: [f32; 3]) {
        self.listener_forward = forward;
        self.listener_up = up;

        // Compute right vector via cross product
        self.listener_right = [
            forward[1] * up[2] - forward[2] * up[1],
            forward[2] * up[0] - forward[0] * up[2],
            forward[0] * up[1] - forward[1] * up[0],
        ];
    }

    /// Get HRTF length
    pub fn hrtf_length(&self) -> usize {
        self.hrtf_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_source_positions logic.
    fn test_source_positions() {
        assert_eq!(SourcePosition::front().azimuth, 0.0);
        assert_eq!(SourcePosition::left().azimuth, PI / 2.0);
        assert_eq!(SourcePosition::right().azimuth, -PI / 2.0);
    }

    #[test]
    /// Technical implementation of the test_spatial_processor_creation logic.
    fn test_spatial_processor_creation() {
        let processor = SpatialAudioProcessor::new(512, 44100.0);
        assert_eq!(processor.hrtf_length(), 512);
    }

    #[test]
    /// Technical implementation of the test_position_update logic.
    fn test_position_update() {
        let mut processor = SpatialAudioProcessor::new(512, 44100.0);
        let pos = SourcePosition::left();
        processor.set_position(pos);

        processor.update_hrtf();
        assert_eq!(processor.position().azimuth, PI / 2.0);
    }

    #[test]
    /// Technical implementation of the test_distance_attenuation logic.
    fn test_distance_attenuation() {
        let mut processor = SpatialAudioProcessor::new(512, 44100.0);

        processor.set_position(SourcePosition::new(0.0, 0.0, 1.0));
        processor.update_hrtf();
        let gain1 = processor.distance_gain();

        processor.set_position(SourcePosition::new(0.0, 0.0, 2.0));
        processor.update_hrtf();
        let gain2 = processor.distance_gain();

        // Farther = less gain
        assert!(gain2 < gain1);
    }

    #[test]
    /// Technical implementation of the test_hrtf_processing logic.
    fn test_hrtf_processing() {
        let processor = SpatialAudioProcessor::new(512, 44100.0);
        let (left, right) = processor.process(1.0);

        // Should produce stereo output
        assert!(left.is_finite());
        assert!(right.is_finite());
    }

    #[test]
    /// Technical implementation of the test_doppler_shift logic.
    fn test_doppler_shift() {
        let mut processor = SpatialAudioProcessor::new(512, 44100.0);
        processor.set_position(SourcePosition::new(0.0, 0.0, 1.0));

        let doppler = processor.doppler_shift_factor();
        assert!(doppler > 0.0 && doppler < 10.0);
    }

    #[test]
    /// Technical implementation of the test_listener_orientation logic.
    fn test_listener_orientation() {
        let mut processor = SpatialAudioProcessor::new(512, 44100.0);
        processor.set_listener_orientation([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]);

        assert_eq!(processor.listener_forward, [0.0, 0.0, 1.0]);
        assert_eq!(processor.listener_up, [0.0, 1.0, 0.0]);
    }
}
