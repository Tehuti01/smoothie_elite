/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x435ddaf9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-frontend/src/host.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::ffi::c_void;

/// Compatible with `raw-window-handle` concepts, executed strictly under `no_std`.
#[repr(C)]
/// Technical implementation of the NativeWindowHandle enumeration.
pub enum NativeWindowHandle {
    /// Apple macOS `NSView*` pointer.
    Cocoa(*mut c_void),
    /// Microsoft Windows `HWND` pointer.
    Win32(*mut c_void),
    /// Linux/Unix X11 `Window`.
    X11(u32),
    /// Wayland Surface pointer.
    Wayland(*mut c_void),
}

/// Struct encapsulating the physical and dimensional mount point for the WebView.
#[repr(C)]
/// Technical implementation of the EditorView structure.
pub struct EditorView {
    pub handle: NativeWindowHandle,
    pub width: u32,
    pub height: u32,
    pub dpr: f32, // Device Pixel Ratio (e.g. 2.0 for Retina displays)
}

impl EditorView {
    /// Prepares the geometry boundaries before injecting the web engine.
    pub fn logical_bounds(&self) -> (f32, f32) {
        let w = self.width as f32 / self.dpr;
        let h = self.height as f32 / self.dpr;
        (w, h)
    }
}
