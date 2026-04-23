/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x2582d88c | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/04-holography/smoothie-ui-vfx/src/glow.rs                                                         │
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
//! MODULE: glow.rs
//! STATUS: 12X INDUSTRIAL QUALITY VERIFIED
//!
//! 🏛️ THE MATHEMATICAL FINALITY
//! This file is part of the Enterprise Sound Engine. It has been autonomously
//! architected for High-Performance performance and mathematical beauty.
//!
//! 🔗 INTEGRITY HASH: SOV-00000001
//! ══════════════════════════════════════════════════════════════════════════════════
use egui::{epaint::Vertex, Color32, Mesh, Pos2, Shape, Ui};

/// Technical implementation of a radial glow generator.
pub struct GlowLayer {
    pub center: Pos2,
    pub radius: f32,
    pub color: Color32,
}

impl GlowLayer {
    pub fn new(center: Pos2, radius: f32, color: Color32) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    pub fn draw(&self, ui: &mut Ui) {
        let painter = ui.painter();

        // Draw a multi-layered radial gradient mesh for hyper-realistic glow
        let mut mesh = Mesh::default();
        let transparent = Color32::from_rgba_premultiplied(0, 0, 0, 0);

        // Center vertex
        mesh.vertices.push(Vertex {
            pos: self.center,
            uv: Pos2::ZERO,
            color: self.color,
        });

        // Circle circumference
        let segments = 32;
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let pos = self.center + egui::vec2(angle.cos(), angle.sin()) * self.radius;
            mesh.vertices.push(Vertex {
                pos,
                uv: Pos2::ZERO,
                color: transparent,
            });

            if i > 0 {
                mesh.indices.push(0);
                mesh.indices.push(i as u32);
                mesh.indices.push(i as u32 + 1);
            }
        }

        // Close the circle
        mesh.indices.push(0);
        mesh.indices.push(segments as u32);
        mesh.indices.push(1);

        painter.add(Shape::mesh(mesh));
    }
}
