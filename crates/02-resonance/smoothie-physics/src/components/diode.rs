/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x6530b986 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/components/diode.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use crate::wdf::ports::WdfNode;
use smoothie_core::math::exp_approx;
// use smoothie_core::math::FloatExt;

/// Technical implementation of the DiodeComponent structure.
pub struct DiodeComponent {
    #[allow(dead_code)]
    model: DiodeModel,
    forward_voltage: f32,
    saturation_current: f32,
    thermal_voltage: f32,
    series_resistance: f32,
    breakdown_voltage: f32,
    state: f32,
}

#[derive(Clone, Copy)]
/// Technical implementation of the DiodeModel enumeration.
pub enum DiodeModel {
    Silicon,
    Germanium,
    Schottky,
    Zener,
    Led,
}

impl DiodeComponent {
    /// Initializes a new instance of the associated type.
    pub fn new(model: DiodeModel) -> Self {
        let (is, vt, vf, rs, vbr) = match model {
            DiodeModel::Silicon => (1e-12, 0.02585, 0.65, 0.25, -100.0),
            DiodeModel::Germanium => (1e-4, 0.026, 0.3, 2.0, -50.0),
            DiodeModel::Schottky => (5e-6, 0.025, 0.35, 0.5, -30.0),
            DiodeModel::Zener => (1e-8, 0.02585, 0.7, 1.0, -5.6),
            DiodeModel::Led => (8e-20, 0.026, 2.0, 10.0, 0.0),
        };
        Self {
            model,
            forward_voltage: vf,
            saturation_current: is,
            thermal_voltage: vt,
            series_resistance: rs,
            breakdown_voltage: vbr,
            state: 0.0,
        }
    }

    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, voltage: f32) -> f32 {
        let vt = self.thermal_voltage;
        let is = self.saturation_current;
        let rs = self.series_resistance;

        if voltage < self.breakdown_voltage {
            let v = voltage.max(self.breakdown_voltage);
            let i = is * (exp_approx(v / vt) - 1.0) + (voltage - self.breakdown_voltage) * 0.01;
            self.state = voltage - i * rs;
        } else if voltage > self.forward_voltage {
            let mut v = voltage;
            for _ in 0..3 {
                let i = is * (exp_approx(v / vt) - 1.0);
                let residual = v + rs * i - voltage;
                let jacobian = 1.0 + (rs * is / vt) * exp_approx(v / vt);
                v -= residual / jacobian;
            }
            self.state = v;
        } else {
            self.state = 0.0;
        }
        self.state
    }
}

impl WdfNode for DiodeComponent {
    /// Technical implementation of the get_port_resistance logic.
    fn get_port_resistance(&self) -> f32 {
        100.0
    }

    /// Technical implementation of the wave_up logic.
    fn wave_up(&mut self) -> f32 {
        self.state
    }

    /// Technical implementation of the wave_down logic.
    fn wave_down(&mut self, wave: f32) {
        self.state = wave;
    }
}
