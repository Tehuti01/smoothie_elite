/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x59b75d7f | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-vst3/src/category.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the Vst3Category enumeration.
pub enum Vst3Category {
    /// Audio effect - processes audio.
    AudioEffect,
    /// Synthesizer - generates sound.
    Synthesizer,
    /// Analysis - analyzes audio.
    Analysis,
    /// MIDI effect - transforms MIDI.
    MidiEffect,
    /// Delay - echo effects.
    Delay,
    /// Reverb - reverb effects.
    Reverb,
    /// Equalizer - frequency processing.
    Equalizer,
    /// Compressor - dynamics processing.
    Compressor,
    /// Limiter - dynamic limiting.
    Limiter,
    /// Gate - noise gate.
    Gate,
    /// Distortion.
    Distortion,
    /// Filter.
    Filter,
    /// Pitch shifter.
    PitchShift,
    /// Modulation.
    Modulation,
    /// Harmonic exciter.
    Harmonic,
    /// Transient shaper.
    Transient,
    /// Stereo width.
    StereoWidth,
    /// Spatial.
    Spatial,
    /// Surround encoder.
    SurroundEncoder,
    /// Up-mixer.
    UpMix,
    /// Loudness meter.
    LoudnessMeter,
    /// Phase meter.
    PhaseMeter,
    /// Spectrum analyzer.
    Spectrum,
    /// Oscilloscope.
    Oscilloscope,
    /// Drum synthesizer.
    DrumSynth,
    /// Sampler.
    Sampler,
    /// Instrument - general.
    Instrument,
    /// Guitar amp simulation.
    GuitarAmp,
    /// Guitar cabinet simulation.
    GuitarCabinet,
    /// Other.
    Other,
}

impl Vst3Category {
    /// Get the canonical VST3 category string.
    pub fn as_str(&self) -> &'static str {
        match self {
            Vst3Category::AudioEffect => "Audio Effect",
            Vst3Category::Synthesizer => "Synthesizer",
            Vst3Category::Analysis => "Analysis",
            Vst3Category::MidiEffect => "MIDI Effect",
            Vst3Category::Delay => "Delay",
            Vst3Category::Reverb => "Reverb",
            Vst3Category::Equalizer => "Equalizer",
            Vst3Category::Compressor => "Compressor",
            Vst3Category::Limiter => "Limiter",
            Vst3Category::Gate => "Gate",
            Vst3Category::Distortion => "Distortion",
            Vst3Category::Filter => "Filter",
            Vst3Category::PitchShift => "Pitch Shift",
            Vst3Category::Modulation => "Modulation",
            Vst3Category::Harmonic => "Harmonic",
            Vst3Category::Transient => "Transient",
            Vst3Category::StereoWidth => "Stereo Width",
            Vst3Category::Spatial => "Spatial",
            Vst3Category::SurroundEncoder => "Surround Encoder",
            Vst3Category::UpMix => "Up-Mix",
            Vst3Category::LoudnessMeter => "Loudness Meter",
            Vst3Category::PhaseMeter => "Phase Meter",
            Vst3Category::Spectrum => "Spectrum",
            Vst3Category::Oscilloscope => "Oscilloscope",
            Vst3Category::DrumSynth => "Drum Synthesizer",
            Vst3Category::Sampler => "Sampler",
            Vst3Category::Instrument => "Instrument",
            Vst3Category::GuitarAmp => "Guitar Amp",
            Vst3Category::GuitarCabinet => "Guitar Cabinet",
            Vst3Category::Other => "Other",
        }
    }

    /// Check if this is an effect category.
    pub fn is_effect(&self) -> bool {
        matches!(
            self,
            Vst3Category::AudioEffect
                | Vst3Category::Delay
                | Vst3Category::Reverb
                | Vst3Category::Equalizer
                | Vst3Category::Compressor
                | Vst3Category::Limiter
                | Vst3Category::Gate
                | Vst3Category::Distortion
                | Vst3Category::Filter
        )
    }

    /// Check if this is an instrument category.
    pub fn is_instrument(&self) -> bool {
        matches!(
            self,
            Vst3Category::Synthesizer
                | Vst3Category::DrumSynth
                | Vst3Category::Sampler
                | Vst3Category::Instrument
        )
    }

    /// Check if this is a synthesizer.
    pub fn is_synth(&self) -> bool {
        matches!(self, Vst3Category::Synthesizer | Vst3Category::DrumSynth)
    }
}

impl Default for Vst3Category {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Vst3Category::AudioEffect
    }
}
