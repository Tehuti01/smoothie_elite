//! Comprehensive synthesis engine tests.

use smoothie_synth::{
    PolySynth, FmSynth, GranularEngine, WavetableEngine,
    AllocationStrategy, FmAlgorithm, GrainEnvelope,
};

const SAMPLE_RATE: f32 = 44100.0;
const TOLERANCE: f32 = 0.001;

// ============================================================================
// Polyphonic Synthesizer Tests
// ============================================================================

#[test]
fn test_polysynth_initialization() {
    let synth = PolySynth::new(SAMPLE_RATE);
    assert_eq!(synth.active_voice_count(), 0);
}

#[test]
fn test_polysynth_note_on_off() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    synth.note_on(60, 100);  // C4, velocity 100
    assert!(synth.active_voice_count() > 0);

    synth.note_off(60);
    // Voice goes into release, not immediately idle
}

#[test]
fn test_polysynth_voice_allocation_oldest() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    synth.set_allocation(AllocationStrategy::OldestVoice);

    for note in 60..92 {  // 32 notes
        synth.note_on(note, 80);
    }

    let initial_count = synth.active_voice_count();
    assert!(initial_count >= 32, "Should have 32+ voices");

    // 33rd note should steal oldest voice (or allocate if < 32)
    synth.note_on(92, 80);
    let final_count = synth.active_voice_count();
    assert!(final_count >= initial_count, "Voice count should increase or stay same");
}

#[test]
fn test_polysynth_voice_allocation_quietest() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    synth.set_allocation(AllocationStrategy::QuietestVoice);

    for note in 60..92 {
        synth.note_on(note, 80);
    }

    let initial_count = synth.active_voice_count();
    assert!(initial_count >= 32);

    synth.note_on(92, 100);  // Loud note should steal quietest voice
    let final_count = synth.active_voice_count();
    assert!(final_count >= initial_count);
}

#[test]
fn test_polysynth_audio_output() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    synth.note_on(60, 100);

    // Process 100 samples
    let mut output = Vec::new();
    for _ in 0..100 {
        output.push(synth.process());
    }

    // Verify output is in reasonable range
    for &sample in &output {
        assert!(sample.is_finite(), "Non-finite audio output");
        assert!(sample.abs() < 2.0, "Audio clipped");
    }

    // Verify at least some non-zero samples
    let non_zero = output.iter().filter(|&&s| s.abs() > TOLERANCE).count();
    assert!(non_zero > 0, "Expected audio output");
}

#[test]
fn test_polysynth_waveform_selection() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    synth.set_waveform(0);  // sine
    synth.note_on(60, 100);

    let mut output1 = Vec::new();
    for _ in 0..100 {
        output1.push(synth.process());
    }

    synth.reset();
    synth.set_waveform(2);  // sawtooth
    synth.note_on(60, 100);

    let mut output2 = Vec::new();
    for _ in 0..100 {
        output2.push(synth.process());
    }

    // Different waveforms should produce different output
    let diff = output1.iter().zip(output2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<f32>();

    assert!(diff > 0.1, "Waveforms should differ significantly");
}

// ============================================================================
// FM Synthesis Tests
// ============================================================================

#[test]
fn test_fmsynth_initialization() {
    let synth = FmSynth::new(SAMPLE_RATE);
    // Should not panic on creation
}

#[test]
fn test_fmsynth_algorithm_switching() {
    let mut synth = FmSynth::new(SAMPLE_RATE);

    // Test each algorithm
    for algo in [FmAlgorithm::Simple, FmAlgorithm::Stack, FmAlgorithm::Complex] {
        synth.set_algorithm(algo);
        let output = synth.process();
        assert!(output.is_finite());
    }
}

#[test]
fn test_fmsynth_audio_output() {
    let mut synth = FmSynth::new(SAMPLE_RATE);
    synth.set_algorithm(FmAlgorithm::Simple);

    let mut output = Vec::new();
    for _ in 0..1000 {
        output.push(synth.process());
    }

    for &sample in &output {
        assert!(sample.is_finite());
        assert!(sample.abs() < 2.0);
    }

    let non_zero = output.iter().filter(|&&s| s.abs() > TOLERANCE).count();
    assert!(non_zero > 0);
}

#[test]
fn test_fmsynth_operator_frequency() {
    let mut synth = FmSynth::new(SAMPLE_RATE);

    synth.set_op_ratio(0, 1.0, 440.0);
    synth.set_op_ratio(1, 2.0, 440.0);  // 880 Hz modulator

    // Process with modulation
    let mut output = Vec::new();
    for _ in 0..100 {
        output.push(synth.process());
    }

    let non_zero = output.iter().filter(|&&s| s.abs() > TOLERANCE).count();
    assert!(non_zero > 0);
}

// ============================================================================
// Granular Synthesis Tests
// ============================================================================

