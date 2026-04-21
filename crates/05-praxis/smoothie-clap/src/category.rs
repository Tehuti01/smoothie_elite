/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa84ea8ec | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/category.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ClapCategory enumeration.
pub enum ClapCategory {
    /// Instrument generates sound (synths, samplers).
    Instrument,
    /// Audio effect processes sound in place.
    AudioEffect,
    /// Note effect transforms incoming notes.
    NoteEffect,
    /// Analyzer visualizes audio without modifying it.
    Analyzer,
}

impl ClapCategory {
    /// Convert category to CLAP feature tag slice.
    pub fn as_features(&self) -> &'static [&'static str] {
        match self {
            ClapCategory::Instrument => &["instrument", "synthesizer"],
            ClapCategory::AudioEffect => &["audio-effect"],
            ClapCategory::NoteEffect => &["note-effect"],
            ClapCategory::Analyzer => &["analyzer"],
        }
    }

    /// Create from feature slice.
    pub fn from_features(features: &[&str]) -> Option<Self> {
        if features.contains(&"instrument") || features.contains(&"synthesizer") {
            return Some(ClapCategory::Instrument);
        }
        if features.contains(&"audio-effect") {
            return Some(ClapCategory::AudioEffect);
        }
        if features.contains(&"note-effect") {
            return Some(ClapCategory::NoteEffect);
        }
        if features.contains(&"analyzer") {
            return Some(ClapCategory::Analyzer);
        }
        None
    }

    /// Check if this is a synth category.
    pub fn is_synth(&self) -> bool {
        matches!(self, ClapCategory::Instrument)
    }

    /// Check if this is an effect category.
    pub fn is_effect(&self) -> bool {
        matches!(
            self,
            ClapCategory::AudioEffect | ClapCategory::NoteEffect | ClapCategory::Analyzer
        )
    }
}

/// Sub-category for instrument plugins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the InstrumentSubcategory enumeration.
pub enum InstrumentSubcategory {
    Synthesizer,
    Sampler,
    DrumMachine,
    Arpeggiator,
    Sequencer,
}

impl InstrumentSubcategory {
    /// Technical implementation of the as_features logic.
    pub fn as_features(&self) -> &'static [&'static str] {
        match self {
            InstrumentSubcategory::Synthesizer => &["synthesizer"],
            InstrumentSubcategory::Sampler => &["sampler"],
            InstrumentSubcategory::DrumMachine => &["drum"],
            InstrumentSubcategory::Arpeggiator => &["arpeggiator"],
            InstrumentSubcategory::Sequencer => &["sequencer"],
        }
    }
}

/// Sub-category for audio effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the EffectSubcategory enumeration.
pub enum EffectSubcategory {
    Equalizer,
    Compressor,
    Limiter,
    Reverb,
    Delay,
    Chorus,
    Phaser,
    Flanger,
    Distortion,
    Filter,
    Gate,
    DeEssser,
    Tremolo,
    RingModulator,
}

impl EffectSubcategory {
    /// Technical implementation of the as_features logic.
    pub fn as_features(&self) -> &'static [&'static str] {
        match self {
            EffectSubcategory::Equalizer => &["equalizer"],
            EffectSubcategory::Compressor => &["compressor"],
            EffectSubcategory::Limiter => &["limiter"],
            EffectSubcategory::Reverb => &["reverb"],
            EffectSubcategory::Delay => &["delay"],
            EffectSubcategory::Chorus => &["chorus"],
            EffectSubcategory::Phaser => &["phaser"],
            EffectSubcategory::Flanger => &["flanger"],
            EffectSubcategory::Distortion => &["distortion"],
            EffectSubcategory::Filter => &["filter"],
            EffectSubcategory::Gate => &["gate"],
            EffectSubcategory::DeEssser => &["de-esser"],
            EffectSubcategory::Tremolo => &["tremolo"],
            EffectSubcategory::RingModulator => &["ring-modulator"],
        }
    }
}

/// Channel configuration options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ChannelConfig enumeration.
pub enum ChannelConfig {
    Mono,
    Stereo,
    Surround(u8),
    Ambisonic,
}

impl ChannelConfig {
    /// Technical implementation of the as_features logic.
    pub fn as_features(&self) -> &'static [&'static str] {
        match self {
            ChannelConfig::Mono => &["mono"],
            ChannelConfig::Stereo => &["stereo"],
            ChannelConfig::Surround(_n) => &["surround"],
            ChannelConfig::Ambisonic => &["ambisonic"],
        }
    }
}

/// Complete plugin classification.
#[derive(Debug, Clone)]
/// Technical implementation of the ClapPluginClassification structure.
pub struct ClapPluginClassification {
    pub category: ClapCategory,
    pub subcategory: Option<u8>,
    pub channels: ChannelConfig,
}

impl ClapPluginClassification {
    /// Initializes a new instance of the associated type.
    pub fn new(category: ClapCategory) -> Self {
        Self {
            category,
            subcategory: None,
            channels: ChannelConfig::Stereo,
        }
    }

    /// Technical implementation of the with_subcategory logic.
    pub fn with_subcategory(mut self, sub: u8) -> Self {
        self.subcategory = Some(sub);
        self
    }

    /// Technical implementation of the with_channels logic.
    pub fn with_channels(mut self, channels: ChannelConfig) -> Self {
        self.channels = channels;
        self
    }

    /// Technical implementation of the is_synth logic.
    pub fn is_synth(&self) -> bool {
        self.category.is_synth()
    }

    /// Technical implementation of the is_effect logic.
    pub fn is_effect(&self) -> bool {
        self.category.is_effect()
    }
}
