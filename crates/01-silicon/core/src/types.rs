/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x04171ec6 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/core/src/types.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::primitives::{MidiNote, MidiVelocity, Sample, Seconds};

/// Stereo sample pair (Left, Right)
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the StereoSample structure.
pub struct StereoSample {
    pub left: Sample,
    pub right: Sample,
}

impl StereoSample {
    /// Create stereo sample
    pub const fn new(left: Sample, right: Sample) -> Self {
        Self { left, right }
    }

    /// Create mono sample (same for both channels)
    pub const fn mono(sample: Sample) -> Self {
        Self {
            left: sample,
            right: sample,
        }
    }

    /// Sum to mono
    pub fn to_mono(&self) -> Sample {
        (self.left + self.right) * 0.5
    }

    /// Scale amplitude
    pub fn scale(&mut self, factor: Sample) {
        self.left *= factor;
        self.right *= factor;
    }
}

/// MIDI note with velocity information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiNoteOn structure.
pub struct MidiNoteOn {
    pub note: MidiNote,
    pub velocity: MidiVelocity,
}

impl MidiNoteOn {
    /// Create note-on event
    pub const fn new(note: MidiNote, velocity: MidiVelocity) -> Self {
        Self { note, velocity }
    }

    /// Velocity as normalized float (0.0 - 1.0)
    pub fn velocity_normalized(&self) -> f32 {
        self.velocity as f32 / 127.0
    }
}

/// MIDI note-off event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiNoteOff structure.
pub struct MidiNoteOff {
    pub note: MidiNote,
}

impl MidiNoteOff {
    /// Create note-off event
    pub const fn new(note: MidiNote) -> Self {
        Self { note }
    }
}

/// MIDI controller value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiCC structure.
pub struct MidiCC {
    pub controller: u8,
    pub value: u8,
}

impl MidiCC {
    /// Create CC event
    pub const fn new(controller: u8, value: u8) -> Self {
        Self { controller, value }
    }

    /// Value as normalized float (0.0 - 1.0)
    pub fn value_normalized(&self) -> f32 {
        self.value as f32 / 127.0
    }
}

/// MIDI pitch bend event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the MidiPitchBend structure.
pub struct MidiPitchBend {
    pub value: u16, // 14-bit value (0 - 16383, center is 8192)
}

impl MidiPitchBend {
    /// Create pitch bend event
    pub const fn new(value: u16) -> Self {
        Self {
            value: value & 0x3FFF,
        }
    }

    /// Get bend amount as semitones (typically -2 to +2)
    pub fn semitones(&self) -> f32 {
        let centered = self.value as f32 - 8192.0;
        (centered / 8192.0) * 2.0 // ±2 semitones default
    }
}

/// MIDI message envelope
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the MidiMessage enumeration.
pub enum MidiMessage {
    NoteOn(MidiNoteOn),
    NoteOff(MidiNoteOff),
    ControlChange(MidiCC),
    PitchBend(MidiPitchBend),
    ProgramChange(u8),
    ChannelPressure(u8),
    PolyPressure(MidiNote, u8),
}

/// Oscillator waveform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the Waveform enumeration.
pub enum Waveform {
    /// Pure sine wave
    Sine,
    /// Triangle wave with harmonics
    Triangle,
    /// Sawtooth wave with ringing
    Sawtooth,
    /// Square wave with PWM
    Square,
    /// Pulse wave with adjustable duty cycle
    Pulse,
    /// Noise generator
    Noise,
    /// Custom user-defined waveform
    Custom(u8),
}

impl Waveform {
    /// Get harmonic content approximation
    pub fn harmonic_richness(&self) -> u8 {
        match self {
            Waveform::Sine => 1,
            Waveform::Triangle => 13, // Fibonacci: perfect for audio range
            Waveform::Sawtooth => 21, // 13 + 8 harmonics
            Waveform::Square => 89,   // Fibonacci series
            Waveform::Pulse => 55,    // Fibonacci
            Waveform::Noise => 144,   // Maximum harmonic density
            Waveform::Custom(n) => *n,
        }
    }
}

