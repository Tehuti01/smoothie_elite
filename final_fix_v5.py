import os
import re

def final_fix_v5(path):
    if not os.path.exists(path): return
    with open(path, 'r') as f:
        content = f.read()
    
    # Mapping of broken names to original names
    mapping = {
        'partitionsize': 'partition_size',
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
        'reverbsize': 'reverb_size',
        'weightssize': 'weights_size',
        'biasessize': 'biases_size',
        'fontsize': 'font_size',
        'localmodel': 'local_model',
        'buffersize': 'buffer_size',
        'commsize': 'comm_size',
        'formsize': 'form_size',
        'ssndsize': 'ssnd_size',
        'filesize': 'file_size',
        'datasize': 'data_size',
        'bufsize': 'buf_size',
    }
    
    new_content = content
    for broken, original in mapping.items():
        new_content = re.sub(r'\b' + broken + r'\b', original, new_content)

    # Manual field restoration
    if 'struct Chorus' in new_content and 'depth: f32,' not in new_content:
         new_content = new_content.replace('lfo_depth: f32,', 'lfo_depth: f32,\n    depth: f32,')
    if 'struct Phaser' in new_content and 'lfo_depth: f32,' not in new_content:
         new_content = new_content.replace('lfo_rate: f32,', 'lfo_rate: f32,\n    lfo_depth: f32,')
    if 'struct Tremolo' in new_content and 'lfo_depth: f32,' not in new_content:
         new_content = new_content.replace('lfo_rate: f32,', 'lfo_rate: f32,\n    lfo_depth: f32,')

    # Fix PitchShift, GranularPitchShift, PhaseVocoderPitchShift
    if 'struct PitchShift' in new_content and 'overlap: usize,' not in new_content:
         new_content = new_content.replace('pitch_ratio: f32,', 'pitch_ratio: f32,\n    overlap: usize,')
    if 'struct GranularPitchShift' in new_content and 'grains: Vec<Grain>,' not in new_content:
         new_content = new_content.replace('buffer: Vec<Sample>,', 'buffer: Vec<Sample>,\n    grains: Vec<Grain>,')
    if 'struct PhaseVocoderPitchShift' in new_content and 'fft_size: usize,' not in new_content:
         new_content = new_content.replace('pitch_ratio: f32,', 'pitch_ratio: f32,\n    fft_size: usize,')
    if 'struct PitchShifter' in new_content and 'overlap: usize,' not in new_content:
         new_content = new_content.replace('pitch: f32,', 'pitch: f32,\n    overlap: usize,')

    # Fix PhaseVocoderPitchShift specific _fftsize vs fft_size
    if 'struct PhaseVocoderPitchShift' in new_content:
        new_content = new_content.replace('_fftsize', 'fft_size')

    if 'struct AutoPan' in new_content and 'lfo_depth: f32,' not in new_content:
         new_content = new_content.replace('lfo_rate: f32,', 'lfo_rate: f32,\n    lfo_depth: f32,')

    if new_content != content:
        with open(path, 'w') as f:
            f.write(new_content)
        print(f"Final fix v5 {path}")

for root, dirs, files in os.walk('crates'):
    for file in files:
        if file.endswith('.rs'):
            final_fix_v5(os.path.join(root, file))
