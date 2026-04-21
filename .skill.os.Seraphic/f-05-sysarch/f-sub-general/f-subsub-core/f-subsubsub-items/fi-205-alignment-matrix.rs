---
id: fi-205-alignment-matrix.rs
category: f-05-sysarch
---

use core::ops::{Deref, DerefMut};

#[repr(align(64))]
pub struct CacheAligned<T> { pub value: T }
impl<T> CacheAligned<T> { pub const fn new(value: T) -> Self { Self { value } } }
impl<T> Deref for CacheAligned<T> { type Target = T; fn deref(&self) -> &T { &self.value } }
impl<T> DerefMut for CacheAligned<T> { fn deref_mut(&mut self) -> &mut T { &mut self.value } }

pub struct PhiPad144 { _data: [u8; 144] }
impl Default for PhiPad144 { fn default() -> Self { Self { _data: [0; 144] } } }

pub struct SiliconIsolated<T> { pub value: T, _pad_before: PhiPad144, _pad_after: PhiPad144 }
impl<T> SiliconIsolated<T> { pub const fn new(value: T) -> Self { Self { value, _pad_before: PhiPad144 { _data: [0; 144] }, _pad_after: PhiPad144 { _data: [0; 144] } } } }
