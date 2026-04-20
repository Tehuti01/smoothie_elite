# SKILL 002: UNSAFE RUST - UNCHECKED PERFORMANCE MASTERY

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
                        UNSAFE RUST: UNCHECKED PERFORMANCE MASTERY
                     The Sovereign Path to Zero-Overhead Abstraction
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## EXECUTIVE SUMMARY

This skill provides comprehensive mastery of unsafe Rust, including pointer manipulation,
FFI interop, manual memory management, SIMD intrinsics, and building zero-cost abstractions.
It covers the darker arts of Rust that enable ultimate performance.

## TABLE OF CONTENTS

1. [Fundamentals of Unsafe](#fundamentals-of-unsafe)
2. [Pointer Mastery](#pointer-mastery)
3. [FFI Deep Dive](#ffi-deep-dive)
4. [SIMD Intrinsics](#simd-intrinsics)
5. [Manual Memory Management](#manual-memory-management)
6. [Zero-Cost Abstractions](#zero-cost-abstractions)
7. [Best Practices](#best-practices)
8. [Security Considerations](#security-considerations)
9. [Real-World Applications](#real-world-applications)
10. [Testing Unsafe Code](#testing-unsafe-code)

---

## FUNDAMENTALS OF UNSAFE

### 1.1 The Five Unsafe Superpowers

```rust
/// The five things only unsafe Rust can do:
/// 1. Dereference a raw pointer
/// 2. Call an unsafe function/method
/// 3. Access or modify a mutable static variable
/// 4. Implement an unsafe trait
/// 5. Read from a union or invalid enum discriminant

/// Example: Manual vtable construction
struct TraitObject {
    data: *mut (),
    vtable: *mut (),
}

trait MyTrait {
    fn do_something(&self);
}

/// Manual vtable (educational - don't do this in production)
unsafe fn create_trait_object<T: MyTrait>(value: T) -> TraitObject {
    let data = Box::into_raw(Box::new(value)) as *mut ();
    
    // Simplified vtable - real one is more complex
    let vtable = std::mem::transmute::<_, *mut ()>(
        <T as MyTrait>::do_something as *mut ()
    );
    
    TraitObject { data, vtable }
}
```

### 1.2 Unsafe Blocks Best Practices

```rust
/// Minimal unsafe blocks - encapsulate unsafe code
pub struct SafeWrapper {
    inner: *mut InnerData,
}

impl SafeWrapper {
    pub fn new() -> Self {
        let inner = Box::into_raw(Box::new(InnerData::default()));
        SafeWrapper { inner }
    }

    /// Public safe API - hides unsafe internals
    pub fn get_value(&self) -> i32 {
        unsafe { (*self.inner).value }
    }

    pub fn set_value(&self, val: i32) {
        unsafe { (*self.inner).value = val; }
    }

    /// Take ownership and cleanup
    pub fn drop(self) {
        unsafe { Box::from_raw(self.inner); }
    }
}

struct InnerData {
    value: i32,
}

impl Default for InnerData {
    fn default() -> Self {
        InnerData { value: 42 }
    }
}
```

---

## POINTER MASTERY

### 2.1 Raw Pointers (*const T, *mut T)

```rust
use std::ptr::{self, NonNull};

/// High-performance ring buffer using raw pointers
pub struct RingBuffer<T> {
    buffer: *mut T,
    capacity: usize,
    read: usize,
    write: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        let capacity = capacity.next_power_of_two();
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        
        RingBuffer {
            buffer: unsafe { std::alloc::alloc(layout) as *mut T },
            capacity,
            read: 0,
            write: 0,
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        let next_write = (self.write + 1) & (self.capacity - 1);
        if next_write == self.read {
            return false; // Full
        }

        unsafe {
            self.buffer.add(self.write).write(item);
            self.write = next_write;
        }
        true
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.read == self.write {
            return None; // Empty
        }

        let item = unsafe { self.buffer.add(self.read).read() };
        self.read = (self.read + 1) & (self.capacity - 1);
        Some(item)
    }

    pub fn is_empty(&self) -> bool {
        self.read == self.write
    }

    pub fn len(&self) -> usize {
        self.write.wrapping_sub(self.read) & (self.capacity - 1)
    }
}

impl<T> Drop for RingBuffer<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {} // Drain
        let layout = std::alloc::Layout::array::<T>(self.capacity).unwrap();
        unsafe { std::alloc::dealloc(self.buffer as *mut u8, layout); }
    }
}
```

### 2.2 NonNull for Sound Pointers

```rust
/// NonNull guarantees non-null pointers
pub struct OptimizedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

struct Node<T> {
    value: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> OptimizedList<T> {
    pub fn new() -> Self {
        OptimizedList {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let node = Box::into_raw(Box::new(Node {
            value,
            next: None,
            prev: self.tail,
        }));

        let node = unsafe { NonNull::new_unchecked(node) };

        match self.tail {
            Some(tail) => {
                unsafe { (*tail.as_ptr()).next = Some(node); }
            }
            None => {
                self.head = Some(node);
            }
        }
        self.tail = Some(node);
        self.len += 1;
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.map(|tail| {
            let node = unsafe { Box::from_raw(tail.as_ptr()) };
            self.tail = node.prev;
            if let Some(t) = self.tail {
                unsafe { (*t.as_ptr()).next = None; }
            } else {
                self.head = None;
            }
            self.len -= 1;
            node.value
        })
    }
}
```

### 2.3 Aliasing Pointers (aliasing *const T)

```rust
/// Multiple readers safely with aliasing pointers
pub struct ConcurrentReader<T: ?Sized> {
    data: *const T,
}

impl<T: ?Sized> ConcurrentReader<T> {
    pub fn new(data: &T) -> Self {
        ConcurrentReader { data }
    }

    /// Safe read through alias
    pub fn read(&self) -> &T {
        unsafe { &*self.data }
    }

    /// Copy data (useful for Copy types)
    pub fn copy(&self) -> T 
    where T: Copy 
    {
        unsafe { *self.data }
    }
}

/// Write alias for exclusive access
pub struct ExclusiveWriter<T: ?Sized> {
    data: *mut T,
}

impl<T: ?Sized> ExclusiveWriter<T> {
    pub fn new(data: &mut T) -> Self {
        ExclusiveWriter { data }
    }

    pub fn write(&mut self, value: T) {
        unsafe { ptr::write(&mut *self.data, value); }
    }
}
```

---

## FFI DEEP DIVE

### 3.1 C Interop Basics

```rust
use std::os::raw::c_char;
use std::ffi::CStr;

/// C function declarations
extern "C" {
    fn c_malloc(size: usize) -> *mut std::os::raw::c_void;
    fn c_free(ptr: *mut std::os::raw::c_void);
    fn c_print_string(s: *const c_char);
    fn c_calculate(x: f64, y: f64) -> f64;
}

/// Safe wrapper around C functions
pub struct CMemory {
    ptr: *mut std::os::raw::c_void,
    size: usize,
}

impl CMemory {
    pub fn new(size: usize) -> Option<Self> {
        unsafe {
            let ptr = c_malloc(size);
            if ptr.is_null() {
                None
            } else {
                Some(CMemory { ptr, size })
            }
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(self.ptr as *const u8, self.size)
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr as *mut u8, self.size)
        }
    }
}

impl Drop for CMemory {
    fn drop(&mut self) {
        unsafe { c_free(self.ptr); }
    }
}

/// Safe C string handling
pub fn print_c_string(s: &str) {
    let c_string = std::ffi::CString::new(s).unwrap();
    unsafe { c_print_string(c_string.as_ptr()); }
}

/// Reading C strings safely
pub fn from_c_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
```

### 3.2 Complex C++ Interop

```rust
/// C++ class wrapper
#[repr(C)]
pub struct CppVector {
    data: *mut f64,
    size: usize,
    capacity: usize,
}

impl CppVector {
    pub fn new() -> Self {
        extern "C" {
            fn cpp_vector_create() -> *mut CppVector;
        }
        unsafe { *CppVector::new() }
    }

    extern "C" fn create() -> Box<CppVector> {
        unsafe { Box::new(CppVector::default()) }
    }

    pub fn push(&mut self, value: f64) {
        if self.size >= self.capacity {
            self.grow();
        }
        unsafe {
            *self.data.add(self.size) = value;
        }
        self.size += 1;
    }

    fn grow(&mut self) {
        let new_cap = self.capacity * 2 + 1;
        let layout = std::alloc::Layout::array::<f64>(new_cap).unwrap();
        let new_data = unsafe { std::alloc::alloc(layout) as *mut f64 };
        
        unsafe {
            std::ptr::copy_nonoverlapping(self.data, new_data, self.size);
            std::alloc::dealloc(self.data as *mut u8, 
                std::alloc::Layout::array::<f64>(self.capacity).unwrap());
        }
        
        self.data = new_data;
        self.capacity = new_cap;
    }

    pub fn get(&self, index: usize) -> f64 {
        assert!(index < self.size);
        unsafe { *self.data.add(index) }
    }
}

impl Default for CppVector {
    fn default() -> Self {
        CppVector {
            data: std::ptr::null_mut(),
            size: 0,
            capacity: 0,
        }
    }
}

impl Drop for CppVector {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let layout = std::alloc::Layout::array::<f64>(self.capacity).unwrap();
            unsafe { std::alloc::dealloc(self.data as *mut u8, layout); }
        }
    }
}
```

### 3.3 syscall Interface

```rust
use std::os::unix::io::{AsRawFd, RawFd};

/// Direct syscall wrapper
pub struct Syscall {
    fd: RawFd,
}

impl Syscall {
    pub fn open(path: &str, flags: i32) -> Result<RawFd, std::io::Error> {
        use std::os::unix::ffi::OsStrExt;
        
        let path = std::ffi::OsStr::new(path);
        let path_ptr = path.as_bytes().as_ptr() as *const i8;
        
        let ret = unsafe {
            libc::syscall(libc::SYS_open, path_ptr, flags)
        };
        
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as RawFd)
        }
    }

    pub fn read(fd: RawFd, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        let ret = unsafe {
            libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len())
        };
        
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    }

    pub fn write(fd: RawFd, buf: &[u8]) -> Result<usize, std::io::Error> {
        let ret = unsafe {
            libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len())
        };
        
        if ret < 0 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(ret as usize)
        }
    }
}
```

---

## SIMD INTRINSICS

### 4.1 Portable SIMD

```rust
use std::simd::{Simd, SimdUint, SimdFloat, lane_count};

/// SIMD-accelerated vector operations
#[derive(Clone, Copy)]
pub struct Vec4(Simd<f32, 4>);

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4(Simd::from_array([x, y, z, w]))
    }

    pub fn splat(x: f32) -> Self {
        Vec4(Simd::splat(x))
    }

    pub fn add(self, other: Vec4) -> Vec4 {
        Vec4(self.0 + other.0)
    }

    pub fn sub(self, other: Vec4) -> Vec4 {
        Vec4(self.0 - other.0)
    }

    pub fn mul(self, other: Vec4) -> Vec4 {
        Vec4(self.0 * other.0)
    }

    pub fn dot(self, other: Vec4) -> f32 {
        (self * other).0.reduce_sum()
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn normalize(self) -> Vec4 {
        let len = self.length();
        if len > 0.0 {
            self / Vec4::splat(len)
        } else {
            self
        }
    }

    pub fn cross(self, other: Vec4) -> Vec4 {
        let a = self.0;
        let b = other.0;
        let a_yzx = Simd::rotate_elements_right::<1>(a);
        let b_yzx = Simd::rotate_elements_right::<1>(b);
        let a_zxy = Simd::rotate_elements_right::<2>(a);
        let b_zxy = Simd::rotate_elements_right::<2>(b);
        Vec4(a_yzx * b_zxy - a_zxy * b_yzx)
    }
}

impl std::ops::Div<Vec4> for Vec4 {
    type Output = Vec4;
    fn div(self, other: Vec4) -> Vec4 {
        Vec4(self.0 / other.0)
    }
}

/// SIMD-accelerated array processing
pub fn simd_sum(values: &[f32]) -> f32 {
    use std::simd::num::SimdFloat;
    
    let mut sum = Simd::<f32, 4>::splat(0.0);
    let mut i = 0;
    
    let chunks = values.chunks_exact(4);
    let remainder = chunks.remainder();
    
    for chunk in chunks {
        let v = Simd::from_array(chunk.try_into().unwrap());
        sum += v;
    }
    
    let mut result = sum.reduce_sum();
    for &r in remainder.iter() {
        result += r;
    }
    
    result
}

/// SIMD dot product
pub fn simd_dot(a: &[f32], b: &[f32]) -> f32 {
    use std::simd::num::SimdFloat;
    
    assert_eq!(a.len(), b.len());
    
    let mut sum = Simd::<f32, 4>::splat(0.0);
    let chunks = a.chunks_exact(4);
    let remainder = chunks.remainder();
    
    for (av, bv) in chunks.zip(b.chunks_exact(4)) {
        let av = Simd::from_array(av.try_into().unwrap());
        let bv = Simd::from_array(bv.try_into().unwrap());
        sum += av * bv;
    }
    
    let mut result = sum.reduce_sum();
    for (av, bv) in remainder.iter().zip(&remainder) {
        result += av * bv;
    }
    
    result
}
```

### 4.2 SIMD String Processing

```rust
/// SIMD-accelerated string search
pub fn simd_find_byte(haystack: &[u8], needle: u8) -> Option<usize> {
    use std::simd::SimdUint;
    
    let simd_len = std::simd::LaneCount::<32>::MAX;
    let mut i = 0;
    
    while i + simd_len <= haystack.len() {
        let chunk = Simd::<u8, 32>::from_slice(&haystack[i..]);
        let needle_simd = Simd::splat(needle);
        
        let matches = chunk.lanes_eq(needle_simd);
        if matches.any() {
            return Some(i + matches.to_bitmask().trailing_zeros() as usize);
        }
        
        i += simd_len;
    }
    
    // Fallback for remainder
    haystack[i..].iter().position(|&b| b == needle).map(|p| i + p)
}

/// SIMD vectorizedmemchr
pub fn simd_memchr(haystack: &[u8], needle: u8) -> Option<usize> {
    simd_find_byte(haystack, needle)
}
```

---

## MANUAL MEMORY MANAGEMENT

### 5.1 Custom Box Implementation

```rust
/// Custom Box with custom allocator
pub struct CustomBox<T, A: Allocator> {
    ptr: NonNull<T>,
    alloc: A,
}

unsafe impl<T: Send, A: Send> Send for CustomBox<T, A> {}
unsafe impl<T: Sync, A: Sync> Sync for CustomBox<T, A> {}

impl<T, A: Allocator> CustomBox<T, A> {
    pub fn new_in(alloc: A, value: T) -> Self {
        let layout = Layout::new::<T>();
        let ptr = unsafe {
            let ptr = alloc.allocate(layout).expect("allocation failed") as *mut T;
            ptr.write(value);
            NonNull::new_unchecked(ptr)
        };
        
        CustomBox { ptr, alloc }
    }

    pub fn into_raw(b: CustomBox<T, A>) -> *mut T {
        let ptr = b.ptr.as_ptr();
        std::mem::forget(b);
        ptr
    }

    pub fn from_raw_in(ptr: *mut T, alloc: A) -> Self {
        unsafe {
            CustomBox {
                ptr: NonNull::new_unchecked(ptr),
                alloc,
            }
        }
    }

    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr.as_ptr() }
    }

    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr.as_ptr() }
    }
}

impl<T, A: Allocator> Drop for CustomBox<T, A> {
    fn drop(&mut self) {
        unsafe {
            self.ptr.as_ptr().drop_in_place();
            let layout = Layout::new::<T>();
            self.alloc.deallocate(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}
```

### 5.2 Reference Counted with Custom Allocator

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

/// Custom Arc with custom allocator
pub struct Arc<T, A: Allocator> {
    ptr: NonNull<ArcInner<T, A>>,
}

struct ArcInner<T, A: Allocator> {
    ref_count: AtomicUsize,
    data: T,
    alloc: A,
}

impl<T, A: Allocator + Default> Arc<T, A> {
    pub fn new(value: T) -> Self {
        A::default().allocate(Layout::new::<ArcInner<T, A>>())
            .map(|mem| {
                unsafe {
                    let ptr = mem as *mut ArcInner<T, A>;
                    ptr.write(ArcInner {
                        ref_count: AtomicUsize::new(1),
                        data: value,
                        alloc: A::default(),
                    });
                    Arc { ptr: NonNull::new_unchecked(ptr) }
                }
            }).unwrap()
    }

    pub fn get(&self) -> &T {
        unsafe { &(*self.ptr.as_ptr()).data }
    }

    pub fn clone(&self) -> Self {
        unsafe {
            (*self.ptr.as_ptr()).ref_count.fetch_add(1, Ordering::Relaxed);
            Arc { ptr: self.ptr }
        }
    }
}

impl<T, A: Allocator> Drop for Arc<T, A> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr.as_ptr()).ref_count.fetch_sub(1, Ordering::Release) == 1 {
                std::sync::atomic::fence(Ordering::Acquire);
                let layout = Layout::new::<ArcInner<T, A>>();
                let ptr = self.ptr.as_ptr() as *mut u8;
                (*self.ptr.as_ptr()).data.drop();
                A::default().deallocate(ptr, layout);
            }
        }
    }
}
```

---

## ZERO-COST ABSTRACTIONS

### 6.1 Iterator Optimization

```rust
/// Zero-cost iterator wrapper
pub struct IterChainer<T> {
    data: Vec<T>,
    index: usize,
}

impl<T> IterChainer<T> {
    pub fn new(data: Vec<T>) -> Self {
        IterChainer { data, index: 0 }
    }

    pub fn filter<F>(self, f: F) -> FilterIter<T, F> 
    where F: FnMut(&T) -> bool 
    {
        FilterIter {
            inner: self,
            predicate: f,
        }
    }

    pub fn map<U, F>(self, f: F) -> MapIter<T, U, F> 
    where F: FnMut(T) -> U 
    {
        MapIter {
            inner: self,
            func: f,
        }
    }

    pub fn collect<B: FromIterator<T>>(self) -> B {
        self.collect()
    }
}

impl<T> Iterator for IterChainer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let item = self.data[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// Compile-time optimized iterator
pub struct SimdIterator<T> {
    slice: *const [T],
    index: usize,
}

impl<T> SimdIterator<T> {
    pub fn new(slice: &[T]) -> Self {
        SimdIterator {
            slice: slice as *const [T],
            index: 0,
        }
    }
}

impl<T: Copy> Iterator for SimdIterator<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.index < unsafe { (*self.slice).len() } {
            let item = unsafe { (*self.slice)[self.index] };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}
```

### 6.2 Inline Closures

```rust
/// Inline-optimized operations
#[inline(always)]
pub fn hot_path(value: i32) -> i32 {
    value * 2 + 1
}

/// Branchless conditional
#[inline]
pub fn branchless_select<T>(cond: bool, a: T, b: T) -> T {
    let mask = -(cond as i32) as usize;
    let a = a as usize;
    let b = b as usize;
    (a & mask) | (b & !mask) as T
}

/// Likely/unlikely hints
#[inline]
pub fn likely(b: bool) -> bool {
    if std::cfg!(feature = "profile") {
        b
    } else {
        unsafe { std::hint::likely(b) }
    }
}
```

---

## BEST PRACTICES

### 7.1 Safety Guidelines

```rust
/// Rule 1: Minimize unsafe surface
mod internal {
    pub struct InternalState {
        ptr: *mut (),
    }
    
    impl InternalState {
        pub fn new() -> Self {
            // Complex unsafe initialization
            InternalState { ptr: std::ptr::null_mut() }
        }
        
        /// Safe public API
        pub fn get_value(&self) -> i32 {
            unsafe { /* ... */ 42 }
        }
    }
}

/// Rule 2: Document safety invariants
/// # Safety
/// Caller must ensure:
/// - `ptr` is valid for `len` bytes
/// - `ptr` is properly aligned
/// - No other threads access this memory
pub unsafe fn dangerous_operation(ptr: *mut u8, len: usize) {
    // ...
}

/// Rule 3: Use RAII for cleanup
pub struct GuardedResource {
    handle: RawHandle,
}

impl GuardedResource {
    pub fn acquire() -> Result<Self, Error> {
        // Acquire resource
        Ok(GuardedResource { handle })
    }
}

impl Drop for GuardedResource {
    fn drop(&mut self) {
        // Release resource
    }
}
```

### 7.2 Testing Unsafe Code

```rust
#[cfg(test)]
mod unsafe_tests {
    use super::*;

    #[test]
    fn test_ring_buffer_properties() {
        let mut rb: RingBuffer<i32> = RingBuffer::new(8);
        
        // Property: empty buffer returns None on pop
        assert_eq!(rb.pop(), None);
        
        // Property: push then pop returns same value
        rb.push(42);
        assert_eq!(rb.pop(), Some(42));
        
        // Property: wrap around works
        for i in 0..7 {
            rb.push(i);
        }
        assert!(!rb.push(100)); // Should be full
        
        // Property: FIFO ordering
        let expected: Vec<_> = (0..7).collect();
        let actual: Vec<_> = std::iter::from_fn(|| rb.pop()).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_simd_correctness() {
        let a = vec![1.0f32; 1000];
        let b = vec![2.0f32; 1000];
        
        let simd_result = simd_dot(&a, &b);
        let expected: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        
        assert!((simd_result - expected).abs() < 1e-6);
    }
}
```

---

## SECURITY CONSIDERATIONS

### 8.1 Memory Safety

```rust
/// Bounds-checked access (safe wrapper)
pub struct SafeSlice<T> {
    data: *const T,
    len: usize,
}

impl<T> SafeSlice<T> {
    pub fn new(data: &[T]) -> Self {
        SafeSlice {
            data: data.as_ptr(),
            len: data.len(),
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            Some(unsafe { &*self.data.add(index) })
        } else {
            None
        }
    }

    /// Constant-time access pattern
    pub fn get_unchecked(&self, index: usize) -> &T {
        assert!(index < self.len);
        unsafe { &*self.data.add(index) }
    }
}

/// Validate pointer before use
pub unsafe fn validate_ptr<T>(ptr: *const T) -> Result<&'static T, Error> {
    if ptr.is_null() {
        return Err(Error::NullPointer);
    }
    
    // Check alignment
    if ptr as usize % std::mem::align_of::<T>() != 0 {
        return Err(Error::Misaligned);
    }
    
    Ok(&*ptr)
}
```

---

## REAL-WORLD APPLICATIONS

### 9.1 High-Performance Networking

```rust
/// Zero-copy packet processing
pub struct PacketBuffer {
    data: *mut u8,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
}

impl PacketBuffer {
    pub fn receive_packet(&mut self, fd: RawFd) -> Result<usize, Error> {
        let available = self.capacity - self.write_pos;
        if available < 4096 {
            return Err(Error::BufferFull);
        }
        
        let ret = unsafe {
            libc::recv(
                fd,
                self.data.add(self.write_pos) as *mut libc::c_void,
                available,
                0,
            )
        };
        
        if ret > 0 {
            self.write_pos += ret as usize;
        }
        
        Ok(ret as usize)
    }

    pub fn parse_header(&self) -> Option<PacketHeader> {
        if self.write_pos - self.read_pos < 16 {
            return None;
        }
        
        unsafe {
            Some(PacketHeader {
                magic: u32::from_le_bytes(*self.data.add(0)),
                version: *self.data.add(4),
                length: u16::from_le_bytes(*self.data.add(6)),
                // ...
            })
        }
    }
}
```

### 9.2 Game Engine ECS

```rust
/// Archetype-based ECS component storage
pub struct ComponentArray<T> {
    data: *mut T,
    capacity: usize,
    active_count: usize,
}

impl<T> ComponentArray<T> {
    pub fn new(capacity: usize) -> Self {
        let layout = std::alloc::Layout::array::<T>(capacity).unwrap();
        let data = unsafe { std::alloc::alloc(layout) as *mut T };
        
        ComponentArray {
            data,
            capacity,
            active_count: 0,
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) -> bool {
        let index = entity.index();
        if index >= self.capacity {
            return false;
        }
        
        unsafe {
            self.data.add(index).write(component);
        }
        self.active_count += 1;
        true
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        let index = entity.index();
        if index >= self.capacity {
            return None;
        }
        
        unsafe {
            let ptr = self.data.add(index);
            if ptr.as_ref().is_some() {
                Some(&*ptr)
            } else {
                None
            }
        }
    }
}
```

---

## RECAP

### Key Takeaways

1. **Minimize unsafe** - Keep it localized and well-encapsulated
2. **Document invariants** - Write safety docs for every unsafe function
3. **Test extensively** - Property testing catches edge cases
4. **Use SIMD** - Portable SIMD for cross-platform vectorization
5. **FFI carefully** - Validate all boundary conditions
6. **Profile first** - Measure before optimizing

### Next Steps

- Explore CPU-specific intrinsics
- Implement lock-free data structures
- Build custom allocators with jemalloc

---

*Skill ID: 002 | Category: Core-Language | Complexity: Expert*
*Version: 1.0.0 | Last Updated: 2024*
