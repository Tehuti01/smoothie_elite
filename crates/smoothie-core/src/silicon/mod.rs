//! High-Performance Silicon Primitives
//! Hand-coded low-level concepts for precise hardware orchestration.
//! Zero crates. Zero std dependencies (where possible). 


#![allow(missing_docs)]
#![allow(dead_code)]


pub mod aba_ptr;
pub mod arena;


pub mod atomic_fabric;
pub mod atomic_ring;


pub mod bus_lock;
pub mod bus_master;


pub mod bypass;
pub mod cache;


pub mod cache_master;
pub mod concurrency;


pub mod const_assert;
pub mod context_master;


pub mod core_tricks;
pub mod cpu_control;


pub mod crypto_master;
pub mod crypto_simd;


pub mod d2h;
pub mod demonstration;


pub mod ebpf;
pub mod ecs;


pub mod encoding_master;
pub mod endgame;


pub mod fiber;
pub mod geometric_dsp;


pub mod geometry;
pub mod gpu_compositor;


pub mod gpu_compute;
pub mod gpu_spectral;


pub mod hardware_crypto;
pub mod hardware_drivers;


pub mod hardware_malloc;
pub mod hardware_math;


pub mod hardware_sync;
pub mod huge_pages;


pub mod interpolation;
pub mod kernel_io;


pub mod lattice_master;
pub mod logic;


pub mod math;
pub mod mpsc_ring;


pub mod multicast;
pub mod nan_box;


pub mod neural;
pub mod neural_bus;


pub mod nic_master;
pub mod os_bypass;


pub mod peripheral_fabric;
pub mod pool;


pub mod protocol;
pub mod quantum_concurrency;


pub mod quantum_math;
pub mod quantum_mem;


pub mod quantum_system;
pub mod quantum_ui;


pub mod rdrand;
pub mod render_silicon;


pub mod saturation;
pub mod silicon_backend;


pub mod silicon_clock;
pub mod silicon_command;


pub mod silicon_frontend;
pub mod silicon_stream;


pub mod simd;
pub mod soa;


pub mod spinlock;
pub mod sso;


pub mod sync_master;
pub mod terminal;


pub mod thermal;
pub mod transistor;


pub mod tsx;
pub mod ui;


pub mod vector;
pub mod virtual_mem;


pub mod vtable;


pub use aba_ptr::AbaPointer;
pub use arena::Arena;


pub use atomic_fabric::*;
pub use atomic_ring::SpscQueue;


pub use bus_lock::*;
pub use bus_master::{PackedVertex, SiliconSeqLock};


pub use bypass::{KernelBypassRing, NvmeCmd};
pub use cache::{CacheAligned, CachePadding};


pub use cache_master::*;
pub use concurrency::{HazardTracker, WorkStealingDeque};


pub use const_assert::*;
pub use context_master::{HidPacket, AnimationNode};


pub use core_tricks::{NoInitBuffer, simd_find_delimiter, FastOpcode};
pub use cpu_control::{pin_current_thread_to_core, bind_memory_to_numa_node, request_high_performance_mode, read_pmu_counter};


pub use crypto_master::*;
pub use crypto_simd::{SimdJsonBuilder, hardware_aes_encrypt_block};


pub use d2h::{SiliconPacer, SiliconDispatcher, fast_hex_to_nibble};
pub use demonstration::DivineOrchestrator;


pub use ebpf::{BpfInsn, EbpfLoader};
pub use ecs::ComponentMask;


pub use encoding_master::*;
pub use endgame::EliteNucleus;


pub use fiber::{SmoothieContext, swap_context};
pub use geometric_dsp::*;


pub use geometry::{RATIO_0, RATIO_1, HARMONIC_PI, HARMONIC_2PI, PYTHAG_TRIAD, vector_norm, normalize_energy};
pub use gpu_compositor::{ExclusiveDisplayHandle, GpuConstantWindow, TextureStreamer};


pub use gpu_compute::{ComputeQueue, GpuAudioBuffer};
pub use gpu_spectral::*;


pub use hardware_crypto::{hardware_crc32, RawSocketSlice};
pub use hardware_drivers::*;


pub use hardware_malloc::{HardwareSlab, enforce_prediction};
pub use hardware_math::{simd_hex_encode, check_batch_pointers_valid, saturating_add_u8_batch};


pub use hardware_sync::{lfence_throttle, cpuid_serialize};
pub use huge_pages::HugePageMemory;


pub use interpolation::TouchPredictor;
pub use kernel_io::{hardware_cache_flush, ZeroSyscallRing};


pub use lattice_master::*;
pub use logic::*;


pub use math::{branchless_select_f32, InputDebouncer};
pub use mpsc_ring::MpscQueue;


pub use multicast::*;
pub use nan_box::TaggedPtr;


pub use neural::{FastTensor, NeuralResynthesizer};
pub use neural_bus::*;


pub use nic_master::*;
pub use os_bypass::{ThreadOrchestrator, query_cpu_topology};


pub use peripheral_fabric::*;
pub use pool::ObjectPool;


pub use protocol::*;
pub use quantum_concurrency::{AdaptiveLock, WaitFreeSnapshot};


pub use quantum_math::{branchless_binary_search, EliteBitset, prefetch_function_ptr};
pub use quantum_mem::{DaxState, critical_path_entry, parse_hot_data};


pub use quantum_system::{SiliconDMA, HardwareTrigger, HotPatch};
pub use quantum_ui::{LayoutConstraint, SdfGlyph, InputCoalescer};


pub use rdrand::hardware_rand_u32;
pub use render_silicon::*;


pub use saturation::*;
pub use silicon_backend::{PinnedPool, AtomicManifold, DescriptorRing, SiliconNodeManager, LatticeTable, SmoothieSpinner};


pub use silicon_clock::SiliconClock;
pub use silicon_command::{execute_parallel_manifold, stream_manifold_ps, migrate_to_sibling_core, verify_sovereignty};


pub use silicon_frontend::{SpeculativeCompositor, FixedCoord, HapticOrchestrator};
pub use silicon_stream::{WriteCombinedLog, JsonNavigator};


pub use simd::*;
pub use soa::*;


pub use spinlock::{Spinlock, TicketLock};
pub use sso::SsoString;


pub use sync_master::{SiliconPtr128, verify_alignment_safe};
pub use terminal::*;


pub use thermal::*;
pub use transistor::*;


pub use tsx::transactional_execute;
pub use ui::GeometryBatcher;


pub use vector::{apply_gain_v8, mix_buffers_unrolled, stream_store_f32};
pub use virtual_mem::{VirtualPageTable, LockFreeMap};


pub use vtable::{FlatObject, SmoothieVTable};


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
