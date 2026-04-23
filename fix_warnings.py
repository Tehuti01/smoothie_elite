import os
import re

def fix_file(path, patterns):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    
    new_content = content
    for old, new in patterns:
        new_content = new_content.replace(old, new)
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Fixed {path}")

# Pattern: (old_string, new_string)
fixes = {
    'crates/03-cognition/smoothie-ai/src/activations/exponential.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/activations/extended.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/activations/trigonometric.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/batching.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('temp_output: Vec<f32>,', '_temp_output: Vec<f32>,')],
    'crates/03-cognition/smoothie-ai/src/conformer.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/embedding.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/loss.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('fn fast_round', 'fn _fast_round')],
    'crates/03-cognition/smoothie-ai/src/normalization.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/positional.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/training.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-ai/src/vae.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('input_dim: usize,', '_input_dim: usize,'), ('latent_dim: usize,', '_latent_dim: usize,'), ('output_dim: usize,', '_output_dim: usize,'), ('condition_dim: usize,', '_condition_dim: usize,')],
    'crates/03-cognition/smoothie-ai/src/attention.rs': [('let mut qkv = 0.0f32;', 'let mut _qkv = 0.0f32;'), ('qkv += q[i]', '_qkv += q[i]')],
    'crates/03-cognition/smoothie-ai/src/optimizer.rs': [('fn fast_round', 'fn _fast_round'), ('grad_avg: Vec<f32>,', '_grad_avg: Vec<f32>,')],
    'crates/02-resonance/smoothie-physics/src/components/diode.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('model: DiodeModel,', '_model: DiodeModel,')],
    'crates/02-resonance/smoothie-physics/src/components/transformer.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('coercivity: f32,', '_coercivity: f32,'), ('remnance: f32,', '_remnance: f32,'), ('secondary_flux: f32,', '_secondary_flux: f32,')],
    'crates/02-resonance/smoothie-physics/src/components/tube.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;'), ('tube_type: TubeType,', '_tube_type: TubeType,')],
    'crates/02-resonance/smoothie-physics/src/components/transistor.rs': [('transistor_type: TransistorType,', '_transistor_type: TransistorType,')],
    'crates/02-resonance/smoothie-physics/src/wdf/tubes.rs': [('ex: f32,', '_ex: f32,')],
    'crates/02-resonance/effects/src/gate.rs': [('sample_rate: f32,', '_sample_rate: f32,')],
    'crates/02-resonance/effects/src/imager.rs': [('sample_rate: f32,', '_sample_rate: f32,')],
    'crates/02-resonance/effects/src/modulation.rs': [('depth: f32,', '_depth: f32,')],
    'crates/02-resonance/effects/src/pitch_shift.rs': [('sample_rate: f32,', '_sample_rate: f32,'), ('overlap: usize,', '_overlap: usize,'), ('grains: Vec<Grain>,', '_grains: Vec<Grain>,'), ('position: usize,', '_position: usize,'), ('length: usize,', '_length: usize,'), ('env: [f32; 2],', '_env: [f32; 2],'), ('fft_size: usize,', '_fft_size: usize,'), ('output_buffer: Vec<Sample>,', '_output_buffer: Vec<Sample>,')],
    'crates/02-resonance/effects/src/stereo.rs': [('sample_rate: f32,', '_sample_rate: f32,')],
    'crates/02-resonance/effects/src/time_stretch.rs': [('search_range: usize,', '_search_range: usize,'), ('output_buffer: Vec<Sample>,', '_output_buffer: Vec<Sample>,'), ('sample_rate: f32,', '_sample_rate: f32,'), ('search_width: usize,', '_search_width: usize,'), ('window_func: Vec<f32>,', '_window_func: Vec<f32>,')],
    'crates/02-resonance/effects/src/vintage.rs': [('fn sample_rate', 'fn _sample_rate'), ('output_stage: f32,', '_output_stage: f32,')],
    'crates/05-praxis/smoothie-vst3/src/component.rs': [('static AUDIO_PROCESSOR_VTABLE', '#[allow(dead_code)]\nstatic AUDIO_PROCESSOR_VTABLE'), ('unsafe extern "system" fn query_interface_impl', '#[allow(dead_code)]\nunsafe extern "system" fn query_interface_impl'), ('unsafe extern "system" fn release_impl', '#[allow(dead_code)]\nunsafe extern "system" fn release_impl'), ('unsafe extern "system" fn set_bus_arrangements_impl', '#[allow(dead_code)]\nunsafe extern "system" fn set_bus_arrangements_impl'), ('unsafe extern "system" fn get_bus_arrangement_impl', '#[allow(dead_code)]\nunsafe extern "system" fn get_bus_arrangement_impl'), ('unsafe extern "system" fn can_process_sample_size_impl', '#[allow(dead_code)]\nunsafe extern "system" fn can_process_sample_size_impl'), ('unsafe extern "system" fn get_latency_samples_impl', '#[allow(dead_code)]\nunsafe extern "system" fn get_latency_samples_impl'), ('unsafe extern "system" fn setup_processing_impl', '#[allow(dead_code)]\nunsafe extern "system" fn setup_processing_impl')],
    'crates/03-cognition/param-automation/src/lib.rs': [('name: &\'static str,', '_name: &\'static str,'), ('voice_id: u8,', '_voice_id: u8,'), ('history: Vec<(u32, f32)>,', '_history: Vec<(u32, f32)>,')],
    'crates/03-cognition/smoothie-preset/src/blob.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-preset/src/diff.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-preset/src/snapshot.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-graph/src/midi/mod.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-graph/src/nodes/mod.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/smoothie-graph/src/connection.rs': [('adjacency: Vec<Vec<usize>>,', '_adjacency: Vec<Vec<usize>>,')],
    'crates/03-cognition/seraphic-prime/src/memory/semantic.rs': [('use smoothie_core::math::FloatExt;', '// use smoothie_core::math::FloatExt;')],
    'crates/03-cognition/seraphic-prime/src/memory/working.rs': [('role: &\'static str,', '_role: &\'static str,'), ('content: String<512>,', '_content: String<512>,')],
    'crates/05-praxis/smoothie-standalone/src/audio.rs': [('host: cpal::Host,', '_host: cpal::Host,')],
    'crates/05-praxis/smoothie-standalone/src/window.rs': [('window: Arc<Window>,', '_window: Arc<Window>,'), ('config: wgpu::SurfaceConfiguration,', '_config: wgpu::SurfaceConfiguration,'), ('size: winit::dpi::PhysicalSize<u32>,', '_size: winit::dpi::PhysicalSize<u32>,')],
}

for path, patterns in fixes.items():
    fix_file(path, patterns)
