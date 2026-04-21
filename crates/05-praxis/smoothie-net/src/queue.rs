/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xfb2de3f9 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/05-praxis/smoothie-net/src/queue.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use core::sync::atomic::{AtomicUsize, Ordering};

/// Technical implementation of the LockFreeQueue structure.
pub struct LockFreeQueue<T, const N: usize> {
    buffer: [Option<T>; N],
    read_pos: AtomicUsize,
    write_pos: AtomicUsize,
}

impl<T: Copy, const N: usize> LockFreeQueue<T, N> {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            buffer: [None; N],
            read_pos: AtomicUsize::new(0),
            write_pos: AtomicUsize::new(0),
        }
    }

    /// Technical implementation of the push logic.
    pub fn push(&mut self, value: T) -> bool {
        let write = self.write_pos.load(Ordering::Relaxed);
        let next_write = (write + 1) % N;
        let read = self.read_pos.load(Ordering::Acquire);
        if next_write == read {
            return false;
        }
        self.buffer[write] = Some(value);
        self.write_pos.store(next_write, Ordering::Release);
        true
    }

    /// Technical implementation of the pop logic.
    pub fn pop(&mut self) -> Option<T> {
        let read = self.read_pos.load(Ordering::Relaxed);
        let write = self.write_pos.load(Ordering::Acquire);
        if read == write {
            return None;
        }
        let value = self.buffer[read].take();
        self.read_pos.store((read + 1) % N, Ordering::Release);
        value
    }
}
