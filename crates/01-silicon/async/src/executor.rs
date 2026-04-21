/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x7c08c453 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/async/src/executor.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::task::{Task, TaskId, TaskQueue};
use crate::waker::TaskWaker;
use alloc::boxed::Box;
use alloc::sync::Arc;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::task::{Context, Poll};

/// Global task ID counter
static NEXT_TASK_ID: AtomicUsize = AtomicUsize::new(1);

/// Get next task ID
fn next_task_id() -> TaskId {
    NEXT_TASK_ID.fetch_add(1, Ordering::SeqCst)
}

/// Technical implementation of the Executor structure.
pub struct Executor {
    task_queue: TaskQueue,
    // Track tasks for waking
    wakers: [Option<Arc<TaskWaker>>; 233],
}

impl Executor {
    /// Create new executor
    pub fn new() -> Self {
        let wakers: [Option<Arc<TaskWaker>>; 233] = unsafe {
            let mut data: core::mem::MaybeUninit<[Option<Arc<TaskWaker>>; 233]> =
                core::mem::MaybeUninit::uninit();
            for i in 0..233 {
                core::ptr::write(&mut (*data.as_mut_ptr())[i], None);
            }
            data.assume_init()
        };

        Self {
            task_queue: TaskQueue::new(),
            wakers,
        }
    }

    /// Spawn a new task
    pub fn spawn<F>(&mut self, future: F) -> Result<TaskId, &'static str>
    where
        F: core::future::Future<Output = ()> + Send + 'static,
    {
        let task_id = next_task_id();
        let task = Task::new(task_id, Box::pin(future), 128);

        // Associate a waker
        let waker_arc = TaskWaker::new_arc(task_id);
        self.wakers[task_id % 233] = Some(waker_arc);

        self.task_queue.push(task)?;
        Ok(task_id)
    }

    /// Step executor once (poll one task)
    pub fn step(&mut self) -> bool {
        if let Some(mut task) = self.task_queue.pop_next() {
            let id = task.id();
            let waker_arc = self.wakers[id % 233].as_ref().unwrap().clone();

            // Re-clear woken status before polling
            waker_arc.clear();

            let waker = core::task::Waker::from(waker_arc);
            let mut context = Context::from_waker(&waker);

            match task.poll(&mut context) {
                Poll::Ready(()) => {
                    // Task done, clear waker
                    self.wakers[id % 233] = None;
                }
                Poll::Pending => {
                    // Task still pending, push back
                    if let Err(_) = self.task_queue.push(task) {
                        // Queue full, this is fatal to the task
                        self.wakers[id % 233] = None;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// Run until all tasks are complete
    pub fn run(&mut self) {
        while self.step() {
            // In a real real-time system, we'd have a spin-loop or yield here
        }
    }
}

/// Technical implementation of the block_on logic.
pub fn block_on<F: core::future::Future>(future: F) -> F::Output {
    let mut pinned = Box::pin(future);
    let task_id = 0; // Pseudo-task ID for block_on
    let waker_arc = TaskWaker::new_arc(task_id);
    let waker = core::task::Waker::from(waker_arc.clone());
    let mut context = Context::from_waker(&waker);

    loop {
        match pinned.as_mut().poll(&mut context) {
            Poll::Ready(val) => return val,
            Poll::Pending => {
                // Wait for the waker to be notified
                while !waker_arc.is_woken() {
                    core::hint::spin_loop();
                }
                waker_arc.clear();
            }
        }
    }
}

/// Technical implementation of the spawn logic.
pub fn spawn<F>(_future: F) -> Result<TaskId, &'static str>
where
    F: core::future::Future<Output = ()> + Send + 'static,
{
    // In a real framework, this would spawn onto a global executor
    Err("Global spawn requires an active executor manifold")
}
