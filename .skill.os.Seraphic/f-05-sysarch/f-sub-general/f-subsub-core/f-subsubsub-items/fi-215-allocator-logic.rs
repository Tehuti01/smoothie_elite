---
id: fi-215-allocator-logic.rs
category: f-05-sysarch
---

use std::alloc::{GlobalAlloc, Layout};
pub struct SovereignAlloc;
unsafe impl GlobalAlloc for SovereignAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { std::alloc::System.alloc(layout) }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) { std::alloc::System.dealloc(ptr, layout) }
}
