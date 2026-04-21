/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb0682f37 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/async/src/waker.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::sync::Arc;
use alloc::task::Wake;
///
/// Bridges Smoothie's atomic state with core::task::Waker
use core::sync::atomic::{AtomicBool, Ordering};

/// Technical implementation of the TaskWaker structure.
pub struct TaskWaker {
    pub(crate) woken: AtomicBool,
    pub(crate) task_id: usize,
}

impl Wake for TaskWaker {
    /// Technical implementation of the wake logic.
    fn wake(self: Arc<Self>) {
        self.woken.store(true, Ordering::Release);
    }
}

impl TaskWaker {
    /// Create new TaskWaker wrapped in Arc
    pub fn new_arc(task_id: usize) -> Arc<Self> {
        Arc::new(Self {
            woken: AtomicBool::new(false),
            task_id,
        })
    }

    /// Check if woken
    pub fn is_woken(&self) -> bool {
        self.woken.load(Ordering::Acquire)
    }

    /// Clear woken state
    pub fn clear(&self) {
        self.woken.store(false, Ordering::Release);
    }

    /// Get task ID
    pub fn task_id(&self) -> usize {
        self.task_id
    }
}
