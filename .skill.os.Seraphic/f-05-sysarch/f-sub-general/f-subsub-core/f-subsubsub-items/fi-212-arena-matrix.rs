---
id: fi-212-arena-matrix.rs
category: f-05-sysarch
---

use core::sync::atomic::{AtomicUsize, Ordering};
use core::cell::UnsafeCell;

pub struct BumpAllocator<const N: usize> { buffer: UnsafeCell<[u8; N]>, cursor: AtomicUsize }
unsafe impl<const N: usize> Sync for BumpAllocator<N> {}
impl<const N: usize> BumpAllocator<N> {
    pub const fn new() -> Self { Self { buffer: UnsafeCell::new([0; N]), cursor: AtomicUsize::new(0) } }
    pub fn alloc<T>(&self) -> Option<&mut T> {
        let size = core::mem::size_of::<T>();
        let align = core::mem::align_of::<T>();
        loop {
            let start = self.cursor.load(Ordering::Acquire);
            let aligned = (start + align - 1) & !(align - 1);
            let next = aligned + size;
            if next > N { return None; }
            if self.cursor.compare_exchange_weak(start, next, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
                unsafe { return Some(&mut *((self.buffer.get() as *mut u8).add(aligned) as *mut T)); }
            }
        }
    }
}
