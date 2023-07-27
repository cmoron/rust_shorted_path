//! Graph module.
//! 
//! This module contains the Node and Edge structures, and the Graph structure which consists
//! of a list of nodes and edges. The Display trait is implemented for the Graph structure to
//! allow printing it to the console.

// Import the node and edge modules.
mod node;
mod edge;

// Re-export the Node and Edge structures so they can be used outside this module.
pub use node::Node;
pub use edge::Edge;

use std::fmt;

/// The Graph structure.
///
/// This structure represents a graph which consists of a list of nodes and edges.
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print all nodes.
        for node in &self.nodes {
            writeln!(f, "Node: {}", node.id)?;
        }
        // Print all edges.
        for edge in &self.edges {
            writeln!(f, "Edge: {} - {}, weight: {}", edge.node1, edge.node2, edge.weight)?;
        }
        Ok(())
    }
}

