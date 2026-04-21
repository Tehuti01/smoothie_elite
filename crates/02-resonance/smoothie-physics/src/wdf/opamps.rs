/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x06a640d8 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/opamps.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the OpAmpNode structure.
pub struct OpAmpNode<'a, Inverting: WdfNode, NonInverting: WdfNode, Output: WdfNode> {
    pub inverting: &'a mut Inverting,
    pub non_inverting: &'a mut NonInverting,
    pub output: &'a mut Output,

    gain: f32,
    slew_rate: f32, // V / sample
    last_v_out: f32,
}

impl<'a, Inverting: WdfNode, NonInverting: WdfNode, Output: WdfNode>
    OpAmpNode<'a, Inverting, NonInverting, Output>
{
    /// Initializes a new instance of the associated type.
    pub fn new(
        inverting: &'a mut Inverting,
        non_inverting: &'a mut NonInverting,
        output: &'a mut Output,
        gain: f32,
        slew_rate: f32,
    ) -> Self {
        Self {
            inverting,
            non_inverting,
            output,
            gain,
            slew_rate,
            last_v_out: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self) {
        let v_inv = self.inverting.wave_up();
        let v_non = self.non_inverting.wave_up();

        let diff = (v_non - v_inv) * self.gain;

        // Slew rate limiting algorithm
        let mut v_out = diff;
        let delta = v_out - self.last_v_out;
        if delta > self.slew_rate {
            v_out = self.last_v_out + self.slew_rate;
        } else if delta < -self.slew_rate {
            v_out = self.last_v_out - self.slew_rate;
        }

        // Extreme clipping limits (rail voltages)
        v_out = v_out.clamp(-15.0, 15.0);

        self.last_v_out = v_out;

        // Ideal opamp draws zero current on inputs
        self.inverting.wave_down(v_inv);
        self.non_inverting.wave_down(v_non);

        // Drive the output node
        self.output.wave_down(v_out);
    }
}
