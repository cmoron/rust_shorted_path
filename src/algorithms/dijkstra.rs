//! Dijkstra module.
//!
//! This module contains the Dijkstra structure and its implementation.

use crate::graph::Graph;
use crate::algorithms::shortest_path::ShortestPathAlgorithm;
use std::collections::HashMap;
use std::usize::MAX;

/// The Dijkstra structure.
///
/// This structure represents the Dijkstra shortest path algorithm.
pub struct Dijkstra {}

impl ShortestPathAlgorithm for Dijkstra {
    /// Find the shortest path between `start` and `end` in the given `graph`.
    ///
    /// This method implements Dijkstra's algorithm to find the shortest path from the start node to
    /// the end node. The graph must be connected; otherwise, the function will return None.
    ///
    /// # Arguments
    ///
    /// * `graph` - A reference to the graph where to find the path.
    /// * `start` - The starting node id.
    /// * `end` - The ending node id.
    ///
    /// # Returns
    ///
    /// * `Some(Vec<usize>)` - If a path is found, returns a vector of node ids representing the path from start to end.
    /// * `None` - If no path is found, returns None.
    ///
    /// # Example
    ///
    /// ```
    /// let dijkstra = Dijkstra {};
    /// let shortest_path = dijkstra.find_shortest_path(&graph, 1, 6);
    ///
    /// match shortest_path {
    ///     Some(path) => println!("The shortest path is: {:?}", path),
    ///     None => println!("There is no path between the nodes."),
    /// }
    /// ```
    fn find_shortest_path(&self, graph: &Graph, start: usize, end: usize) -> Option<Vec<usize>> {
        // Initialize the distances map with the maximum possible value.
        // This represents the current known distance from `start` to each node.
        // Initialize `start` with distance 0.
        let mut distances: HashMap<usize, usize> = HashMap::new();

        // This map will hold the previous node for each node on the shortest path.
        // It's how we'll reconstruct the shortest path at the end.
        let mut previous: HashMap<usize, usize> = HashMap::new();

        // List of unvisited nodes. We start with all nodes unvisited.
        let mut nodes: Vec<usize> = Vec::new();

        // Initialize the data structures.
        for node in &graph.nodes {
            distances.insert(node.id, MAX);
            nodes.push(node.id);
        }
        distances.insert(start, 0);

        // Main loop. We will keep running until we've visited all nodes, or
        // we've confirmed the shortest path to the end node.
        while let Some((closest_node, _)) = nodes.iter().map(|n| (*n, *distances.get(n).unwrap())).min_by_key(|&(_, dist)| dist) {
            // We're done if the closest node is the end node or all remaining nodes are inaccessible from the start node.
            if closest_node == end || distances[&closest_node] == MAX {
                break;
            }

            // Remove this node from the list of unvisited nodes.
            nodes.retain(|&n| n != closest_node);

            // Update distances to neighboring nodes.
            for edge in &graph.edges {
                // Determine the neighbor and the distance to that neighbor via the closest_node.
                let (neighbor, alt_dist) = if edge.node1 == closest_node { (edge.node2, distances[&closest_node] + edge.weight) }
                else if edge.node2 == closest_node { (edge.node1, distances[&closest_node] + edge.weight) }
                else { continue };

                // If we've found a shorter path to the neighbor, update the distance and previous node.
                if alt_dist < distances[&neighbor] {
                    distances.insert(neighbor, alt_dist);
                    previous.insert(neighbor, closest_node);
                }
            }
        }

        // Build the shortest path by following the trail from end to start.
        let mut path = Vec::new();
        let mut current = end;

        while let Some(prev_node) = previous.get(&current) {
            path.push(current);
            current = *prev_node;
        }
        path.push(current);

        // The path is from `end` to `start`, so reverse it and return.
        path.reverse();

        // If the path is empty or it doesn't start at `start`, then there's no path from `start` to `end`.
        if path.is_empty() || *path.first().unwrap() != start {
            None
        } else {
            Some(path)
        }
    }
}

#[cfg(test)]
mod tests {
    //! This module contains the tests for the Dijkstra module.
    use super::*;
    use std::path::PathBuf;
    use crate::graph::graph_loader::load_from_file;
    use crate::graph::graph_loader::retrieve_start_end_nodes;

    /// This function returns all the file paths in the "resources" directory.
    ///
    /// # Returns
    ///
    /// * `Vec<PathBuf>` - A vector of PathBuf objects each representing a file path in the "resources" directory.
    fn get_resource_files() -> Vec<PathBuf> {
        let mut files = Vec::new();
        let dir = PathBuf::from("resources");

        if dir.is_dir() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    files.push(path);
                }
            }
        }

        files
    }

    #[test]
    /// Test the Dijkstra's find_shortest_path function using the graph files from the "resources" directory.
    ///
    /// This test function uses each file in the "resources" directory to create a graph and test the Dijkstra's find_shortest_path function.
    /// It compares the output of the function with the expected output from the file.
    fn test_find_shortest_path() {
        let dijkstra = Dijkstra {};
        let files = get_resource_files();
        for file in &files {
            let (graph, expected_shortest_path) = load_from_file(file).unwrap();
            let (start_node, end_node) = retrieve_start_end_nodes(expected_shortest_path.clone()).unwrap();

            let result = dijkstra.find_shortest_path(&graph, start_node, end_node);
            let mut expected_result = Some(expected_shortest_path);
            if file.to_string_lossy() == "resources/not_connected_graph.txt" {
                expected_result = None;
            }
            assert_eq!(result, expected_result, "Failed on file: {}", file.display());
        }
    }
}

