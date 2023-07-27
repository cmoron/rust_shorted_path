use crate::graph::{Graph, Node};

/// `ShortestPathAlgorithm` is a trait representing a shortest path algorithm.
///
/// Any shortest path algorithm will implement this trait. It provides a 
/// method to find the shortest path between two nodes in a graph.
pub trait ShortestPathAlgorithm {
    /// Find the shortest path between `start` and `end` in the given `graph`.
    ///
    /// # Arguments
    ///
    /// * `graph` - A reference to the graph where to find the path.
    /// * `start` - The starting node.
    /// * `end` - The ending node.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<Node>)` - If a path is found, returns a vector of nodes 
    /// representing the path from start to end.
    /// * `None` - If no path is found, returns None.
    fn find_shortest_path(&self, graph: &Graph, start: &Node, end: &Node) -> Option<Vec<Node>>;
}

