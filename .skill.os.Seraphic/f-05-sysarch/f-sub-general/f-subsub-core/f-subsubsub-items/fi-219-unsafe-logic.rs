---
id: fi-219-unsafe-logic.rs
category: f-05-sysarch
---

pub unsafe fn fast_copy(src: *const f32, dst: *mut f32, count: usize) {
    std::ptr::copy_nonoverlapping(src, dst, count);
}
