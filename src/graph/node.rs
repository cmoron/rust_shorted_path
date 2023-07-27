//! Node module.
//!
//! This module contains the Node structure which represents a single node in a graph.
//! A node is identified by a unique id.

/// The Node structure.
///
/// This structure represents a single node in a graph. A node is identified by a unique id.
#[derive(Clone, Debug)]
pub struct Node {
    pub id: usize,
}

