//! Edge module.
//!
//! This module contains the Edge structure which represents an edge in a graph.
//! An edge connects two nodes (represented by their ids) and has a weight.

/// The Edge structure.
///
/// This structure represents an edge in a graph. An edge connects two nodes (represented by
/// their ids) and has a weight.
#[derive(Clone, Debug)]
pub struct Edge {
    /// The starting node of the edge.
    pub node1: usize,
    /// The ending node of the edge.
    pub node2: usize,
    /// The weight of the edge.
    pub weight: usize,
}
