/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0d2a3e6b | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/lock-free-queue/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem::MaybeUninit;
use alloc::vec::Vec;

/// Cache line size for optimal cache performance
const CACHE_LINE: usize = 64;

///
/// - `T: Copy` - Element type must be Copy for lock-free operation
/// # Properties:
/// - Circular buffer with atomic indices
/// - Zero allocation after initialization
/// Technical implementation of the LockFreeQueue structure.
pub struct LockFreeQueue<T: Copy> {
    buffer: Vec<MaybeUninit<T>>,
    capacity: usize,
    capacity_mask: usize,

    // Padded for cache-line alignment to prevent false sharing
    head: CacheLinePadded<AtomicUsize>,
    tail: CacheLinePadded<AtomicUsize>,
}

/// Cache-line padded atomic for false-sharing prevention
struct CacheLinePadded<T> {
    value: T,
    _padding: [u8; CACHE_LINE - core::mem::size_of::<AtomicUsize>()],
}

impl<T: Copy> LockFreeQueue<T> {
    /// Create a new lock-free queue with specified capacity
    ///
    /// # Arguments
    /// * `capacity` - Must be a power of 2 (256, 512, 1024, etc.)
    ///
    /// # Panics
    /// If capacity is not a power of 2 or is zero
    pub fn new(capacity: usize) -> Self {
        assert!(capacity.is_power_of_two(), "Capacity must be power of 2");
        assert!(capacity > 0, "Capacity must be positive");

        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buffer.push(MaybeUninit::uninit());
        }

        Self {
            buffer,
            capacity,
            capacity_mask: capacity - 1,
            head: CacheLinePadded {
                value: AtomicUsize::new(0),
                _padding: [0; CACHE_LINE - core::mem::size_of::<AtomicUsize>()],
            },
            tail: CacheLinePadded {
                value: AtomicUsize::new(0),
                _padding: [0; CACHE_LINE - core::mem::size_of::<AtomicUsize>()],
            },
        }
    }

    /// Enqueue an element into the queue
    ///
    /// Returns true if enqueued successfully, false if queue is full
    pub fn enqueue(&mut self, value: T) -> bool {
        let tail = self.tail.value.load(Ordering::Acquire);
        let next_tail = (tail + 1) & self.capacity_mask;

        let head = self.head.value.load(Ordering::Acquire);

        if next_tail == head {
            // Queue is full
            return false;
        }

        // SAFETY: We've verified the index is within capacity and not the head position
        unsafe {
            self.buffer[tail].write(value);
        }

        self.tail.value.store(next_tail, Ordering::Release);
        true
    }

    /// Dequeue an element from the queue
    ///
    /// Returns Some(element) if dequeue successful, None if queue is empty
    pub fn dequeue(&mut self) -> Option<T> {
        let head = self.head.value.load(Ordering::Acquire);
        let tail = self.tail.value.load(Ordering::Acquire);

        if head == tail {
            // Queue is empty
            return None;
        }

        // SAFETY: Head is always at a valid written position when head != tail
        let value = unsafe { self.buffer[head].assume_init() };

        let next_head = (head + 1) & self.capacity_mask;
        self.head.value.store(next_head, Ordering::Release);

        Some(value)
    }

    /// Check if queue is empty without dequeuing
    pub fn is_empty(&self) -> bool {
        self.head.value.load(Ordering::Acquire) == self.tail.value.load(Ordering::Acquire)
    }

    /// Check if queue is full
    pub fn is_full(&self) -> bool {
        let tail = self.tail.value.load(Ordering::Acquire);
        let next_tail = (tail + 1) & self.capacity_mask;
        next_tail == self.head.value.load(Ordering::Acquire)
    }

    /// Get current number of elements in queue
    pub fn len(&self) -> usize {
        let head = self.head.value.load(Ordering::Acquire);
        let tail = self.tail.value.load(Ordering::Acquire);

        if tail >= head {
            tail - head
        } else {
            self.capacity - head + tail
        }
    }

    /// Get queue capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clear the queue
    pub fn clear(&mut self) {
        self.head.value.store(0, Ordering::Release);
        self.tail.value.store(0, Ordering::Release);
    }
}

/// Technical implementation of the SpscQueue structure.
pub struct SpscQueue<T: Copy> {
    inner: LockFreeQueue<T>,
}

impl<T: Copy> SpscQueue<T> {
    /// Initializes a new instance of the associated type.
    pub fn new(capacity: usize) -> Self {
        Self {
            inner: LockFreeQueue::new(capacity),
        }
    }

    /// Producer side: enqueue element
    pub fn enqueue(&mut self, value: T) -> bool {
        self.inner.enqueue(value)
    }

    /// Consumer side: dequeue element
    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.dequeue()
    }

    /// Technical implementation of the is_empty logic.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Technical implementation of the len logic.
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_enqueue_dequeue logic.
    fn test_enqueue_dequeue() {
        let mut queue: LockFreeQueue<i32> = LockFreeQueue::new(256);
        assert!(queue.enqueue(42));
        assert_eq!(queue.dequeue(), Some(42));
        assert!(queue.is_empty());
    }

    #[test]
    /// Technical implementation of the test_queue_full logic.
    fn test_queue_full() {
        let mut queue: LockFreeQueue<i32> = LockFreeQueue::new(4);
        assert!(queue.enqueue(1));
        assert!(queue.enqueue(2));
        assert!(queue.enqueue(3));
        assert!(!queue.enqueue(4)); // Queue is full
    }

    #[test]
    /// Technical implementation of the test_spsc_queue logic.
    fn test_spsc_queue() {
        let mut queue: SpscQueue<f32> = SpscQueue::new(512);
        for i in 0..100 {
            assert!(queue.enqueue(i as f32));
        }
        for i in 0..100 {
            assert_eq!(queue.dequeue(), Some(i as f32));
        }
    }
}
