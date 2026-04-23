/*
 *  S E R A P H I C   T E C H N O L O G I E S
 * ╭──────────────────────────────────────────────────────────────────────────╮
 * │ FILE ID: SER-0xf0eb7919 | REVISION: 2026.04.20                           │
 * │ PATH: smoothie_elite/crates/03-cognition/smoothie-graph/src/connection.rs                                                         │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ DESCRIPTION: Professional technical implementation and documentation.    │
 * ├──────────────────────────────────────────────────────────────────────────┤
 * │ TECHNICAL NOTES: Optimized for industrial-grade performance standards.   │
 * ╰──────────────────────────────────────────────────────────────────────────╯
 *   SERAPHIC TECH - Precision Engineering
 */

extern crate alloc;

use crate::nodes::NodeId;
use crate::port::PortDirection;
use alloc::vec::Vec;

pub const MAX_CONNECTIONS: usize = 512;
pub const MAX_PORTS_PER_NODE: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ConnectionState enumeration.
pub enum ConnectionState {
    Disconnected,
    Connected,
    Error,
}

/// Technical implementation of the Connection structure.
pub struct Connection {
    pub from_node: NodeId,
    pub from_port: u8,
    pub to_node: NodeId,
    pub to_port: u8,
    pub state: ConnectionState,
    pub buffer_handle: Option<usize>,
}

impl Connection {
    /// Initializes a new instance of the associated type.
    pub const fn new(from_node: NodeId, from_port: u8, to_node: NodeId, to_port: u8) -> Self {
        Self {
            from_node,
            from_port,
            to_node,
            to_port,
            state: ConnectionState::Disconnected,
            buffer_handle: None,
        }
    }

    /// Technical implementation of the is_valid logic.
    pub fn is_valid(&self) -> bool {
        self.from_node != self.to_node
    }
}

/// Technical implementation of the ConnectionManager structure.
pub struct ConnectionManager {
    connections: Vec<Connection>,
    #[allow(dead_code)]
    adjacency: Vec<Vec<usize>>,
    node_port_counts: [(u8, u8); 64],
}

impl ConnectionManager {
    /// Initializes a new instance of the associated type.
    pub const fn new() -> Self {
        Self {
            connections: Vec::new(),
            adjacency: Vec::new(),
            node_port_counts: [(0, 0); 64],
        }
    }

    /// Technical implementation of the register_node logic.
    pub fn register_node(&mut self, id: NodeId, inputs: u8, outputs: u8) {
        let idx = id.0 as usize;
        if idx < 64 {
            self.node_port_counts[idx] = (inputs, outputs);
        }
    }

    /// Technical implementation of the connect logic.
    pub fn connect(
        &mut self,
        from_node: NodeId,
        from_port: u8,
        to_node: NodeId,
        to_port: u8,
    ) -> Result<usize, ConnectionError> {
        if !self.validate_port(from_node, from_port, PortDirection::Output) {
            return Err(ConnectionError::InvalidSourcePort);
        }
        if !self.validate_port(to_node, to_port, PortDirection::Input) {
            return Err(ConnectionError::InvalidTargetPort);
        }

        let connection = Connection::new(from_node, from_port, to_node, to_port);
        let handle = self.connections.len();
        self.connections.push(connection);
        Ok(handle)
    }

    /// Technical implementation of the disconnect logic.
    pub fn disconnect(&mut self, handle: usize) -> Result<(), ConnectionError> {
        if handle < self.connections.len() {
            self.connections.remove(handle);
            Ok(())
        } else {
            Err(ConnectionError::NotFound)
        }
    }

    /// Technical implementation of the get_connections_from logic.
    pub fn get_connections_from(&self, node: NodeId) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| c.from_node == node)
            .collect()
    }

    /// Technical implementation of the get_connections_to logic.
    pub fn get_connections_to(&self, node: NodeId) -> Vec<&Connection> {
        self.connections
            .iter()
            .filter(|c| c.to_node == node)
            .collect()
    }

    /// Technical implementation of the get_connection logic.
    pub fn get_connection(&self, handle: usize) -> Option<&Connection> {
        self.connections.get(handle)
    }

    /// Technical implementation of the connections logic.
    pub fn connections(&self) -> &Vec<Connection> {
        &self.connections
    }

    /// Technical implementation of the validate_port logic.
    fn validate_port(&self, node: NodeId, port: u8, direction: PortDirection) -> bool {
        let idx = node.0 as usize;
        if idx >= 64 {
            return false;
        }
        let (inputs, outputs) = self.node_port_counts[idx];
        match direction {
            PortDirection::Input => port < inputs,
            PortDirection::Output => port < outputs,
        }
    }
}

impl Default for ConnectionManager {
    /// Technical implementation of the default logic.
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Technical implementation of the ConnectionError enumeration.
pub enum ConnectionError {
    InvalidSourcePort,
    InvalidTargetPort,
    NodeNotFound,
    AlreadyConnected,
    NotFound,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Technical implementation of the test_connection_creation logic.
    fn test_connection_creation() {
        let conn = Connection::new(NodeId(0), 0, NodeId(1), 0);
        assert!(conn.is_valid());
    }

    #[test]
    /// Technical implementation of the test_connection_manager logic.
    fn test_connection_manager() {
        let mut mgr = ConnectionManager::new();
        mgr.register_node(NodeId(0), 2, 2);
        mgr.register_node(NodeId(1), 2, 0);
    }
}