/// Filter types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the FilterType enumeration.
pub enum FilterType {
    /// Low-pass filter (removes high frequencies)
    LowPass,
    /// High-pass filter (removes low frequencies)
    HighPass,
    /// Band-pass filter (isolates frequency range)
    BandPass,
    /// Band-stop / Notch filter
    BandStop,
    /// All-pass filter (constant magnitude, phase shifting)
    AllPass,
    /// Peak / Peaking EQ
    Peak,
    /// Shelf filters for frequency shaping
    Shelf,
}

/// Filter response steepness
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the FilterSlope enumeration.
pub enum FilterSlope {
    /// 6 dB/octave, 1st order
    SlopeFirst,
    /// 12 dB/octave, 2nd order
    SlopeSecond,
    /// 18 dB/octave, 3rd order
    SlopeThird,
    /// 24 dB/octave, 4th order (Fibonacci state boundary)
    SlopeFourth,
}

impl FilterSlope {
    /// Get order as usize for DSP algorithms
    pub fn order(&self) -> usize {
        match self {
            FilterSlope::SlopeFirst => 1,
            FilterSlope::SlopeSecond => 2,
            FilterSlope::SlopeThird => 3,
            FilterSlope::SlopeFourth => 4,
        }
    }

    /// Get rolloff in dB per octave
    pub fn db_per_octave(&self) -> f32 {
        match self {
            FilterSlope::SlopeFirst => 6.0,
            FilterSlope::SlopeSecond => 12.0,
            FilterSlope::SlopeThird => 18.0,
            FilterSlope::SlopeFourth => 24.0,
        }
    }
}

/// Envelope stage in ADSR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the EnvelopeStage enumeration.
pub enum EnvelopeStage {
    /// Attack phase (rising to peak)
    Attack,
    /// Decay phase (falling to sustain)
    Decay,
    /// Sustain phase (held level)
    Sustain,
    /// Release phase (falling to zero)
    Release,
    /// Not triggered (idle)
    Idle,
}

/// ADSR Envelope parameters
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the ADSRParameters structure.
pub struct ADSRParameters {
    /// Attack time in seconds
    pub attack: Seconds,
    /// Decay time in seconds
    pub decay: Seconds,
    /// Sustain level (0.0 - 1.0)
    pub sustain: f32,
    /// Release time in seconds
    pub release: Seconds,
    /// Peak level (0.0 - 1.0)
    pub peak: f32,
}

impl ADSRParameters {
    /// Create ADSR with typical values
    pub const fn new(attack: Seconds, decay: Seconds, sustain: f32, release: Seconds) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            peak: 1.0,
        }
    }

    /// Create ADSR with explicit peak level
    pub const fn with_peak(
        attack: Seconds,
        decay: Seconds,
        sustain: f32,
        release: Seconds,
        peak: f32,
    ) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            peak,
        }
    }

    /// Total envelope duration (attack + decay + release)
    pub fn total_duration(&self) -> Seconds {
        self.attack + self.decay + self.release
    }
}

impl Default for ADSRParameters {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new(0.01, 0.1, 0.5, 0.3)
    }
}

/// Quantization bit depths
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the BitDepth enumeration.
pub enum BitDepth {
    /// 8-bit (256 levels)
    Bit8,
    /// 16-bit (65536 levels)
    Bit16,
    /// 24-bit (16.7 million levels)
    Bit24,
    /// 32-bit floating point
    Bit32F,
}

impl BitDepth {
    /// Maximum value for this bit depth
    pub fn max_value(&self) -> f32 {
        match self {
            BitDepth::Bit8 => 255.0,
            BitDepth::Bit16 => 65535.0,
            BitDepth::Bit24 => 16777215.0,
            BitDepth::Bit32F => 1.0,
        }
    }

