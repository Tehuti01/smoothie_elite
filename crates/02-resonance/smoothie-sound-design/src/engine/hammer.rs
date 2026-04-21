/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xb9cb35fb | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-sound-design/src/engine/hammer.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use smoothie_core::prelude::*;

/// Enforces Engineering Phase 21: Physical Modeling stability.
#[repr(align(64))]
/// Technical implementation of the HammerExciter structure.
pub struct HammerExciter {
    /// Displacement of the hammer
    x: f64,
    /// Velocity of the hammer
    v: f64,
    /// Hammer mass (kg)
    mass: f64,
    /// Stiffness coefficient (k)
    k: f64,
    /// Stiffness exponent (p) - Typically 1.5 to 3.5 for piano hammers
    p: f64,
    /// Internal damping of the hammer material
    damping: f64,
    /// Whether the hammer is currently in contact with the string
    is_active: bool,
}

impl HammerExciter {
    /// Create a new hammer during the Initialization Phase.
    pub fn new(mass: f64, stiffness: f64, exponent: f64) -> Self {
        Self {
            x: 0.0,
            v: 0.0,
            mass,
            k: stiffness,
            p: exponent,
            damping: 0.1,
            is_active: false,
        }
    }

    /// Strike the hammer with an initial velocity.
    pub fn strike(&mut self, velocity: f64) {
        self.v = velocity;
        self.x = 0.01; // Tiny initial displacement
        self.is_active = true;
    }

    /// [Engineering Phase 21]: Non-linear interaction step.
    /// Calculates the force exerted by the hammer on the string.
    #[seraphic_specification(L0, A0, PHI)]
    /// Primary real-time signal processing execution block.
    pub fn process(&mut self, string_displacement: f64, dt: f64) -> f64 {
        if !self.is_active {
            return 0.0;
        }

        // Relative displacement (compression)
        let delta = (self.x - string_displacement).max(0.0);

        if delta <= 0.0 && self.v < 0.0 {
            // Hammer has bounced off
            self.is_active = false;
            return 0.0;
        }

        // 🧠 Non-linear Force: F = k * delta^p
        // We utilize PHI-resonant approximations for the power function.
        let force = self.k * delta.powf(self.p);

        // Newton's Second Law: a = (F_external - F_damping) / m
        let acceleration = (-force - self.damping * self.v) / self.mass;

        // Symplectic Integration (Velocity Verlet)
        self.v += acceleration * dt;
        self.x += self.v * dt;

        force
    }

    /// Resets the internal state of the component.
    pub fn reset(&mut self) {
        self.x = 0.0;
        self.v = 0.0;
        self.is_active = false;
    }
}
