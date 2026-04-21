/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xd2a28648 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-vst3/src/plugin_entry.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::string::String;

/// VST3 plugin entry information.
#[derive(Debug, Clone)]
#[repr(align(64))]
/// Technical implementation of the Vst3PluginEntry structure.
pub struct Vst3PluginEntry {
    pub vendor: &'static str,
    pub url: &'static str,
    pub email: &'static str,
    pub name: String,
    pub version: u32,
    pub sdk_version: u32,
    pub class_count: usize,
}

impl Vst3PluginEntry {
    /// Initializes a new instance of the associated type.
    pub fn new(name: impl Into<String>, vendor: &'static str) -> Self {
        Self {
            vendor,
            url: "https://github.com/tehuti01/smoothie_elite",
            email: "support@smoothieaudio.dev",
            name: name.into(),
            version: 0x01000000,
            sdk_version: 0x030000,
            class_count: 1,
        }
    }

    /// Technical implementation of the with_version logic.
    pub fn with_version(mut self, major: u8, minor: u8, micro: u8) -> Self {
        self.version = ((major as u32) << 24) | ((minor as u32) << 16) | (micro as u32);
        self
    }

    /// Technical implementation of the with_sdk_version logic.
    pub fn with_sdk_version(mut self, major: u8, minor: u8, patch: u8) -> Self {
        self.sdk_version = ((major as u32) << 16) | ((minor as u32) << 8) | (patch as u32);
        self
    }

    /// Technical implementation of the version_string logic.
    pub fn version_string(&self) -> alloc::string::String {
        let major = (self.version >> 24) as u8;
        let minor = (self.version >> 16) as u8;
        let micro = (self.version >> 8) as u8;
        alloc::format!("{}.{}.{}", major, minor, micro)
    }
}

/// VST3 component flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(align(64))]
/// Technical implementation of the Vst3ComponentFlags structure.
pub struct Vst3ComponentFlags(pub u32);

impl Vst3ComponentFlags {
    pub const NONE: Self = Self(0);
    pub const SIMPLE: Self = Self(1 << 0);
    pub const DUAL: Self = Self(1 << 1);
    pub const DUAL_COPY: Self = Self(1 << 2);
    pub const REQUIRES_GUI: Self = Self(1 << 3);
    pub const DAW_SILENT_SWITCH: Self = Self(1 << 4);
    pub const CHAINING: Self = Self(1 << 5);
    pub const MULTI_FEATURES: Self = Self(1 << 6);
}

impl Default for Vst3ComponentFlags {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::NONE
    }
}

impl core::ops::BitOr for Vst3ComponentFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// VST3 processor info.
#[derive(Debug, Clone)]
#[repr(align(64))]
/// Technical implementation of the Vst3ProcessorInfo structure.
pub struct Vst3ProcessorInfo {
    pub flags: Vst3ComponentFlags,
    pub latency_samples: u32,
    pub initial_delay: u32,
    pub tail_samples: u32,
    pub silnable_tail: bool,
}

impl Default for Vst3ProcessorInfo {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            flags: Vst3ComponentFlags::SIMPLE,
            latency_samples: 0,
            initial_delay: 0,
            tail_samples: 0,
            silnable_tail: false,
        }
    }
}
