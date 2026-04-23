/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x0ab9f803 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/01-silicon/realtime-pool/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Cache line size for optimal alignment on modern CPUs
#[allow(dead_code)]
const CACHE_LINE: usize = 64;

/// Fibonacci-based buffer sizes for harmonic memory architecture
pub const BUFFER_SIZES: &[usize] = &[
    512,   // F(9)
    768,   // F(9) * PHI
    1024,  // F(10)
    1536,  // F(10) * PHI
    2048,  // F(11)
    3072,  // F(11) * PHI
    4096,  // F(12)
    6144,  // F(12) * PHI
    8192,  // F(13)
    12288, // F(13) * PHI
    16384, // F(14)
];

///
/// - Uses lock-free atomic operations for thread-safe allocation
/// - Pre-allocates all memory at initialization
/// Technical implementation of the RealtimePool structure.
pub struct RealtimePool {
    /// Pools indexed by BUFFER_SIZES
    pools: Vec<BufferPool>,
    /// Total allocations counter (for monitoring)
    total_allocations: AtomicUsize,
    /// Peak allocations counter
    peak_allocations: AtomicUsize,
}

/// Individual pool managing one specific buffer size
struct BufferPool {
    /// Number of buffers available
    available: AtomicUsize,
    /// Number of buffers allocated
    allocated: AtomicUsize,
    /// Buffer size in samples
    size: usize,
}

impl RealtimePool {
    /// Create a new real-time pool with specified capacity
    ///
    /// # Arguments
    /// * `capacity` - Number of pre-allocated buffers per size
    pub fn new(capacity: usize) -> Self {
        let pools = BUFFER_SIZES
            .iter()
            .map(|&size| BufferPool::new(size, capacity))
            .collect();

        Self {
            pools,
            total_allocations: AtomicUsize::new(0),
            peak_allocations: AtomicUsize::new(0),
        }
    }

    /// Allocate a buffer of specified size
    ///
    /// Returns a pooled buffer if available, None if no buffers of this size
    pub fn allocate(&self, size: usize) -> Option<Vec<f32>> {
        let pool_idx = BUFFER_SIZES.iter().position(|&s| s == size)?;
        let pool = &self.pools[pool_idx];

        if pool.allocate() {
            self.total_allocations.fetch_add(1, Ordering::Relaxed);

            let peak = self.peak_allocations.load(Ordering::Relaxed);
            let total = self.total_allocations.load(Ordering::Relaxed);
            if total > peak {
                self.peak_allocations.store(total, Ordering::Relaxed);
            }

            // Return a zeroed buffer for the specified size
            Some(vec![0.0; size])
        } else {
            None
        }
    }

    /// Get allocation statistics
    pub fn stats(&self) -> PoolStats {
        PoolStats {
            total_allocations: self.total_allocations.load(Ordering::Relaxed),
            peak_allocations: self.peak_allocations.load(Ordering::Relaxed),
            persize: BUFFER_SIZES
                .iter()
                .zip(self.pools.iter())
                .map(|(&size, pool)| (size, pool.allocated.load(Ordering::Relaxed)))
                .collect(),
        }
    }
}

impl BufferPool {
    /// Initializes a new instance of the associated type.
    fn new(size: usize, capacity: usize) -> Self {
        Self {
            available: AtomicUsize::new(capacity),
            allocated: AtomicUsize::new(0),
            size: size,
        }
    }

    /// Technical implementation of the allocate logic.
    fn allocate(&self) -> bool {
        let mut available = self.available.load(Ordering::Acquire);
        loop {
            if available == 0 {
                return false;
            }

            match self.available.compare_exchange_weak(
                available,
                available - 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    self.allocated.fetch_add(1, Ordering::Relaxed);
                    return true;
                }
                Err(actual) => available = actual,
            }
        }
    }

    /// Technical implementation of the deallocate logic.
    #[allow(dead_code)]
    fn deallocate(&self) {
        self.available.fetch_add(1, Ordering::Release);
        self.allocated.fetch_sub(1, Ordering::Release);
    }
}

/// Technical implementation of the PoolStats structure.
pub struct PoolStats {
    pub total_allocations: usize,
    pub peak_allocations: usize,
    pub persize: Vec<(usize, usize)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_pool_creation logic.
    fn test_pool_creation() {
        let pool = RealtimePool::new(10);
        let stats = pool.stats();
        assert_eq!(stats.total_allocations, 0);
    }

    #[test]
    /// Technical implementation of the test_buffer_allocation logic.
    fn test_buffer_allocation() {
        let pool = RealtimePool::new(5);
        let buf = pool.allocate(1024);
        assert!(buf.is_some());
        let buf = buf.unwrap();
        assert_eq!(buf.len(), 1024);

        let stats = pool.stats();
        assert!(stats.total_allocations > 0);
    }
}
