/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6b49b4c1 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-au/src/types.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the AudioComponentDescription structure.
pub struct AudioComponentDescription {
    pub component_type: u32,
    pub component_sub_type: u32,
    pub component_manufacturer: u32,
    pub component_flags: u32,
    pub component_flags_mask: u32,
}

#[repr(C)]
/// Technical implementation of the AudioBuffer structure.
pub struct AudioBuffer {
    pub m_number_channels: u32,
    pub m_data_byte_size: u32,
    pub m_data: *mut core::ffi::c_void,
}

#[repr(C)]
/// Technical implementation of the AudioBufferList structure.
pub struct AudioBufferList {
    pub m_number_buffers: u32,
    pub m_buffers: [AudioBuffer; 1],
}

#[repr(C)]
/// Technical implementation of the AudioTimeStamp structure.
pub struct AudioTimeStamp {
    pub m_sample_time: f64,
    pub m_host_time: u64,
    pub m_rate_scalar: f64,
    pub m_word_clock_time: u64,
    pub m_smpte_time: u32, // Struct omitted
    pub m_flags: u32,
    pub m_reserved: u32,
}

pub type OSStatus = i32;
pub const NO_ERR: OSStatus = 0;
pub const K_AUDIO_UNIT_ERR_INVALID_PROPERTY: OSStatus = -10879;
