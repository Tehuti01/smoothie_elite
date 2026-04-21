/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb95711dc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/descriptor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the ClapDescriptor structure.
pub struct ClapDescriptor {
    /// CLAP protocol version this plugin targets (major, minor, revision).
    pub clap_version: (u32, u32, u32),
    /// Unique reverse-DNS style identifier, e.g. `"com.smoothieaudio.reverb"`.
    pub id: &'static str,
    /// Display name shown in the host plugin browser.
    pub name: &'static str,
    /// Vendor / developer name.
    pub vendor: &'static str,
    /// Plugin homepage URL.
    pub url: &'static str,
    /// Support contact URL or email.
    pub manual_url: &'static str,
    /// Support URL.
    pub support_url: &'static str,
    /// Semantic version string, e.g. `"1.0.0"`.
    pub version: &'static str,
    /// One-line description used in the host browser.
    pub description: &'static str,
    /// Null-terminated array of feature tags. Use `clap_plugin_features` constants.
    pub features: &'static [&'static str],
}

impl ClapDescriptor {
    /// Construct a descriptor with CLAP 1.x compatibility.
    pub const fn new(
        id: &'static str,
        name: &'static str,
        vendor: &'static str,
        version: &'static str,
        description: &'static str,
        features: &'static [&'static str],
    ) -> Self {
        Self {
            clap_version: (1, 2, 0),
            id,
            name,
            vendor,
            url: "https://github.com/tehuti01/smoothie_elite",
            manual_url: "https://github.com/tehuti01/smoothie_elite/wiki",
            support_url: "https://github.com/tehuti01/smoothie_elite/issues",
            version,
            description,
            features,
        }
    }
}

// Standard CLAP feature tag constants.
pub mod features {
    pub const INSTRUMENT: &str = "instrument";
    pub const AUDIO_EFFECT: &str = "audio-effect";
    pub const NOTE_EFFECT: &str = "note-effect";
    pub const ANALYZER: &str = "analyzer";
    pub const SYNTHESIZER: &str = "synthesizer";
    pub const SAMPLER: &str = "sampler";
    pub const DRUM: &str = "drum";
    pub const MIDI_EFFECT: &str = "midi-effect";
    pub const FILTER: &str = "filter";
    pub const PHASER: &str = "phaser";
    pub const EQUALIZER: &str = "equalizer";
    pub const DEESSER: &str = "de-esser";
    pub const REVERB: &str = "reverb";
    pub const DELAY: &str = "delay";
    pub const DISTORTION: &str = "distortion";
    pub const COMPRESSOR: &str = "compressor";
    pub const LIMITER: &str = "limiter";
    pub const MONO: &str = "mono";
    pub const STEREO: &str = "stereo";
    pub const SURROUND: &str = "surround";
    pub const AMBISONIC: &str = "ambisonic";
}
