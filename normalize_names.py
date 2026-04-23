import os
import re

def normalize_names(path):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    
    # List of broken names to revert
    mapping = {
        'tablesize': 'table_size',
        'zonesize': 'zone_size',
        'batchsize': 'batch_size',
        'inputsize': 'input_size',
        'hiddensize': 'hidden_size',
        'outputsize': 'output_size',
        'temp_buffersize': 'temp_buffer_size',
        'projectname': 'project_name',
        'pluginname': 'plugin_name',
        'dmodel': 'd_model',
        'kernelsize': 'kernel_size',
        'currentsize': 'current_size',
        'fftsize': 'fft_size',
        'hopsize': 'hop_size',
        'windowsize': 'window_size',
        'maxsize': 'max_size',
        'blocksize': 'block_size',
        'grainsize': 'grain_size',
        'roomsize': 'room_size',
        'weightssize': 'weights_size',
        'biasessize': 'biases_size',
    }
    
    new_content = content
    for broken, original in mapping.items():
        # Match only full word to avoid partial replacement (though these names are fairly unique)
        new_content = re.sub(r'\b' + broken + r'\b', original, new_content)
    
    # Also fix the incorrect #[allow(dead_code)] placements I might have introduced
    new_content = new_content.replace('pub #[allow(dead_code)]\n    ', '#[allow(dead_code)]\n    pub ')
    new_content = new_content.replace('_#[allow(dead_code)]\n    ', '#[allow(dead_code)]\n    ')
    
    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Normalized {path}")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file.endswith('.rs'):
            normalize_names(os.path.join(root, file))

for root, dirs, files in os.walk('examples'):
    for file in files:
        if file.endswith('.rs'):
            normalize_names(os.path.join(root, file))
