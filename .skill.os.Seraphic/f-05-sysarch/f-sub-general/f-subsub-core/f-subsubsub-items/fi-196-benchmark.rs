---
id: fi-196-benchmark.rs
category: f-05-sysarch
---

use std::thread;
use std::sync::Arc;
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering, AtomicU32, AtomicBool, AtomicPtr};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ptr;

// 🌌 SERAPHIC SKILL RS-011: WAIT-FREE ATOMIC FABRIC (RUST BENCHMARK)
// Native Implementation for Industrial Quality Certification.

#[repr(align(64))]
pub struct SpscBuffer<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    head: AtomicUsize,
    tail: AtomicUsize,
}

unsafe impl<T, const N: usize> Sync for SpscBuffer<T, N> {}

impl<T, const N: usize> SpscBuffer<T, N> {
    pub fn new() -> Self {
        let buffer = unsafe { MaybeUninit::<[UnsafeCell<MaybeUninit<T>>; N]>::uninit().assume_init() };
        Self { buffer, head: AtomicUsize::new(0), tail: AtomicUsize::new(0) }
    }

    pub fn push(&self, value: T) -> Option<()> {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Acquire);
        if (head + 1) % N == tail { return None; }
        unsafe { *self.buffer[head].get() = MaybeUninit::new(value); }
        self.head.store((head + 1) % N, Ordering::Release);
        Some(())
    }

    pub fn pop(&self) -> Option<T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let head = self.head.load(Ordering::Acquire);
        if tail == head { return None; }
        let value = unsafe { (*self.buffer[tail].get()).as_ptr().read() };
        self.tail.store((tail + 1) % N, Ordering::Release);
        Some(value)
    }
}

fn main() {
    println!("--- SERAPHIC ATOMIC STRESS TEST (NATIVE RUST) ---");
    let iterations = 1_000_000;
    let start = Instant::now();
    
    let buffer = Arc::new(SpscBuffer::<u32, 1024>::new());
    let b_clone = buffer.clone();

    let producer = thread::spawn(move || {
        for i in 0..iterations {
            while b_clone.push(i).is_none() {
                std::hint::spin_loop();
            }
        }
    });

    for i in 0..iterations {
        loop {
            if let Some(val) = buffer.pop() {
                if val % 200_000 == 0 { println!("   [L0] Processed {} iterations...", val); }
                break;
            }
            std::hint::spin_loop();
        }
    }

    producer.join().unwrap();
    let duration = start.elapsed();
    println!("✅ SUCCESS: {} operations in {:?}", iterations, duration);
    println!("   Industrial Throughput: {:.2} ops/sec", iterations as f64 / duration.as_secs_f64());
}
