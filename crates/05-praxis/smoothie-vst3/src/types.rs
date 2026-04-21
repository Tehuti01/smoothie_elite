/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb92adb6a | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-vst3/src/types.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the ProcessSetup structure.
pub struct ProcessSetup {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub max_samples_per_block: i32,
    pub sample_rate: f64,
}

#[repr(C)]
/// Technical implementation of the AudioBusBuffers structure.
pub struct AudioBusBuffers {
    pub num_channels: i32,
    pub silence_flags: u64,
    /// Arrays of pointers (f32** or f64**)
    pub channel_buffers32: *mut *mut f32,
    pub channel_buffers64: *mut *mut f64,
}

#[repr(C)]
/// Technical implementation of the ProcessData structure.
pub struct ProcessData {
    pub process_mode: i32,
    pub symbolic_sample_size: i32,
    pub num_samples: i32,
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub inputs: *mut AudioBusBuffers,
    pub outputs: *mut AudioBusBuffers,
    pub parameter_changes: *mut core::ffi::c_void,
    pub event_list: *mut core::ffi::c_void,
}
