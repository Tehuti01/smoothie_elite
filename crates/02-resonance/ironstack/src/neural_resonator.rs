/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x9a38f122 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/ironstack/src/neural_resonator.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Industrial-Grade Neuromorphic Resonator for IRONSTACK-100.   │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Real-time neural inference with SIMD acceleration.      │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_ai::dense::DenseLayer;
use smoothie_ai::rnn::gru::GRULayer;
use smoothie_core::primitives::Sample;

/// 🧠 IronStackNeuralResonator
/// Technical implementation of a non-linear neural drive stage.
/// uses a hybrid Dense-GRU architecture for temporal circuit modeling.
pub struct IronStackNeuralResonator {
    /// Input compression/expansion layer
    input_dense: DenseLayer,
    /// Temporal modeling layer (Stateful)
    recurrent_layer: GRULayer,
    /// Output synthesis layer
    output_dense: DenseLayer,

    /// Internal buffers for neural signals (zero-allocation)
    input_buf: [f32; 1],
    hidden_buf: [f32; 16],
    output_buf: [f32; 1],

    pub mix: f32,
    pub drive: f32,
}

impl IronStackNeuralResonator {
    /// Initializes a new instance of the associated type.
    pub fn new(hidden_size: usize) -> Self {
        assert!(hidden_size <= 16, "Sovereign performance cap: hidden_size exceeds 16");

        Self {
            input_dense: DenseLayer::new(1, hidden_size),
            recurrent_layer: GRULayer::new(hidden_size, hidden_size),
            output_dense: DenseLayer::new(hidden_size, 1),
            input_buf: [0.0; 1],
            hidden_buf: [0.0; 16],
            output_buf: [0.0; 1],
            mix: 0.5,
            drive: 1.0,
        }
    }

    /// Technical implementation of the process logic for single samples.
    pub fn process(&mut self, input: Sample) -> Sample {
        if self.mix <= 0.001 {
            return input;
        }

        // 1. Prepare input with Drive scaling
        self.input_buf[0] = input * self.drive;

        // 2. Input Projection (Dense)
        let mut proj_buf = [0.0; 16];
        self.input_dense.forward(&self.input_buf, &mut proj_buf[..self.input_dense.outputs]);

        // 3. Temporal Modeling (GRU Step)
        self.recurrent_layer.step(&proj_buf[..self.recurrent_layer.input_size], &mut self.hidden_buf[..self.recurrent_layer.hidden_size]);

        // 4. Output Projection (Dense)
        self.output_dense.forward(&self.hidden_buf[..self.output_dense.inputs], &mut self.output_buf);

        // 5. Final Dry/Wet Mix
        let neural_out = self.output_buf[0];
        input * (1.0 - self.mix) + neural_out * self.mix
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.recurrent_layer.reset_state();
        for x in self.hidden_buf.iter_mut() {
            *x = 0.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_neural_saturation logic.
    fn test_neural_saturation() {
        let mut resonator = IronStackNeuralResonator::new(8);
        resonator.mix = 1.0;
        resonator.drive = 2.0;

        // Process a few samples to warm up the recurrent state
        let mut last_out = 0.0;
        for _ in 0..10 {
            last_out = resonator.process(0.5);
        }

        // Verify that the output is non-zero and non-NaN
        assert!(last_out.abs() > 0.0);
        assert!(!last_out.is_nan());
    }
}
