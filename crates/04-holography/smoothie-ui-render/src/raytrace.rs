/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb0a8c9bd | REVISION: 2026.04.20                           │
 * │ PATH: crates/04-holography/smoothie-ui-render/src/raytrace.rs            │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Low-latency UI raycasting and intersection engine.          │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Specialized for 2.5D holographic UI interaction.        │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_math::matrix::Vec3;

/// A Ray representing a screen pixel unprojected into the scene.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Initializes a new instance of the associated type.
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Technical implementation of the intersect_plane logic.
    pub fn intersect_plane(&self, plane_normal: Vec3, plane_origin: Vec3) -> Option<f32> {
        let denom = plane_normal.dot(&self.direction);
        if denom.abs() > 1e-6 {
            let diff = plane_origin.sub(&self.origin);
            let t = diff.dot(&plane_normal) / denom;
            if t >= 0.0 {
                return Some(t);
            }
        }
        None
    }
}
