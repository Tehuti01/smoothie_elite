/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6ee3fbdd | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-frontend/src/ipc.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::bridge::UiPayload;
use smoothie_core::ring_buffer::RingBuffer;

/// Ring buffer size for parameter updates (must be a power of two).
const PARAM_QUEUE_CAP: usize = 1024;

/// Technical implementation of the UiBridgeIpc structure.
pub struct UiBridgeIpc {
    /// UI -> DSP (Parameter adjustments from knobs)
    pub ui_to_dsp: RingBuffer<UiPayload>,
    /// DSP -> UI (Parameter automation updates, meters, playhead)
    pub dsp_to_ui: RingBuffer<UiPayload>,
}

impl UiBridgeIpc {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            ui_to_dsp: RingBuffer::new(PARAM_QUEUE_CAP),
            dsp_to_ui: RingBuffer::new(PARAM_QUEUE_CAP),
        }
    }

    /// DSP Thread: Consume pending updates from the UI.
    #[inline(always)]
    /// Technical implementation of the poll_ui_updates logic.
    pub fn poll_ui_updates<F>(&mut self, mut handler: F)
    where
        F: FnMut(UiPayload),
    {
        while let Some(payload) = self.ui_to_dsp.pop() {
            handler(payload);
        }
    }

    /// UI Thread: Consume pending updates from the DSP.
    pub fn poll_dsp_updates<F>(&mut self, mut handler: F)
    where
        F: FnMut(UiPayload),
    {
        while let Some(payload) = self.dsp_to_ui.pop() {
            handler(payload);
        }
    }
}

/// When processing 10,000 parameter updates per second from a GUI, we batch them.
pub trait ParamBuffer {
    /// Technical implementation of the push_update logic.
    fn push_update(&mut self, index: u32, normalized_value: f32);
    /// Technical implementation of the flush logic.
    fn flush(&mut self, ipc: &mut UiBridgeIpc);
}

/// Technical implementation of the BatchParamBuffer structure.
pub struct BatchParamBuffer {
    pending: [(u32, f32); 16],
    count: usize,
}

impl Default for BatchParamBuffer {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            pending: [(0, 0.0); 16],
            count: 0,
        }
    }
}

impl ParamBuffer for BatchParamBuffer {
    #[inline(always)]
    /// Technical implementation of the push_update logic.
    fn push_update(&mut self, index: u32, normalized_value: f32) {
        if self.count < 16 {
            self.pending[self.count] = (index, normalized_value);
            self.count += 1;
        }
    }

    /// Technical implementation of the flush logic.
    fn flush(&mut self, ipc: &mut UiBridgeIpc) {
        for i in 0..self.count {
            let (idx, val) = self.pending[i];
            let _ = ipc.ui_to_dsp.push(UiPayload::SetParameter {
                index: idx,
                value: val,
            });
        }
        self.count = 0;
    }
}
