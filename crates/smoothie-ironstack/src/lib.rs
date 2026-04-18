pub mod audio;
pub mod dsp;
pub mod presets;

pub use audio::{Sample, AudioBuffer, SampleRate};
pub use dsp::amplifiers::Amplifier;
pub use dsp::cabinets::Cabinet;
pub use dsp::dynamics::{Compressor, Limiter, NoiseGate};
pub use dsp::eq::Equalizer;

pub mod plugin;


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
