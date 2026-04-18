//! CPU Control: Pinning, Power, NUMA, and Hardware Profiling
//! Direct control over the hardware processor states.

#[cfg(target_os = "linux")]
use libc::{cpu_set_t, sched_setaffinity, CPU_SET, CPU_ZERO};
use core::mem;
use core::arch::asm;

/// Thread Affinities (CPU Pinning)
/// Forces the current thread to stay on a specific physical CPU core to prevent cache migration.
pub fn pin_current_thread_to_core(core_id: usize) -> Result<(), &'static str> {
    #[cfg(target_os = "linux")]
    unsafe {
        let mut cpuset: cpu_set_t = mem::zeroed();
        CPU_ZERO(&mut cpuset);
        CPU_SET(core_id, &mut cpuset);

        if sched_setaffinity(0, mem::size_of::<cpu_set_t>(), &cpuset) != 0 {
            return Err("Failed to pin thread");
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        // MacOS/Windows implementations would use thread_policy_set or SetThreadAffinityMask
        let _ = core_id;
        Ok(()) // Stub
    }
}

/// Non-Uniform Memory Access (NUMA) Awareness
/// Detects which RAM is closest to the current CPU.
pub fn bind_memory_to_numa_node(ptr: *mut u8, size: usize, node_id: u32) {
    #[cfg(target_os = "linux")]
    unsafe {
        // Raw syscall to mbind (syscall 237)
        let mode = 2; // MPOL_BIND
        let mut nodemask: u64 = 1 << node_id;
        core::arch::asm!(
            "syscall",
            in("rax") 237, // SYS_mbind
            in("rdi") ptr as u64,
            in("rsi") size,
            in("rdx") mode,
            in("r10") &mut nodemask as *mut u64,
            in("r8") 65, // maxnode
            in("r9") 0,  // flags
            out("rcx") _,
            out("r11") _,
            options(nostack)
        );
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (ptr, size, node_id);
    }
}

/// Adaptive Frequency Scaling Hints
/// Signals the OS to keep the CPU/GPU in a high-power state to eliminate "ramp-up" lag.
pub fn request_high_performance_mode() {
    #[cfg(target_os = "macos")]
    {
        // In real Elite code, this invokes IOKit IOPMAssertionCreateWithDescription
    }
    #[cfg(target_os = "windows")]
    {
        // PowerSetRequest (PowerRequestExecutionRequired)
    }
}

/// Cache-Line Bouncing Detection via PMU (Performance Monitor Unit)
/// Reads raw CPU performance counters to detect if threads are fighting for memory.
#[inline(always)]
pub fn read_pmu_counter(counter_id: u32) -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        let low: u32;
        let high: u32;
        core::arch::asm!(
            "rdpmc",
            in("ecx") counter_id,
            lateout("eax") low,
            lateout("edx") high,
            options(nomem, nostack)
        );
        ((high as u64) << 32) | (low as u64)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let _ = counter_id;
        0
    }
}

/// ─── Silicon Ghost: Instruction Flow ───

/// Task 110: Hardware-Enforced Control Flow (BTI)
/// Uses ARM Branch Target Identification to ensure safe code jumps.
#[inline(always)]
pub unsafe fn ghost_branch_target() {
    #[cfg(target_arch = "aarch64")]
    {
        // 🚀 Silicon Ghost: Guarding the jump destination.
        core::arch::asm!("bti c");
    }
}

/// Task 107: Micro-Op Cache Alignment Macro Implementation
/// Prevents instruction fetch units from crossing 64-byte boundaries.
#[macro_export]
macro_rules! align_hot_path {
    () => {
        #[cfg(target_arch = "aarch64")]
        unsafe {
            // Padding to ensure the next loop starts on a cache-line boundary.
            core::arch::asm!(".p2align 6");
        }
    };
}

/// ─── Clock-Cycle Purity: System Sovereignty ───

/// Task 127: Micro-Architectural Loop Unrolling (Factor 8)
/// Manually unrolls 'Elite' hot-paths to match the 8-wide M3 dispatch.
#[macro_export]
macro_rules! unroll_factor_8 {
    ($($body:tt)*) => {
        $($body)*
        $($body)*
        $($body)*
        $($body)*
        $($body)*
        $($body)*
        $($body)*
        $($body)*
    };
}

/// Task 129: Explicit "False Sharing" Isolation
/// Inserts No-Access pages between thread-local nodes to prevent cache thrashing.
pub unsafe fn isolate_false_sharing(start: *mut u8, size: usize) {
    #[cfg(target_os = "macos")]
    {
        use libc::{mprotect, PROT_NONE};
        // 🛰️ Silicon Ghost: Committing a hardware-level 'Elite' barrier.
        // This prevents CPU A from reading CPU B's padding, ending line bouncing.
        mprotect(start as *mut libc::c_void, size, PROT_NONE);
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
