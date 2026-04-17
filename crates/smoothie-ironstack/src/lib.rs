pub mod audio;
pub mod dsp;
pub mod presets;

pub use audio::{Sample, AudioBuffer, SampleRate};
pub use dsp::amplifiers::Amplifier;
pub use dsp::cabinets::Cabinet;
pub use dsp::dynamics::{Compressor, Limiter, NoiseGate};
pub use dsp::eq::Equalizer;

pub mod plugin;
