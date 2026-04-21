/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xec31b717 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/wdf/topology.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::components::{Capacitor, Inductor, Resistor};
use crate::wdf::ports::WdfNode;

/// Technical implementation of the SeriesNode structure.
pub struct SeriesNode<'a, Left: WdfNode, Right: WdfNode> {
    pub left: &'a mut Left,
    pub right: &'a mut Right,
    r_eq: f32,
    gamma_l: f32,
    gamma_r: f32,
}

impl<'a, Left: WdfNode, Right: WdfNode> SeriesNode<'a, Left, Right> {
    /// Initializes a new instance of the associated type.
    pub fn new(left: &'a mut Left, right: &'a mut Right) -> Self {
        let r_l = left.get_port_resistance();
        let r_r = right.get_port_resistance();
        let r_eq = r_l + r_r;
        Self {
            left,
            right,
            r_eq,
            gamma_l: r_l / r_eq,
            gamma_r: r_r / r_eq,
        }
    }
}

impl<'a, Left: WdfNode, Right: WdfNode> WdfNode for SeriesNode<'a, Left, Right> {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.r_eq
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        -(self.left.wave_up() + self.right.wave_up())
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        let w_up_l = self.left.wave_up();
        let w_up_r = self.right.wave_up();
        let w_diff = wave + w_up_l + w_up_r;
        let down_l = w_up_l - self.gamma_l * w_diff;
        let down_r = w_up_r - self.gamma_r * w_diff;
        self.left.wave_down(down_l);
        self.right.wave_down(down_r);
    }
}

/// Technical implementation of the ParallelNode structure.
pub struct ParallelNode<'a, Top: WdfNode, Bottom: WdfNode> {
    pub top: &'a mut Top,
    pub bottom: &'a mut Bottom,
    r_eq: f32,
    g_top: f32,
    g_bottom: f32,
}

impl<'a, Top: WdfNode, Bottom: WdfNode> ParallelNode<'a, Top, Bottom> {
    /// Initializes a new instance of the associated type.
    pub fn new(top: &'a mut Top, bottom: &'a mut Bottom) -> Self {
        let g_t = 1.0 / top.get_port_resistance();
        let g_b = 1.0 / bottom.get_port_resistance();
        let g_eq = g_t + g_b;
        Self {
            top,
            bottom,
            r_eq: 1.0 / g_eq,
            g_top: g_t / g_eq,
            g_bottom: g_b / g_eq,
        }
    }
}

impl<'a, Top: WdfNode, Bottom: WdfNode> WdfNode for ParallelNode<'a, Top, Bottom> {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.r_eq
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.g_top * self.top.wave_up() + self.g_bottom * self.bottom.wave_up()
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        let w_up_t = self.top.wave_up();
        let w_up_b = self.bottom.wave_up();
        let w_up = self.g_top * w_up_t + self.g_bottom * w_up_b;
        let w_diff = wave - w_up;
        let down_t = w_up_t + w_diff;
        let down_b = w_up_b + w_diff;
        self.top.wave_down(down_t);
        self.bottom.wave_down(down_b);
    }
}

/// Technical implementation of the LadderSection structure.
pub struct LadderSection {
    pub resistor: Resistor,
    pub capacitor: Capacitor,
    pub impedance: f32,
}

impl LadderSection {
    /// Initializes a new instance of the associated type.
    pub fn new(r: f32, c: f32, fs: f32) -> Self {
        let r_l = r;
        let c_x = c;
        let rp_cap = 1.0 / (2.0 * c_x * fs);
        Self {
            resistor: Resistor::new(r_l),
            capacitor: Capacitor::new(c_x, fs),
            impedance: r_l + rp_cap,
        }
    }
}

impl WdfNode for LadderSection {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.impedance
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.resistor.wave_up() + self.capacitor.wave_up()
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        let up_r = self.resistor.wave_up();
        let up_c = self.capacitor.wave_up();
        let gamma_r = self.resistor.get_port_resistance() / self.impedance;
        let gamma_c = self.capacitor.get_port_resistance() / self.impedance;
        self.resistor.wave_down(wave * gamma_r);
        self.capacitor.wave_down(wave * gamma_c);
    }
}

/// Technical implementation of the TNetworkNode structure.
pub struct TNetworkNode<'a, Series1: WdfNode, Series2: WdfNode, Parallel: WdfNode> {
    pub series1: &'a mut Series1,
    pub series2: &'a mut Series2,
    pub parallel: &'a mut Parallel,
    r_eq: f32,
}

impl<'a, Series1: WdfNode, Series2: WdfNode, Parallel: WdfNode>
    TNetworkNode<'a, Series1, Series2, Parallel>
{
    /// Initializes a new instance of the associated type.
    pub fn new(s1: &'a mut Series1, p: &'a mut Parallel, s2: &'a mut Series2) -> Self {
        let r_series = s1.get_port_resistance() + s2.get_port_resistance();
        let g_p = 1.0 / p.get_port_resistance();
        let g_series = 1.0 / r_series;
        Self {
            series1: s1,
            series2: s2,
            parallel: p,
            r_eq: 1.0 / (g_series + g_p),
        }
    }
}

impl<'a, Series1: WdfNode, Series2: WdfNode, Parallel: WdfNode> WdfNode
    for TNetworkNode<'a, Series1, Series2, Parallel>
{
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        self.r_eq
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        let w_s1 = self.series1.wave_up();
        let w_p = self.parallel.wave_up();
        let w_s2 = self.series2.wave_up();
        let r_s1 = self.series1.get_port_resistance();
        let r_p = self.parallel.get_port_resistance();
        let r_s2 = self.series2.get_port_resistance();
        let g_eq = 1.0 / r_s1 + 1.0 / r_p + 1.0 / r_s2;
        let w = (w_s1 / r_s1 + w_p / r_p + w_s2 / r_s2) / g_eq;
        w
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.series1.wave_down(wave);
        self.parallel.wave_down(wave);
        self.series2.wave_down(wave);
    }
}
