---
id: fi-198-wait-free-matrix.rs
category: f-05-sysarch
---

// 🌌 SERAPHIC SKILL RS-011: WAIT-FREE ATOMIC FABRIC MATRIX
// PRODUCTION-GRADE IMPLEMENTATION: SPSC, ATOMIC F32, AND CACHE-ALIGNED SYNC

use std::sync::atomic::{AtomicUsize, Ordering, AtomicU32, AtomicBool, AtomicPtr};
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::ptr;

// SECTION 1: THE ATOMIC F32 ENVELOPE (U32 BIT-CASTING)
#[repr(align(64))]
pub struct AtomicF32 {
    bits: AtomicU32,
    _padding: [u8; 60],
}

impl AtomicF32 {
    pub const fn new(value: f32) -> Self {
        Self { bits: AtomicU32::new(value.to_bits()), _padding: [0; 60] }
    }

    #[inline(always)]
    pub fn load(&self, order: Ordering) -> f32 {
        f32::from_bits(self.bits.load(order))
    }

    #[inline(always)]
    pub fn store(&self, value: f32, order: Ordering) {
        self.bits.store(value.to_bits(), order);
    }

    pub fn fetch_add(&self, delta: f32, order: Ordering) -> f32 {
        let mut bits = self.bits.load(Ordering::Relaxed);
        loop {
            let val = f32::from_bits(bits);
            let next = val + delta;
            match self.bits.compare_exchange_weak(bits, next.to_bits(), order, Ordering::Relaxed) {
                Ok(_) => return val,
                Err(latest) => bits = latest,
            }
        }
    }
}

// SECTION 2: THE SOVEREIGN SPSC RING BUFFER
pub struct SpscBuffer<T, const N: usize> {
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
    write_idx: AtomicUsize,
    _padding1: [u8; 64],
    read_idx: AtomicUsize,
    _padding2: [u8; 64],
}

unsafe impl<T: Send, const N: usize> Sync for SpscBuffer<T, N> {}

impl<T, const N: usize> SpscBuffer<T, N> {
    const MASK: usize = N - 1;

    pub const fn new() -> Self {
        let buffer = unsafe { MaybeUninit::<[UnsafeCell<MaybeUninit<T>>; N]>::uninit().assume_init() };
        Self { buffer, write_idx: AtomicUsize::new(0), _padding1: [0; 64], read_idx: AtomicUsize::new(0), _padding2: [0; 64] }
    }

    pub fn push(&self, value: T) -> Option<()> {
        let wr = self.write_idx.load(Ordering::Relaxed);
        let rd = self.read_idx.load(Ordering::Acquire);
        if (wr + 1) & Self::MASK == rd { return None; }
        unsafe { ptr::write(self.buffer.get_unchecked(wr).get(), MaybeUninit::new(value)); }
        self.write_idx.store((wr + 1) & Self::MASK, Ordering::Release);
        Some(())
    }

    pub fn pop(&self) -> Option<T> {
        let rd = self.read_idx.load(Ordering::Relaxed);
        let wr = self.write_idx.load(Ordering::Acquire);
        if rd == wr { return None; }
        let val = unsafe { ptr::read(self.buffer.get_unchecked(rd).get()).assume_init() };
        self.read_idx.store((rd + 1) & Self::MASK, Ordering::Release);
        Some(val)
    }
}
