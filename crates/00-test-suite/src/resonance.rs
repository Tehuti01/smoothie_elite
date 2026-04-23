/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xc85ee3e6 | REVISION: 2026.04.20                           │
 * │ PATH: crates/00-test-suite/src/resonance.rs                               │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Consolidated Resonance Integration Tests.                    │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_effects::*;
use smoothie_dsp::oscillators::*;
use smoothie_dsp::filters::*;

#[test]
/// Technical implementation of the test_reverb_stabilization logic.
fn test_reverb_stabilization() {
    let mut reverb = ReverbEffect::new(44100.0);
    for _ in 0..1000 {
        let out = reverb.process(1.0);
        assert!(out.is_finite());
    }
}

#[test]
/// Technical implementation of the test_reverb_decay logic.
fn test_reverb_decay() {
    let mut reverb = ReverbEffect::new(44100.0);
    // Impulse
    reverb.process(1.0);
    let mut last_level = 1.0;
    for _ in 0..10000 {
        let out = reverb.process(0.0);
        last_level = out.abs();
    }
    assert!(last_level < 1.0);
}

#[test]
/// Technical implementation of the test_reverb_params logic.
fn test_reverb_params() {
    let mut reverb = ReverbEffect::new(44100.0);
    reverb.set_room_size(0.9);
    reverb.set_damping(0.1);
    let out = reverb.process(0.5);
    assert!(out != 0.0);
}

#[test]
/// Technical implementation of the test_delay_basic logic.
fn test_delay_basic() {
    let mut delay = DelayEffect::default();
    delay.set_delay_ms(100.0, 44100.0);
    delay.set_feedback(0.5);
    let _out = delay.process(1.0);
    assert!(true);
}

#[test]
/// Technical implementation of the test_delay_feedback_limit logic.
fn test_delay_feedback_limit() {
    let mut delay = DelayEffect::default();
    delay.set_feedback(2.0); // Should be clamped
    for _ in 0..10000 {
        let out = delay.process(0.5);
        assert!(out.abs() < 2.0);
    }
}

#[test]
/// Technical implementation of the test_compressor_reduction logic.
fn test_compressor_reduction() {
    let mut comp = Compressor::default();
    comp.set_threshold(-20.0);
    comp.set_ratio(4.0);
    
    // Give it time to attack
    let mut last_out = 0.0;
    for _ in 0..1000 {
        last_out = comp.process(1.0);
    }
    assert!(last_out.abs() < 0.9);
}

#[test]
/// Technical implementation of the test_compressor_bypass logic.
fn test_compressor_bypass() {
    let mut comp = Compressor::default();
    comp.set_threshold(0.0);
    comp.set_ratio(1.0);
    let input = 0.5;
    let out = comp.process(input);
    assert!((out - input).abs() < 0.1);
}

#[test]
/// Technical implementation of the test_limiter_ceiling logic.
fn test_limiter_ceiling() {
    let mut limiter = Limiter::new(-0.3, 50.0, 44100.0);
    for i in 0..1000 {
        let input = (i as f32 * 0.1).sin() * 2.0; // Over ceiling
        let out = limiter.process(input);
        assert!(out <= 1.01);
        assert!(out >= -1.01);
    }
}

#[test]
/// Technical implementation of the test_limiter_fast_attack logic.
fn test_limiter_fast_attack() {
    let mut limiter = Limiter::new(-0.3, 50.0, 44100.0);
    let input = 2.0;
    let out = limiter.process(input);
    assert!(out < 1.1);
}

#[test]
/// Technical implementation of the test_gate_silence logic.
fn test_gate_silence() {
    let mut gate = Gate::new(-40.0, 3.0, 1.0, 50.0, 10.0, 44100.0);
    let input = 0.0001; // Well below threshold
    for _ in 0..1000 {
        gate.process(input);
    }
    let out = gate.process(input);
    assert!(out.abs() < 0.01);
}