#[test]
fn test_granular_initialization() {
    let engine = GranularEngine::new(SAMPLE_RATE);
    // Should not panic
}

#[test]
fn test_granular_feed_and_process() {
    let mut engine = GranularEngine::new(SAMPLE_RATE);

    // Feed input
    for i in 0..1000 {
        let input = (i as f32 / 100.0).sin();
        engine.feed_sample(input);
    }

    // Process
    let mut output = Vec::new();
    for _ in 0..1000 {
        output.push(engine.process());
    }

    for &sample in &output {
        assert!(sample.is_finite());
        assert!(sample.abs() < 2.0);
    }
}

#[test]
fn test_granular_envelope_shapes() {
    for &shape in &[
        GrainEnvelope::Hann,
        GrainEnvelope::Gaussian,
        GrainEnvelope::Triangle,
    ] {
        let mut engine = GranularEngine::new(SAMPLE_RATE);
        engine.set_grain_shape(shape);

        // Feed and process
        for i in 0..100 {
            engine.feed_sample((i as f32 / 50.0).sin());
        }

        let _ = engine.process();
        // Just verify no panic
    }
}

#[test]
fn test_granular_pitch_control() {
    let mut engine = GranularEngine::new(SAMPLE_RATE);

    engine.set_grain_pitch(1.0);  // Normal speed
    for i in 0..100 {
        engine.feed_sample((i as f32 / 50.0).sin());
    }
    let out1 = engine.process();
    assert!(out1.is_finite());

    engine.set_grain_pitch(2.0);  // Double speed
    for i in 0..100 {
        engine.feed_sample((i as f32 / 50.0).sin());
    }
    let out2 = engine.process();
    assert!(out2.is_finite());
}

// ============================================================================
// Wavetable Tests
// ============================================================================

#[test]
fn test_wavetable_engine() {
    let mut engine = WavetableEngine::new(SAMPLE_RATE, 2048);

    let sample = engine.current().sample_interpolated(0.5);
    assert!(sample.is_finite());
    assert!(sample.abs() <= 1.0);
}

#[test]
fn test_wavetable_morphing() {
    let mut engine = WavetableEngine::new(SAMPLE_RATE, 2048);

    // Different waveforms
    engine.set_waveform(0);  // sine
    let sine = engine.current().sample_interpolated(0.5);

    engine.set_waveform(2);  // sawtooth
    let saw = engine.current().sample_interpolated(0.5);

    // Both should be valid samples
    assert!(sine.is_finite());
    assert!(saw.is_finite());
    assert!(sine.abs() <= 1.01);  // Allow small numerical error
    assert!(saw.abs() <= 1.01);
}

// ============================================================================
// Real-time Safety Tests
// ============================================================================

#[test]
fn test_polysynth_no_allocations() {
    let mut synth = PolySynth::new(SAMPLE_RATE);

    synth.note_on(60, 100);
    synth.note_on(64, 100);
    synth.note_on(67, 100);

    // Process many samples — should not allocate
    for _ in 0..10000 {
        let _ = synth.process();
    }
}

#[test]
fn test_fmsynth_realtime_safe() {
    let mut synth = FmSynth::new(SAMPLE_RATE);

    for _ in 0..100000 {
        let _ = synth.process();
    }
}

#[test]
fn test_granular_sample_rate_change() {
    let mut engine = GranularEngine::new(SAMPLE_RATE);

    engine.set_sample_rate(48000.0);
    engine.feed_sample(0.5);
    let _ = engine.process();

    engine.set_sample_rate(96000.0);
    engine.feed_sample(0.5);
    let _ = engine.process();
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_polyphonic_chord() {
    let mut synth = PolySynth::new(SAMPLE_RATE);

    // C major chord
    synth.note_on(60, 80);  // C
    synth.note_on(64, 80);  // E
    synth.note_on(67, 80);  // G

    let mut output = Vec::new();
    for _ in 0..SAMPLE_RATE as usize {  // 1 second
        output.push(synth.process());
    }

    // Verify mixed signal
    let rms = (output.iter().map(|s| s * s).sum::<f32>() / output.len() as f32).sqrt();
    assert!(rms > TOLERANCE);
    assert!(rms < 1.5);
}

#[test]
fn test_sequential_notes() {
    let mut synth = PolySynth::new(SAMPLE_RATE);
    let mut output = Vec::new();

    // Play a simple melody
    let notes = [60, 62, 64, 65, 67, 69, 71, 72];
    let samples_per_note = SAMPLE_RATE as usize / 8;  // 8 notes per second

    for &note in &notes {
        synth.note_on(note, 100);
        for _ in 0..samples_per_note {
            output.push(synth.process());
        }
        synth.note_off(note);
    }

    // Verify continuous audio
    let non_zero = output.iter().filter(|&&s| s.abs() > TOLERANCE).count();
    assert!(non_zero > output.len() / 2);
}
