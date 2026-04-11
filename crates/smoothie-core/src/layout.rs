//! Audio I/O layout declarations.

/// Number of channels (0 = disabled).
pub type ChannelCount = u32;

/// Declares an audio I/O configuration supported by a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AudioLayout {
    pub input_channels:  ChannelCount,
    pub output_channels: ChannelCount,
    pub name:            &'static str,
}

impl AudioLayout {
    pub const fn new(input: ChannelCount, output: ChannelCount, name: &'static str) -> Self {
        Self { input_channels: input, output_channels: output, name }
    }

    /// Mono input → stereo output (most common guitar FX layout).
    pub const fn mono_in_stereo_out() -> Self {
        Self::new(1, 2, "Mono → Stereo")
    }

    /// Stereo in, stereo out.
    pub const fn stereo_in_stereo_out() -> Self {
        Self::new(2, 2, "Stereo → Stereo")
    }

    /// Mono in, mono out.
    pub const fn mono() -> Self {
        Self::new(1, 1, "Mono")
    }

    /// No audio input (synth / generator).
    pub const fn instrument() -> Self {
        Self::new(0, 2, "Instrument (0→2)")
    }
}
