/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xcca38b84 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui-core/src/lib.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;

///
/// for SDF-generated UI components.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Technical implementation of the Material enumeration.
pub enum Material {
    /// Anisotropic brushed metal (High-end hardware look)
    BrushedMetal {
        anisotropy: f32,
        roughness: f32,
        grain_scale: f32,
    },
    /// Multi-pass frosted glass with PHI-refraction
    FrostedGlass {
        blur_radius: f32,
        ior: f32, // Index of Refraction
    },
    /// Physically-based light emission (LED/Oscilloscope)
    Radiance {
        color: [f32; 3],
        intensity: f32,
        falloff: f32, // PHI-exponential decay
    },
    /// Absolute Abyssal Charcoal base
    Substrate,
}

///
/// Technical implementation of the UiNode structure.
pub struct UiNode {
    pub id: &'static str,
    pub material: Material,
    pub depth: f32,
    pub bounds: [f32; 4], // [x, y, w, h]
}

impl UiNode {
    /// Initializes a new instance of the associated type.
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            material: Material::Substrate,
            depth: 0.0,
            bounds: [0.0, 0.0, 100.0, 100.0],
        }
    }

    /// Technical implementation of the material logic.
    pub fn material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }

    /// Technical implementation of the depth logic.
    pub fn depth(mut self, d: f32) -> Self {
        self.depth = d;
        self
    }
}