    /// Quantization level count
    pub fn levels(&self) -> u32 {
        match self {
            BitDepth::Bit8 => 256,
            BitDepth::Bit16 => 65536,
            BitDepth::Bit24 => 16777216,
            BitDepth::Bit32F => 0xFFFFFFFF,
        }
    }
}

/// Audio channel configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ChannelConfig enumeration.
pub enum ChannelConfig {
    /// Mono (1 channel)
    Mono,
    /// Stereo (2 channels)
    Stereo,
    /// 2.1 (3 channels with subwoofer)
    TwoPointOne,
    /// 5.1 Surround (6 channels)
    FivePointOne,
    /// 7.1 Surround (8 channels)
    SevenPointOne,
}

impl ChannelConfig {
    /// Get channel count
    pub fn channels(&self) -> usize {
        match self {
            ChannelConfig::Mono => 1,
            ChannelConfig::Stereo => 2,
            ChannelConfig::TwoPointOne => 3,
            ChannelConfig::FivePointOne => 6,
            ChannelConfig::SevenPointOne => 8,
        }
    }
}

/// Sample rate constants (Fibonacci-harmonized options)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SampleRate enumeration.
pub enum SampleRate {
    /// 44.1 kHz (CD quality, 89 samples per 2ms)
    Rate44100,
    /// 48 kHz (industry standard, 96 samples per 2ms)
    Rate48000,
    /// 88.2 kHz (high quality, 144 Fibonacci samples per 1.6ms)
    Rate88200,
    /// 96 kHz (professional, 192 samples per 2ms)
    Rate96000,
    /// 192 kHz (mastering quality)
    Rate192000,
}

impl SampleRate {
    /// Get sample rate as f32
    pub fn hz(&self) -> f32 {
        match self {
            SampleRate::Rate44100 => 44100.0,
            SampleRate::Rate48000 => 48000.0,
            SampleRate::Rate88200 => 88200.0,
            SampleRate::Rate96000 => 96000.0,
            SampleRate::Rate192000 => 192000.0,
        }
    }

    /// Get sample period in seconds
    pub fn period(&self) -> Seconds {
        1.0 / self.hz()
    }

    /// Get samples per millisecond
    pub fn samples_per_ms(&self) -> f32 {
        self.hz() / 1000.0
    }
}

/// Interpolation method for pitch shifting and time stretching
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the InterpolationMethod enumeration.
pub enum InterpolationMethod {
    /// Nearest neighbor (fastest, lowest quality)
    Nearest,
    /// Linear interpolation (good compromise)
    Linear,
    /// Cubic Hermite interpolation (high quality)
    Hermite,
    /// Catmull-Rom spline (best quality, more expensive)
    CatmullRom,
}

/// Modulation LFO shape
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the LFOShape enumeration.
pub enum LFOShape {
    /// Sine wave modulation
    Sine,
    /// Triangle wave modulation
    Triangle,
    /// Sawtooth wave modulation
    Sawtooth,
    /// Square wave modulation
    Square,
    /// Random stepped modulation
    SampleHold,
    /// Smooth random modulation
    SmoothRandom,
}

/// Effect position in audio chain
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the EffectPosition enumeration.
pub enum EffectPosition {
    /// Pre-processing (before main processing)
    Pre,
    /// Insert effect (main processing path)
    Insert,
    /// Parallel effect (mixed with dry signal)
    Parallel,
    /// Post-processing (after main processing)
    Post,
}

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the CompressorType enumeration.
pub enum CompressorType {
    /// Standard VCA compressor
    VCA,
    /// FET (Field Effect Transistor) compressor
    FET,
    /// VLA (Variable Mu) compressor
    VLA,
    /// Optical compressor
    Optical,
}

/// Stereo linking mode for dynamics processors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the StereoLink enumeration.
pub enum StereoLink {
    /// Mono summing (same gain reduction for both channels)
    Sum,
    /// Maximum (use loudest channel for both)
    Maximum,
    /// Independent (separate processing for each channel)
    Independent,
}

