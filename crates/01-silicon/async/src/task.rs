/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x5a3b67fe | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/async/src/task.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::boxed::Box;
/// Task scheduling, execution, and lifecycle (Silicon Hardened)
use core::pin::Pin;
use core::task::{Context, Poll};

/// Boxed future type for tasks
pub type BoxFuture<'a, T> = Pin<Box<dyn core::future::Future<Output = T> + Send + 'a>>;

/// Task ID type
pub type TaskId = usize;

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the TaskState enumeration.
pub enum TaskState {
    /// Task is pending execution
    Pending,
    /// Task is currently running
    Running,
    /// Task is waiting/blocked
    Waiting,
    /// Task has completed
    Completed,
    /// Task is cancelled
    Cancelled,
}

/// Technical implementation of the Task structure.
pub struct Task {
    id: TaskId,
    state: TaskState,
    future: BoxFuture<'static, ()>,
    priority: u8,
}

impl Task {
    /// Create new task from a future
    pub fn new(id: TaskId, future: BoxFuture<'static, ()>, priority: u8) -> Self {
        Self {
            id,
            state: TaskState::Pending,
            future,
            priority,
        }
    }

    /// Poll the task future
    pub fn poll(&mut self, context: &mut Context<'_>) -> Poll<()> {
        self.future.as_mut().poll(context)
    }

    /// Get task ID
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Get task state
    pub fn state(&self) -> TaskState {
        self.state
    }

    /// Set task state
    pub fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    /// Get priority (0 = highest, 255 = lowest)
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// Check if completed
    pub fn is_completed(&self) -> bool {
        self.state == TaskState::Completed
    }
}

/// Technical implementation of the TaskQueue structure.
pub struct TaskQueue {
    tasks: [Option<Task>; 233], // 233 = Fibonacci for task capacity
    count: usize,
}

impl TaskQueue {
    /// Create new task queue
    pub fn new() -> Self {
        let tasks: [Option<Task>; 233] = unsafe {
            let mut data: core::mem::MaybeUninit<[Option<Task>; 233]> =
                core::mem::MaybeUninit::uninit();
            for i in 0..233 {
                core::ptr::write(&mut (*data.as_mut_ptr())[i], None);
            }
            data.assume_init()
        };

        Self { tasks, count: 0 }
    }

    /// Add task to queue
    pub fn push(&mut self, task: Task) -> Result<(), &'static str> {
        if self.count >= 233 {
            return Err("Task queue full");
        }

        self.tasks[self.count] = Some(task);
        self.count += 1;
        Ok(())
    }

    /// Get next task by priority
    pub fn pop_next(&mut self) -> Option<Task> {
        if self.count == 0 {
            return None;
        }

        // Find highest priority (lowest value)
        let mut best_idx = 0;
        let mut best_priority = 255u8;

        for i in 0..self.count {
            if let Some(ref task) = self.tasks[i] {
                if task.priority < best_priority {
                    best_idx = i;
                    best_priority = task.priority;
                }
            }
        }

        // Extract task
        let task = self.tasks[best_idx].take();

        // Remove from queue by shifting
        for i in best_idx..self.count - 1 {
            self.tasks[i] = self.tasks[i + 1].take();
        }
        self.count -= 1;

        task
    }

    /// Get length
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}
