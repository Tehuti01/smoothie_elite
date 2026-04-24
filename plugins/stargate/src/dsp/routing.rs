/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x524f5554 | REVISION: 2026.04.20                           │
 * │ PATH: plugins/stargate/src/dsp/routing.rs                                │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Signal Routing and Modulation Matrix.                       │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 */

use smoothie_modulation::matrix::ModMatrix;

pub struct StargateRouting {
    pub matrix: ModMatrix,
}

impl StargateRouting {
    pub fn new() -> Self {
        Self {
            matrix: ModMatrix::new(),
        }
    }
}

impl Default for StargateRouting {
    fn default() -> Self {
        Self::new()
    }
}
