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

reverts = {
    'crates/02-resonance/effects/src/gate.rs': [('_sample_rate: f32,', 'sample_rate: f32,'), ('sample_rate,', 'sample_rate,')], # Already correct in reverts
    'crates/02-resonance/smoothie-physics/src/components/diode.rs': [('_model: DiodeModel,', 'model: DiodeModel,')],
    'crates/02-resonance/smoothie-physics/src/components/transformer.rs': [('_coercivity: f32,', 'coercivity: f32,'), ('_remnance: f32,', 'remnance: f32,'), ('_secondary_flux: f32,', 'secondary_flux: f32,')],
    'crates/03-cognition/smoothie-graph/src/connection.rs': [('_adjacency: Vec<Vec<usize>>,', 'adjacency: Vec<Vec<usize>>,')],
    'crates/02-resonance/smoothie-physics/src/components/transistor.rs': [('_transistor_type: TransistorType,', 'transistor_type: TransistorType,')],
    'crates/02-resonance/smoothie-physics/src/components/tube.rs': [('_tube_type: TubeType,', 'tube_type: TubeType,')],
    'crates/03-cognition/param-automation/src/lib.rs': [('_name: &\'static str,', 'name: &\'static str,'), ('_voice_id: u8,', 'voice_id: u8,'), ('_history: Vec<(u32, f32)>,', 'history: Vec<(u32, f32)>,')],
    'crates/03-cognition/seraphic-prime/src/memory/working.rs': [('_role: &\'static str,', 'role: &\'static str,'), ('_content: String<512>,', 'content: String<512>,')],
    'crates/02-resonance/smoothie-physics/src/wdf/tubes.rs': [('_ex: f32,', 'ex: f32,')],
    'crates/02-resonance/effects/src/vintage.rs': [('_output_stage: f32,', 'output_stage: f32,')],
    'crates/03-cognition/smoothie-ai/src/batching.rs': [('_temp_output: Vec<f32>,', 'temp_output: Vec<f32>,')],
    'crates/03-cognition/smoothie-ai/src/optimizer.rs': [('_grad_avg: Vec<f32>,', 'grad_avg: Vec<f32>,')],
    'crates/03-cognition/smoothie-ai/src/vae.rs': [('_input_dim: usize,', 'input_dim: usize,'), ('_latent_dim: usize,', 'latent_dim: usize,'), ('_output_dim: usize,', 'output_dim: usize,'), ('_condition_dim: usize,', 'condition_dim: usize,')],
}

for path, patterns in reverts.items():
    fix_file(path, patterns)

# Now apply #[allow(dead_code)] to the struct fields instead of renaming them
dead_code_fixes = [
    ('crates/02-resonance/effects/src/gate.rs', 'sample_rate: f32,'),
    ('crates/02-resonance/effects/src/imager.rs', 'sample_rate: f32,'),
    ('crates/02-resonance/effects/src/modulation.rs', 'depth: f32,'),
    ('crates/02-resonance/effects/src/pitch_shift.rs', 'sample_rate: f32,'),
    ('crates/02-resonance/effects/src/stereo.rs', 'sample_rate: f32,'),
    ('crates/02-resonance/effects/src/vintage.rs', 'output_stage: f32,'),
    ('crates/03-cognition/param-automation/src/lib.rs', 'name: &\'static str,'),
    ('crates/03-cognition/param-automation/src/lib.rs', 'voice_id: u8,'),
    ('crates/03-cognition/param-automation/src/lib.rs', 'history: Vec<(u32, f32)>,'),
    ('crates/02-resonance/smoothie-physics/src/components/diode.rs', 'model: DiodeModel,'),
    ('crates/02-resonance/smoothie-physics/src/components/transformer.rs', 'coercivity: f32,'),
    ('crates/02-resonance/smoothie-physics/src/components/transformer.rs', 'remnance: f32,'),
    ('crates/02-resonance/smoothie-physics/src/components/transformer.rs', 'secondary_flux: f32,'),
    ('crates/02-resonance/smoothie-physics/src/components/transistor.rs', 'transistor_type: TransistorType,'),
    ('crates/02-resonance/smoothie-physics/src/components/tube.rs', 'tube_type: TubeType,'),
    ('crates/02-resonance/smoothie-physics/src/wdf/tubes.rs', 'ex: f32,'),
    ('crates/03-cognition/smoothie-graph/src/connection.rs', 'adjacency: Vec<Vec<usize>>,'),
    ('crates/03-cognition/seraphic-prime/src/memory/working.rs', 'role: &\'static str,'),
    ('crates/03-cognition/seraphic-prime/src/memory/working.rs', 'content: String<512>,'),
    ('crates/03-cognition/smoothie-ai/src/batching.rs', 'temp_output: Vec<f32>,'),
    ('crates/03-cognition/smoothie-ai/src/optimizer.rs', 'grad_avg: Vec<f32>,'),
    ('crates/03-cognition/smoothie-ai/src/vae.rs', 'input_dim: usize,'),
    ('crates/03-cognition/smoothie-ai/src/vae.rs', 'latent_dim: usize,'),
    ('crates/03-cognition/smoothie-ai/src/vae.rs', 'output_dim: usize,'),
    ('crates/03-cognition/smoothie-ai/src/vae.rs', 'condition_dim: usize,'),
]

for path, field in dead_code_fixes:
    fix_file(path, [(field, '#[allow(dead_code)]\n    ' + field)])
