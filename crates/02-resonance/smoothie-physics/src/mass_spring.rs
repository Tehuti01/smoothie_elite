/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x48bfacbc | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/02-resonance/smoothie-physics/src/mass_spring.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use alloc::vec::Vec;
use smoothie_core::primitives::Sample;

/// A single node in a physical mesh.
#[derive(Clone, Copy)]
#[repr(align(64))]
/// Technical implementation of the Node structure.
pub struct Node {
    pub position: f32,
    pub velocity: f32,
    pub mass: f32,
    pub damping: f32,
    pub fixed: bool,
}

impl Default for Node {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self {
            position: 0.0,
            velocity: 0.0,
            mass: 1.0,
            damping: 0.05,
            fixed: false,
        }
    }
}

/// A connection (spring) between two nodes.
#[repr(align(64))]
/// Technical implementation of the Spring structure.
pub struct Spring {
    pub node_a: usize,
    pub node_b: usize,
    pub stiffness: f32,
    pub rest_length: f32,
}

/// 1D or 2D generic mass-spring simulation network.
#[repr(align(64))]
/// Technical implementation of the PhysicalMesh structure.
pub struct PhysicalMesh {
    nodes: Vec<Node>,
    springs: Vec<Spring>,
    forces: Vec<f32>,
    pub sample_rate: f32,
}

impl PhysicalMesh {
    /// Initializes a new instance of the associated type.
    pub fn new(sample_rate: f32) -> Self {
        Self {
            nodes: Vec::new(),
            springs: Vec::new(),
            forces: Vec::new(),
            sample_rate,
        }
    }

    /// Performs vector addition logic.
    pub fn add_node(&mut self, node: Node) -> usize {
        let idx = self.nodes.len();
        self.nodes.push(node);
        self.forces.push(0.0);
        idx
    }

    /// Performs vector addition logic.
    pub fn add_spring(&mut self, node_a: usize, node_b: usize, stiffness: f32) {
        self.springs.push(Spring {
            node_a,
            node_b,
            stiffness,
            rest_length: 0.0, // Assuming initial positions are 0 or tension is absolute
        });
    }

    /// Technical implementation of the strike_node logic.
    pub fn strike_node(&mut self, node_idx: usize, velocity: f32) {
        if node_idx < self.nodes.len() && !self.nodes[node_idx].fixed {
            self.nodes[node_idx].velocity += velocity;
        }
    }

    /// Process one sample tick and return the displacement of a pickup node.
    #[inline(always)]
    pub fn process(&mut self, pickup_node: usize) -> Sample {
        let dt = 1.0 / self.sample_rate;

        // Reset forces
        for f in self.forces.iter_mut() {
            *f = 0.0;
        }

        // Calculate spring forces (Hooke's Law)
        for spring in self.springs.iter() {
            let pos_a = self.nodes[spring.node_a].position;
            let pos_b = self.nodes[spring.node_b].position;

            // F = -k * x
            let force = spring.stiffness * (pos_b - pos_a);

            self.forces[spring.node_a] += force;
            self.forces[spring.node_b] -= force;
        }

        // Apply forces & integrate
        for (i, node) in self.nodes.iter_mut().enumerate() {
            if node.fixed {
                continue;
            }

            // Damping force: F = -c * v
            let damping_force = -node.damping * node.velocity;
            let total_force = self.forces[i] + damping_force;

            // a = F / m
            let acceleration = total_force / node.mass;

            // Semi-implicit Euler integration
            node.velocity += acceleration * dt;
            node.position += node.velocity * dt;
        }

        if pickup_node < self.nodes.len() {
            self.nodes[pickup_node].position
        } else {
            0.0
        }
    }
}
