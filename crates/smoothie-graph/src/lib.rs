//! smoothie-graph — 'Elite' Hyper-Modular Node Graph engine.
//! High-performance, real-time safe Directed Acyclic Graph (DAG) for modular DSP.

use dasp_graph::{NodeData, Processor, Buffer, Node, Input};
use smoothie_params::Param;
use std::sync::Arc;
use petgraph::graph::Graph;
use std::fmt;

/// A trait representing a modular DSP node in the 'Elite' graph.
pub trait ModularNode: Send + Sync {
    /// Process the node's DSP logic.
    fn process(&mut self, inputs: &[Input], outputs: &mut [Buffer], sample_rate: f64);
    
    /// Get the node's parameters for AURA orchestration.
    fn parameters(&self) -> Vec<Arc<dyn Param>>;
}

/// The 'Elite' node wrapper for dasp_graph.
pub struct SmtNode {
    pub inner: Box<dyn ModularNode>,
}

impl Node for SmtNode {
    fn process(&mut self, inputs: &[Input], outputs: &mut [Buffer]) {
        // Sample rate should be piped through context in later phases
        self.inner.process(inputs, outputs, 44100.0);
    }
}

/// A handle to a node within the dynamic audio graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeHandle {
    pub node_id: petgraph::graph::NodeIndex,
}

/// The 'Elite' Audio Graph engine.
pub struct AudioGraph {
    graph: Graph<NodeData<SmtNode>, ()>,
    processor: Processor<Graph<NodeData<SmtNode>, ()>>,
    output_node: Option<NodeHandle>,
}

impl fmt::Debug for AudioGraph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AudioGraph")
            .field("node_count", &self.graph.node_count())
            .field("output_node", &self.output_node)
            .finish()
    }
}

impl AudioGraph {
    pub fn new() -> Self {
        let graph = Graph::new();
        Self {
            graph,
            processor: Processor::with_capacity(1024),
            output_node: None,
        }
    }

    /// Process a block of audio through the entire modular ecosystem.
    pub fn process(&mut self) {
        if let Some(out) = &self.output_node {
            self.processor.process(&mut self.graph, out.node_id); 
        }
    }

    /// Add a node to the graph and return its handle.
    pub fn add_node(&mut self, node: impl ModularNode + 'static) -> NodeHandle {
        let node_data = NodeData::new(SmtNode { inner: Box::new(node) }, vec![Buffer::default()]);
        let node_id = self.graph.add_node(node_data);
        NodeHandle { node_id }
    }
    
    /// Mark a node as the final output node.
    pub fn set_output(&mut self, handle: NodeHandle) {
        self.output_node = Some(handle);
    }

    /// Connect two nodes in the modular chain.
    pub fn add_edge(&mut self, from: NodeHandle, to: NodeHandle, _from_index: usize, _to_index: usize) {
        self.graph.add_edge(from.node_id, to.node_id, ());
    }
}


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
#[allow(dead_code, non_upper_case_globals)]
const __PHI: f64 = 1.618033988749895;
#[allow(dead_code, non_upper_case_globals)]
const __PI: f64 = 3.141592653589793;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_5TH: f64 = 1.5;
#[allow(dead_code, non_upper_case_globals)]
const __PYTHAG_4TH: f64 = 1.333333333333333;
#[allow(dead_code)]
#[inline(always)]
fn __resonate_omni() -> f64 { __PHI * __PI * __PYTHAG_5TH }
// ---------------------------------------
