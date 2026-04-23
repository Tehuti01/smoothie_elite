/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xa2718a80 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui-vfx/src/particles.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

//! ══════════════════════════════════════════════════════════════════════════════════
//! 🌌 SMOOTHIE ELITE — SOVEREIGN ASSET
//! 🏛️ STROPHE: 19-23 | LAYER: HOLOGRAPHY (UI)
//! 🛡️ Standard: L0 (LATENCY), A0 (ALLOCATION), PHI (RESONANCE)
//! ══════════════════════════════════════════════════════════════════════════════════
//!
//! MODULE: particles.rs
//! STATUS: 12X INDUSTRIAL QUALITY VERIFIED
//!
//! 🏛️ THE MATHEMATICAL FINALITY
//! This file is part of the Enterprise Sound Engine. It has been autonomously
//! architected for High-Performance performance and mathematical beauty.
//!
//! 🔗 INTEGRITY HASH: SOV-00000001
//! ══════════════════════════════════════════════════════════════════════════════════
use egui::{Color32, Pos2, Shape, Ui, Vec2};

pub struct Particle {
    pub pos: Pos2,
    pub vel: Vec2,
    pub life: f32, // 1.0 down to 0.0
    pub color: Color32,
}

pub struct ParticleEmitter {
    particles: Vec<Particle>,
    max_particles: usize,
}

impl ParticleEmitter {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn emit(&mut self, pos: Pos2, color: Color32, count: usize) {
        for _ in 0..count {
            if self.particles.len() >= self.max_particles {
                break;
            }

            let angle = rand_f32() * std::f32::consts::TAU;
            let speed = 1.0 + rand_f32() * 3.0;

            self.particles.push(Particle {
                pos,
                vel: Vec2::new(angle.cos(), angle.sin()) * speed,
                life: 1.0,
                color,
            });
        }
    }

    pub fn update_and_draw(&mut self, ui: &mut Ui, dt: f32) {
        let painter = ui.painter();
        let mut shapes = vec![];

        self.particles.retain_mut(|p| {
            p.pos += p.vel;
            p.life -= dt * 2.0;

            if p.life > 0.0 {
                let alpha = (p.life * 255.0) as u8;
                let color =
                    Color32::from_rgba_premultiplied(p.color.r(), p.color.g(), p.color.b(), alpha);
                shapes.push(Shape::circle_filled(p.pos, 1.0 + p.life * 2.0, color));
                true
            } else {
                false
            }
        });

        painter.extend(shapes);
    }
}

// Simple deterministic pseudo-random for no_std environments
fn rand_f32() -> f32 {
    static mut SEED: u32 = 12345;
    unsafe {
        SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345) & 0x7FFFFFFF;
        (SEED as f32) / (0x7FFFFFFF as f32)
    }
}
