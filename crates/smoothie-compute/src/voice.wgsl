// 'Elite' Parallel Voice Shader (WGSL)
// Orchestrates 1000+ oscillator voices in parallel on the GPU.

struct Voice {
    frequency: f32,
    amplitude: f32,
    phase: f32,
    pan: f32,
};

@group(0) @binding(0) var<storage, read_write> voices: array<Voice>;
@group(0) @binding(1) var<storage, read_write> output: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&voices)) {
        return;
    }

    var voice = voices[index];
    
    // PolyBLEP Sinusoid Generation (GPU Optimized)
    let PI = 3.14159265;
    let sample_rate = 44100.0;
    
    voice.phase += (voice.frequency / sample_rate);
    if (voice.phase >= 1.0) {
        voice.phase -= 1.0;
    }

    let out_sample = sin(2.0 * PI * voice.phase) * voice.amplitude;
    
    // Atomic or reduction-based accumulation would happen here or in a second pass.
    // For now, write to index (parallel output).
    output[index] = out_sample;

    // Write back updated phase
    voices[index] = voice;
}
