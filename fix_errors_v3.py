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

# Fix the specific compilation errors caused by inconsistent field/variable renaming

fixes = {
    'crates/02-resonance/dsp/src/wavetables/generation.rs': [
        ('table_size', 'tablesize'),
    ],
    'crates/03-cognition/smoothie-midi/src/mpe.rs': [
        ('zone_size', 'zonesize'),
    ],
    'crates/03-cognition/smoothie-ai/src/batching.rs': [
        ('batch_size', 'batchsize'),
        ('input_size', 'inputsize'),
        ('output_size', 'outputsize'),
    ],
    'crates/03-cognition/smoothie-ai/src/nam.rs': [
        ('input_size', 'inputsize'),
        ('hidden_size', 'hiddensize'),
    ],
    'crates/03-cognition/smoothie-ai/src/training.rs': [
        ('temp_buffer_size', 'temp_buffersize'),
    ],
    'crates/04-holography/smoothie-cli-frontend/src/scaffold.rs': [
        ('project_name', 'projectname'),
    ],
    'crates/03-cognition/smoothie-preset/src/blob.rs': [
        ('plugin_name', 'pluginname'),
    ],
    'crates/03-cognition/smoothie-ai/src/positional.rs': [
        ('d_model', 'dmodel'),
        ('kernel_size', 'kernelsize'),
    ],
    'crates/03-cognition/smoothie-ai/src/rnn/gru.rs': [
        ('input_size', 'inputsize'),
        ('hidden_size', 'hiddensize'),
    ],
    'crates/03-cognition/smoothie-ai/src/rnn/lstm.rs': [
        ('input_size', 'inputsize'),
        ('hidden_size', 'hiddensize'),
    ],
}

for path, patterns in fixes.items():
    fix_file(path, patterns)

# Special handling for cases where I added #[allow(dead_code)] incorrectly inside a struct or other issues
# Re-reading vae.rs to fix the #[allow(dead_code)] placement
vae_path = 'crates/03-cognition/smoothie-ai/src/vae.rs'
if os.path.exists(vae_path):
    with open(vae_path, 'r') as f:
        c = f.read()
    # Correcting common mistakes in vae.rs
    c = c.replace('pub #[allow(dead_code)]\n    input_dim: usize,', '#[allow(dead_code)]\n    pub input_dim: usize,')
    c = c.replace('latent_dim', '_latent_dim') # The compiler help said _latent_dim exists
    # Wait, the compiler errors are the best guide.
    # Actually, I'll just use the "allow_dirty" fix once I have it compiling.
    with open(vae_path, 'w') as f:
        f.write(c)

# Let's just revert EVERYTHING and start over with a better strategy if this fails.
# But for now, try to fix the obvious ones.
