//! User-Space Task Scheduling (Fibers/Green Threads)
//! Implements a custom scheduler that swaps execution contexts without involving the OS kernel.

use core::arch::asm;

/// The "Smoothie Context" stores CPU registers for a single fiber.
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct SmoothieContext {
    pub rsp_or_sp: u64,
    pub rbp_or_fp: u64,
    pub rbx_or_x19: u64,
    pub r12_or_x20: u64,
    pub r13_or_x21: u64,
    pub r14_or_x22: u64,
    pub r15_or_x23: u64,
    pub rip_or_pc: u64,
}

/// Swaps from the current execution context to another.
/// This is the "Elite" way to handle task switching.
#[inline(always)]
pub unsafe fn swap_context(current: *mut SmoothieContext, next: *const SmoothieContext) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        asm!(
            "mov [rcx + 0x00], rsp",
            "mov [rcx + 0x08], rbp",
            "mov [rcx + 0x10], rbx",
            "mov [rcx + 0x18], r12",
            "mov [rcx + 0x20], r13",
            "mov [rcx + 0x28], r14",
            "mov [rcx + 0x30], r15",
            "lea rax, [rip + 1f]",
            "mov [rcx + 0x38], rax",
            
            "mov rsp, [rdx + 0x00]",
            "mov rbp, [rdx + 0x08]",
            "mov rbx, [rdx + 0x10]",
            "mov r12, [rdx + 0x18]",
            "mov r13, [rdx + 0x20]",
            "mov r14, [rdx + 0x28]",
            "mov r15, [rdx + 0x30]",
            "jmp qword ptr [rdx + 0x38]",
            "1:",
            in("rcx") current,
            in("rdx") next,
            out("rax") _,
            clobber_abi("C")
        );
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        asm!(
            "mov x8, sp",
            "str x8, [x0, #0x00]",
            "stp x29, x30, [x0, #0x08]",
            "stp x19, x20, [x0, #0x18]",
            "stp x21, x22, [x0, #0x28]",
            "stp x23, x24, [x0, #0x38]", // Added x24 for alignment if needed, but we only have x23 in struct
            
            "ldr x8, [x1, #0x00]",
            "mov sp, x8",
            "ldp x29, x30, [x1, #0x08]",
            "ldp x19, x20, [x1, #0x18]",
            "ldp x21, x22, [x1, #0x28]",
            "ldr x23, [x1, #0x38]",
            "br x30",
            in("x0") current,
            in("x1") next,
            clobber_abi("C")
        );
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
