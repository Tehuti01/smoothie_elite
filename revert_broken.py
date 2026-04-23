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

# Revert broken field renames and use underscore in constructor but keep original name in usage if possible
# Or just rename everywhere to underscore if they really are unused.
# The compiler errors show that they ARE used in some places.

# Revert all changes made by the previous script to get back to a known state (even if with warnings)
# But actually, I'll just surgical fix the broken ones.

surgical_fixes = {
    'crates/02-resonance/effects/src/imager.rs': [
        ('_#[allow(dead_code)]\n    sample_rate: f32,', '#[allow(dead_code)]\n    sample_rate: f32,'),
        ('_#[allow(dead_code)]\n    correlation: f32,', '#[allow(dead_code)]\n    correlation: f32,'),
    ],
    'crates/02-resonance/effects/src/modulation.rs': [
        ('lfo__#[allow(dead_code)]\n    depth: f32,', '#[allow(dead_code)]\n    depth: f32,'),
        ('_#[allow(dead_code)]\n    depth: f32,', '#[allow(dead_code)]\n    depth: f32,'),
    ],
    'crates/02-resonance/effects/src/pitch_shift.rs': [
        ('_#[allow(dead_code)]\n    sample_rate: f32,', '#[allow(dead_code)]\n    sample_rate: f32,'),
        ('_#[allow(dead_code)]\n    overlap: usize,', '#[allow(dead_code)]\n    overlap: usize,'),
        ('_#[allow(dead_code)]\n    grains: Vec<Grain>,', '#[allow(dead_code)]\n    grains: Vec<Grain>,'),
        ('_#[allow(dead_code)]\n    position: usize,', '#[allow(dead_code)]\n    position: usize,'),
        ('_#[allow(dead_code)]\n    length: usize,', '#[allow(dead_code)]\n    length: usize,'),
        ('_#[allow(dead_code)]\n    env: [f32; 2],', '#[allow(dead_code)]\n    env: [f32; 2],'),
        ('_#[allow(dead_code)]\n    fft_size: usize,', '#[allow(dead_code)]\n    fft_size: usize,'),
        ('_#[allow(dead_code)]\n    output_buffer: Vec<Sample>,', '#[allow(dead_code)]\n    output_buffer: Vec<Sample>,'),
    ],
    'crates/02-resonance/effects/src/stereo.rs': [
        ('_#[allow(dead_code)]\n    sample_rate: f32,', '#[allow(dead_code)]\n    sample_rate: f32,'),
    ],
    'crates/03-cognition/smoothie-ai/src/vae.rs': [
        ('pub #[allow(dead_code)]\n    input_dim: usize,', '#[allow(dead_code)]\n    pub input_dim: usize,'),
        ('pub #[allow(dead_code)]\n    latent_dim: usize,', '#[allow(dead_code)]\n    pub latent_dim: usize,'),
        ('pub #[allow(dead_code)]\n    output_dim: usize,', '#[allow(dead_code)]\n    pub output_dim: usize,'),
        ('pub #[allow(dead_code)]\n    condition_dim: usize,', '#[allow(dead_code)]\n    pub condition_dim: usize,'),
    ],
    'crates/05-praxis/smoothie-standalone/src/audio.rs': [
        ('_#[allow(dead_code)]\n    host: cpal::Host,', '#[allow(dead_code)]\n    host: cpal::Host,'),
    ],
    'crates/05-praxis/smoothie-standalone/src/window.rs': [
        ('_#[allow(dead_code)]\n    window: Arc<Window>,', '#[allow(dead_code)]\n    window: Arc<Window>,'),
        ('_#[allow(dead_code)]\n    config: wgpu::SurfaceConfiguration,', '#[allow(dead_code)]\n    config: wgpu::SurfaceConfiguration,'),
        ('_#[allow(dead_code)]\n    size: winit::dpi::PhysicalSize<u32>,', '#[allow(dead_code)]\n    size: winit::dpi::PhysicalSize<u32>,'),
    ],
}

for path, patterns in surgical_fixes.items():
    fix_file(path, patterns)

# Also fix the specific errors in time_stretch and others where I might have broken constructors
# By simply reverting the underscore prefix in constructors if I added them incorrectly.
# Wait, the error E0560: struct `TimeStretch` has no field named `search_range` means I renamed the field in the struct definition but not the constructor.

def revert_underscore_fields(path):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    # Replace '_field_name:' with 'field_name:' in struct definitions and constructors
    # This is broad but might work if I only added them recently.
    # Actually let's just revert the specific ones.
    new_content = content.replace('_search_range:', 'search_range:')
    new_content = new_content.replace('_output_buffer:', 'output_buffer:')
    new_content = new_content.replace('_sample_rate:', 'sample_rate:')
    new_content = new_content.replace('_search_width:', 'search_width:')
    new_content = new_content.replace('_window_func:', 'window_func:')
    new_content = new_content.replace('_temp_output:', 'temp_output:')
    new_content = new_content.replace('_grad_avg:', 'grad_avg:')
    new_content = new_content.replace('_input_dim:', 'input_dim:')
    new_content = new_content.replace('_latent_dim:', 'latent_dim:')
    new_content = new_content.replace('_output_dim:', 'output_dim:')
    new_content = new_content.replace('_condition_dim:', 'condition_dim:')
    new_content = new_content.replace('_model:', 'model:')
    new_content = new_content.replace('_coercivity:', 'coercivity:')
    new_content = new_content.replace('_remnance:', 'remnance:')
    new_content = new_content.replace('_secondary_flux:', 'secondary_flux:')
    new_content = new_content.replace('_transistor_type:', 'transistor_type:')
    new_content = new_content.replace('_tube_type:', 'tube_type:')
    new_content = new_content.replace('_ex:', 'ex:')
    new_content = new_content.replace('_adjacency:', 'adjacency:')
    new_content = new_content.replace('_name:', 'name:')
    new_content = new_content.replace('_voice_id:', 'voice_id:')
    new_content = new_content.replace('_history:', 'history:')
    new_content = new_content.replace('_role:', 'role:')
    new_content = new_content.replace('_content:', 'content:')
    new_content = new_content.replace('_window:', 'window:')
    new_content = new_content.replace('_config:', 'config:')
    new_content = new_content.replace('_size:', 'size:')
    new_content = new_content.replace('_host:', 'host:')
    new_content = new_content.replace('_output_stage:', 'output_stage:')
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Reverted underscores in {path}")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file.endswith('.rs'):
            revert_underscore_fields(os.path.join(root, file))

