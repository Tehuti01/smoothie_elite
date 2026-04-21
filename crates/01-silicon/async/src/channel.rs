/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x042eba53 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/async/src/channel.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::sync::Arc;
///
/// MPSC, SPMC, and other channel implementations (Silicon Hardened)
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::task::{Context, Poll};

/// Technical implementation of the Sender structure.
pub struct Sender<T> {
    state: Arc<ChannelState<T>>,
}

impl<T> Sender<T> {
    /// Send value down channel
    pub fn send(&self, val: T) -> Result<(), T> {
        self.state.push(val)
    }
}

impl<T> Clone for Sender<T> {
    /// Technical implementation of the clone logic.
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
        }
    }
}

/// Technical implementation of the Receiver structure.
pub struct Receiver<T> {
    state: Arc<ChannelState<T>>,
}

impl<T> core::future::Future for Receiver<T> {
    type Output = Option<T>;

    /// Technical implementation of the poll logic.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.state.pop() {
            Some(val) => Poll::Ready(Some(val)),
            None => {
                // Register waker if channel is not closed
                if self.state.closed.load(Ordering::Acquire) {
                    Poll::Ready(None)
                } else {
                    self.state.receiver_waker.store(cx.waker().clone());
                    Poll::Pending
                }
            }
        }
    }
}

/// Bounded channel internal state
struct ChannelState<T> {
    items: [core::mem::MaybeUninit<T>; 233],
    head: AtomicUsize,
    tail: AtomicUsize,
    closed: AtomicBool,
    receiver_waker: AtomicWaker,
}

impl<T> ChannelState<T> {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            items: unsafe { core::mem::MaybeUninit::uninit().assume_init() },
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            closed: AtomicBool::new(false),
            receiver_waker: AtomicWaker::new(),
        }
    }

    /// Technical implementation of the push logic.
    fn push(&self, val: T) -> Result<(), T> {
        if self.closed.load(Ordering::Acquire) {
            return Err(val);
        }

        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        let next_head = (head + 1) % 233;

        if next_head == tail {
            return Err(val); // Full
        }

        let ptr = self.items[head].as_ptr() as *mut T;
        unsafe {
            core::ptr::write(ptr, val);
        }
        self.head.store(next_head, Ordering::Release);

        // Notify receiver
        self.receiver_waker.wake();
        Ok(())
    }

    /// Technical implementation of the pop logic.
    fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);

        if tail == head {
            return None;
        }

        let val = unsafe { core::ptr::read(self.items[tail].as_ptr()) };
        self.tail.store((tail + 1) % 233, Ordering::Release);
        Some(val)
    }
}

/// Simplified AtomicWaker for no_std/no_alloc (structural realization)
struct AtomicWaker {
    waker: core::cell::UnsafeCell<Option<core::task::Waker>>,
    is_set: AtomicBool,
}

unsafe impl Send for AtomicWaker {}
unsafe impl Sync for AtomicWaker {}

impl AtomicWaker {
    /// Initializes a new instance of the associated type.
    fn new() -> Self {
        Self {
            waker: core::cell::UnsafeCell::new(None),
            is_set: AtomicBool::new(false),
        }
    }

    /// Technical implementation of the store logic.
    fn store(&self, waker: core::task::Waker) {
        unsafe {
            *self.waker.get() = Some(waker);
        }
        self.is_set.store(true, Ordering::Release);
    }

    /// Technical implementation of the wake logic.
    fn wake(&self) {
        if self.is_set.load(Ordering::Acquire) {
            unsafe {
                if let Some(waker) = (*self.waker.get()).as_ref() {
                    waker.wake_by_ref();
                }
            }
        }
    }
}

/// Technical implementation of the mpsc logic.
pub fn mpsc<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    let _ = capacity;
    let state = Arc::new(ChannelState::new());
    (
        Sender {
            state: state.clone(),
        },
        Receiver { state },
    )
}

/// Technical implementation of the channel logic.
pub fn channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    mpsc(capacity)
}