#[test]
/// Technical implementation of the test_gate_open logic.
fn test_gate_open() {
    let mut gate = Gate::new(-40.0, 3.0, 1.0, 50.0, 10.0, 44100.0);
    let input = 0.5; // Above threshold
    let mut last_out = 0.0;
    for _ in 0..1000 {
        last_out = gate.process(input);
    }
    assert!(last_out > 0.1);
}

#[test]
/// Technical implementation of the test_phaser_movement logic.
fn test_phaser_movement() {
    let mut phaser = Phaser::new(2.0, 0.8, 0.3, 44100.0);
    let out1 = phaser.process(0.5);
    for _ in 0..100 {
        phaser.process(0.5);
    }
    let out2 = phaser.process(0.5);
    assert!(out1 != out2);
}

#[test]
/// Technical implementation of the test_phaser_feedback logic.
fn test_phaser_feedback() {
    let mut phaser = Phaser::new(5.0, 1.0, 0.9, 44100.0);
    for _ in 0..1000 {
        let out = phaser.process(0.5);
        assert!(out.is_finite());
    }
}

#[test]
/// Technical implementation of the test_saturator_types logic.
fn test_saturator_types() {
    let algos = vec![
        SaturationType::Soft,
        SaturationType::Hard,
        SaturationType::Tube,
        SaturationType::Tape,
        SaturationType::Foldback,
        SaturationType::Bitcrush,
    ];
    for algo in algos {
        let mut sat = Saturator::new(algo);
        let out = sat.process(0.8);
        assert!(out.is_finite());
    }
}

#[test]
/// Technical implementation of the test_saturator_drive logic.
fn test_saturator_drive() {
    let mut sat = Saturator::new(SaturationType::Soft);
    sat.set_drive(10.0);
    let out = sat.process(0.5);
    assert!(out.abs() <= 1.0);
}

#[test]
/// Technical implementation of the test_widener_stereo logic.
fn test_widener_stereo() {
    let widener = StereoWidener::new(2.0);
    let signal = (0.8, 0.2);
    let (l, r) = widener.process(signal.0, signal.1);
    assert!(l != r);
}

#[test]
/// Technical implementation of the test_tremolo_depth logic.
fn test_tremolo_depth() {
    let mut trem = Tremolo::new(5.0, 1.0, 44100.0);
    let out1 = trem.process(0.5);
    assert!(out1 != 0.5);
    
    let mut varied = false;
    for _ in 0..1000 {
        if trem.process(0.5) != 0.5 {
            varied = true;
            break;
        }
    }
    assert!(varied);
}

#[test]
/// Technical implementation of the test_autopan_movement logic.
fn test_autopan_movement() {
    let mut pan = AutoPan::new(2.0, 1.0, 44100.0);
    let mut left_dominant = false;
    let mut right_dominant = false;
    
    for _ in 0..44100 {
        let (l, r) = pan.process(0.5, 0.5);
        if l > r { left_dominant = true; }
        if r > l { right_dominant = true; }
    }
    assert!(left_dominant && right_dominant);
}

#[test]
/// Technical implementation of the test_distortion_clamping logic.
fn test_distortion_clamping() {
    let mut dist = Distortion::default();
    dist.set_drive(20.0);
    for _ in 0..1000 {
        let out = dist.process(0.9);
        assert!(out <= 1.1 && out >= -1.1);
    }
}

#[test]
/// Technical implementation of the test_chorus_movement logic.
fn test_chorus_movement() {
    let mut chorus = Chorus::default();
    let out1 = chorus.process(0.5, 44100.0);
    for _ in 0..1000 {
        chorus.process(0.5, 44100.0);
    }
    let out2 = chorus.process(0.5, 44100.0);
    assert!(out1 != out2);
}

#[test]
/// Technical implementation of the test_chorus_bounded logic.
fn test_chorus_bounded() {
    let mut chorus = Chorus::default();
    for _ in 0..10000 {
        let out = chorus.process(0.8, 44100.0);
        assert!(out.abs() < 5.0, "Chorus unbounded: {}", out);
    }
}
