/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb174016c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/adaptors.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::ports::WdfNode;

/// Technical implementation of the SeriesAdaptor structure.
pub struct SeriesAdaptor<'a, Port1: WdfNode, Port2: WdfNode> {
    pub port1: &'a mut Port1,
    pub port2: &'a mut Port2,
    resistance: f32,
    gamma1: f32,
    gamma2: f32,
}

impl<'a, Port1: WdfNode, Port2: WdfNode> SeriesAdaptor<'a, Port1, Port2> {
    /// Initializes a new instance of the associated type.
    pub fn new(port1: &'a mut Port1, port2: &'a mut Port2) -> Self {
        let r1 = port1.get_port_resistance();
        let r2 = port2.get_port_resistance();
        let r_port = r1 + r2;
        Self {
            port1,
            port2,
            resistance: r_port,
            gamma1: r1 / r_port,
            gamma2: r2 / r_port,
        }
    }
}

impl<'a, Port1: WdfNode, Port2: WdfNode> WdfNode for SeriesAdaptor<'a, Port1, Port2> {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        -(self.port1.wave_up() + self.port2.wave_up())
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        let w_up1 = self.port1.wave_up();
        let w_up2 = self.port2.wave_up();

        // Compute incident wave down to children
        let w_diff = wave + w_up1 + w_up2;
        let down1 = w_up1 - self.gamma1 * w_diff;
        let down2 = w_up2 - self.gamma2 * w_diff;

        self.port1.wave_down(down1);
        self.port2.wave_down(down2);
    }
}

/// Technical implementation of the ParallelAdaptor structure.
pub struct ParallelAdaptor<'a, Port1: WdfNode, Port2: WdfNode> {
    pub port1: &'a mut Port1,
    pub port2: &'a mut Port2,
    resistance: f32,
    gamma1: f32,
    gamma2: f32,
}

impl<'a, Port1: WdfNode, Port2: WdfNode> ParallelAdaptor<'a, Port1, Port2> {
    /// Initializes a new instance of the associated type.
    pub fn new(port1: &'a mut Port1, port2: &'a mut Port2) -> Self {
        let g1 = 1.0 / port1.get_port_resistance();
        let g2 = 1.0 / port2.get_port_resistance();
        let g_port = g1 + g2;
        Self {
            port1,
            port2,
            resistance: 1.0 / g_port,
            gamma1: g1 / g_port,
            gamma2: g2 / g_port,
        }
    }
}

impl<'a, Port1: WdfNode, Port2: WdfNode> WdfNode for ParallelAdaptor<'a, Port1, Port2> {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.resistance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        let w_up1 = self.port1.wave_up();
        let w_up2 = self.port2.wave_up();
        self.gamma1 * w_up1 + self.gamma2 * w_up2
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        let w_up1 = self.port1.wave_up();
        let w_up2 = self.port2.wave_up();

        // Sum incident wave down to children
        let w_up = self.gamma1 * w_up1 + self.gamma2 * w_up2;
        let w_diff = wave - w_up;
        let down1 = w_up1 + w_diff;
        let down2 = w_up2 + w_diff;

        self.port1.wave_down(down1);
        self.port2.wave_down(down2);
    }
}
