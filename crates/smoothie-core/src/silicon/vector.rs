//! Manual Loop Unrolling & SIMD Vectorization
//! Processes multiple data points (like audio samples or packet headers) in a single CPU clock cycle.
//! Manually expands loops so the CPU can execute multiple iterations in parallel.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Process 8 audio samples in parallel using AVX (256-bit).
/// This is how C++ backends achieve 10x performance gains.
pub unsafe fn apply_gain_v8(data: *mut f32, count: usize, gain: f32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let v_gain = _mm256_set1_ps(gain);
        let mut i = 0;
        
        // Process in chunks of 8
        while i + 8 <= count {
            let ptr = data.add(i);
            let v_data = _mm256_loadu_ps(ptr);
            let v_res = _mm256_mul_ps(v_data, v_gain);
            _mm256_storeu_ps(ptr, v_res);
            i += 8;
        }
        
        // Handle remainder (Manual Unrolling)
        while i < count {
            *data.add(i) *= gain;
            i += 1;
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    unsafe {
        for i in 0..count {
            *data.add(i) *= gain;
        }
    }
}

/// Mix two buffers with 4x manual unrolling.
/// Reducing the number of "jump" instructions keeps the CPU pipeline full.
pub unsafe fn mix_buffers_unrolled(dst: *mut f32, src: *const f32, count: usize, volume: f32) {
    let mut i = 0;
    unsafe {
        while i + 4 <= count {
            *dst.add(i) += *src.add(i) * volume;
            *dst.add(i + 1) += *src.add(i + 1) * volume;
            *dst.add(i + 2) += *src.add(i + 2) * volume;
            *dst.add(i + 3) += *src.add(i + 3) * volume;
            i += 4;
        }
        while i < count {
            *dst.add(i) += *src.add(i) * volume;
            i += 1;
        }
    }
}

/// Non-Temporal Store (MOVNTI)
/// Writes data directly to RAM, bypassing the CPU cache entirely.
/// Prevents "Cache Pollution" when writing large blocks of data that won't be read again immediately.
pub unsafe fn stream_store_f32(ptr: *mut f32, val: f32) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        // Treat as i32 for the intrinsic
        let bits = core::mem::transmute::<f32, i32>(val);
        _mm_stream_si32(ptr as *mut i32, bits);
    }
    #[cfg(not(target_arch = "x86_64"))]
    unsafe {
        ptr.write_volatile(val);
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
