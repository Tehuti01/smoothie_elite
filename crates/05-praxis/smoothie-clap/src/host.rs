/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xfd3c41ab | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-clap/src/host.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

/// Technical implementation of the HostHandle structure.
pub struct HostHandle {
    /// Version string reported by the host (e.g. `"Bitwig Studio 5.2"`).
    pub hostname: &'static str,
    /// CLAP version implemented by the host (major, minor, revision).
    pub host_version: (u32, u32, u32),
}

impl HostHandle {
    /// Log a debug message to the host's console.
    /// Callable from non-realtime threads only.
    pub fn log_debug(&self, msg: &str) {
        // In a real ABI implementation, this calls back into the host function pointer.
        // Here we provide a no-op placeholder.
        let _ = msg;
    }

    /// Notify the host that the plugin's output latency has changed.
    pub fn request_restart(&self) {}

    /// Notify the host that a resize of the plugin GUI is needed.
    pub fn request_resize(&self, _width: u32, _height: u32) {}
}
