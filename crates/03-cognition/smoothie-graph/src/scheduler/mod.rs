/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0x30d535bd | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/scheduler/mod.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

use super::nodes::NodeId;
use alloc::vec;
use alloc::vec::Vec;

/// Technical implementation of the ProcessOrder structure.
pub struct ProcessOrder {
    /// Node IDs in topological (dependency-first) order.
    pub order: Vec<NodeId>,
}

/// An edge connecting an upstream node's output to a downstream node's input.
#[derive(Debug, Clone, Copy)]
/// Technical implementation of the Edge structure.
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
    pub from_port: u8,
    pub to_port: u8,
}

/// Technical implementation of the GraphScheduler structure.
pub struct GraphScheduler {
    /// All registered edges (can only be mutated outside the audio thread).
    edges: Vec<Edge>,
    /// The resolved processing order, computed once on `build()`.
    order: Option<ProcessOrder>,
    next_id: u32,
}

impl GraphScheduler {
    /// Initializes a new instance of the associated type.
    pub fn new() -> Self {
        Self {
            edges: Vec::new(),
            order: None,
            next_id: 0,
        }
    }

    /// Register a new node and return its unique `NodeId`.
    pub fn register_node(&mut self) -> NodeId {
        let id = NodeId(self.next_id);
        self.next_id += 1;
        id
    }

    /// Connect `from_port` of `from` → `to_port` of `to`.
    pub fn connect(&mut self, from: NodeId, from_port: u8, to: NodeId, to_port: u8) {
        self.edges.push(Edge {
            from,
            to,
            from_port,
            to_port,
        });
        self.order = None; // Invalidate cached sort
    }

    /// Resolve the DAG using Kahn's algorithm. Must be called after every
    /// topology change and **before** the audio thread starts.
    pub fn build(&mut self) {
        let n = self.next_id as usize;
        let mut in_degree = vec![0usize; n];
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); n];

        for edge in &self.edges {
            adjacency[edge.from.0 as usize].push(edge.to.0 as usize);
            in_degree[edge.to.0 as usize] += 1;
        }

        // BFS queue seeded with zero-in-degree nodes (sources / generators)
        let mut queue: Vec<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::with_capacity(n);

        while let Some(current) = queue.first().copied() {
            queue.remove(0);
            order.push(NodeId(current as u32));
            for &neighbour in &adjacency[current] {
                in_degree[neighbour] -= 1;
                if in_degree[neighbour] == 0 {
                    queue.push(neighbour);
                }
            }
        }

        self.order = Some(ProcessOrder { order });
    }

    /// Returns an immutable reference to the resolved `ProcessOrder`.
    ///
    /// # Panics
    /// Panics if `build()` has not been called after the last topology change.
    pub fn process_order(&self) -> &ProcessOrder {
        self.order
            .as_ref()
            .expect("GraphScheduler::build() must be called before processing")
    }
}
