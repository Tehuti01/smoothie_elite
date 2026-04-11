//! Plugin format flags and identifiers.

bitflags::bitflags! {
    /// Bit-flags for the plugin formats a plugin supports.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct FormatFlags: u8 {
        const VST3       = 0b0000_0001;
        const CLAP       = 0b0000_0010;
        const AU         = 0b0000_0100;
        const AAX        = 0b0000_1000;
        const STANDALONE = 0b0001_0000;
    }
}

/// The active plugin format this instance is running as.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginFormat {
    Vst3,
    Clap,
    Au,
    Aax,
    Standalone,
}

impl PluginFormat {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Self::Vst3       => "VST3",
            Self::Clap       => "CLAP",
            Self::Au         => "AU",
            Self::Aax        => "AAX",
            Self::Standalone => "Standalone",
        }
    }
}
