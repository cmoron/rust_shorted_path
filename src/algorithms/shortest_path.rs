use crate::graph::Graph;

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
    /// * `start` - The starting node id.
    /// * `end` - The ending node id.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<usize>)` - If a path is found, returns a vector of nodes id.
    /// representing the path from start to end.
    /// * `None` - If no path is found, returns None.
    fn find_shortest_path(&self, graph: &Graph, start: usize, end: usize) -> Option<Vec<usize>>;
}

