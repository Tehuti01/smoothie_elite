//! smoothie-graph — 'Elite' Hyper-Modular Node Graph engine.
//! High-performance, real-time safe Directed Acyclic Graph (DAG) for modular DSP.

use dasp_graph::{NodeData, Processor, Buffer};
use smoothie_params::Param;
use smoothie_core::Buffer as SmoothieBuffer;
use std::sync::Arc;

/// A trait representing a modular DSP node in the 'Elite' graph.
pub trait ModularNode: Send + Sync {
    /// Process the node's DSP logic.
    fn process(&mut self, inputs: &[&Buffer], outputs: &mut [Buffer], sample_rate: f64);
    
    /// Get the node's parameters for AURA orchestration.
    fn parameters(&self) -> Vec<Arc<dyn Param>>;
}

impl Node for Box<dyn ModularNode> {
    fn process(&mut self, inputs: &[&Buffer], outputs: &mut [Buffer]) {
        // Sample rate should be piped through context in later phases
        self.as_mut().process(inputs, outputs, 44100.0);
    }
}

/// A handle to a node within the dynamic audio graph.
pub struct NodeHandle {
    pub node_id: dasp_graph::NodeIndex,
}

/// The 'Elite' Audio Graph engine.
pub struct AudioGraph {
    processor: Processor<Box<dyn ModularNode>>,
}

impl AudioGraph {
    pub fn new() -> Self {
        Self {
            processor: Processor::with_capacity(1024),
        }
    }

    /// Process a block of audio through the entire modular ecosystem.
    pub fn process(&mut self) {
        // Traverses the DAG and executes nodes in topological order.
        self.processor.process(); 
    }

    /// Add a node to the graph and return its handle.
    pub fn add_node(&mut self, node: impl ModularNode + 'static) -> NodeHandle {
        let node_id = self.processor.add_node(Box::new(node));
        NodeHandle { node_id }
    }

    /// Connect two nodes in the modular chain.
    pub fn add_edge(&mut self, from: NodeHandle, to: NodeHandle, from_index: usize, to_index: usize) {
        self.processor.add_edge(from.node_id, to.node_id, from_index, to_index);
    }
}