/// Spectral analysis window type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the SpectrumWindow enumeration.
pub enum SpectrumWindow {
    /// Rectangular window (fastest, highest spectral leakage)
    Rectangular,
    /// Hann window (balanced, FFT standard)
    Hann,
    /// Hamming window (slightly better than Hann)
    Hamming,
    /// Blackman window (best frequency resolution)
    Blackman,
}

/// Phase correlation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the PhaseMode enumeration.
pub enum PhaseMode {
    /// Track absolute phase
    Absolute,
    /// Track relative phase between channels
    Relative,
    /// Ignore phase (magnitude only)
    Magnitude,
}

/// Tempo/BPM information
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the TempoInfo structure.
pub struct TempoInfo {
    /// Beats per minute
    pub bpm: f32,
    /// Time signature numerator
    pub time_sig_num: u8,
    /// Time signature denominator
    pub time_sig_den: u8,
    /// Current beat position (0.0 - 1.0 per measure)
    pub beat_position: f32,
}

impl TempoInfo {
    /// Create tempo information
    pub const fn new(bpm: f32, time_sig_num: u8, time_sig_den: u8) -> Self {
        Self {
            bpm,
            time_sig_num,
            time_sig_den,
            beat_position: 0.0,
        }
    }

    /// Get beat duration in seconds
    pub fn beat_duration(&self) -> Seconds {
        60.0 / self.bpm
    }

    /// Get measure duration in seconds
    pub fn measure_duration(&self) -> Seconds {
        self.beat_duration() * self.time_sig_num as f32
    }

    /// Convert samples to beat fraction
    pub fn samples_to_beats(&self, samples: usize, sample_rate: f32) -> f32 {
        let seconds = samples as f32 / sample_rate;
        seconds / self.beat_duration()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_stereo_sample logic.
    fn test_stereo_sample() {
        let sample = StereoSample::new(0.5, 0.3);
        assert_eq!(sample.left, 0.5);
        assert_eq!(sample.right, 0.3);

        let mono = sample.to_mono();
        assert!((mono - 0.4).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_midi_note logic.
    fn test_midi_note() {
        let note = MidiNoteOn::new(60, 100);
        assert_eq!(note.note, 60);
        assert!((note.velocity_normalized() - (100.0 / 127.0)).abs() < 0.01);
    }

    #[test]
    /// Technical implementation of the test_adsr_parameters logic.
    fn test_adsr_parameters() {
        let adsr = ADSRParameters::new(0.1, 0.2, 0.7, 0.5);
        let total = adsr.total_duration();
        assert!((total - 0.8).abs() < 0.001);
    }

    #[test]
    /// Technical implementation of the test_sample_rate logic.
    fn test_sample_rate() {
        let sr = SampleRate::Rate44100;
        assert_eq!(sr.hz(), 44100.0);
        assert!(sr.samples_per_ms() > 44.0 && sr.samples_per_ms() < 45.0);
    }

    #[test]
    /// Technical implementation of the test_channel_config logic.
    fn test_channel_config() {
        assert_eq!(ChannelConfig::Stereo.channels(), 2);
        assert_eq!(ChannelConfig::FivePointOne.channels(), 6);
    }

    #[test]
    /// Technical implementation of the test_tempo_info logic.
    fn test_tempo_info() {
        let tempo = TempoInfo::new(120.0, 4, 4);
        assert_eq!(tempo.beat_duration(), 0.5);
    }

    #[test]
    /// Technical implementation of the test_waveform_harmonics logic.
    fn test_waveform_harmonics() {
        assert_eq!(Waveform::Sine.harmonic_richness(), 1);
        assert_eq!(Waveform::Square.harmonic_richness(), 89);
        assert_eq!(Waveform::Noise.harmonic_richness(), 144);
    }
}
