//! Graph Loader module.
//!
//! This module provides functions to load and handle graph data from a file. 
//! The file must have the following sections:
//!
//! - `# Nodes`: followed by lines with the ids of the nodes
//! - `# Edges`: followed by lines with the ids of the nodes connected by the edge and the weight of the edge
//! - `# ShortestPath`: followed by a line with the ids of the nodes representing the start and end of the shortest path
//!
//! The `load_from_file` function reads a graph from a file and creates a `Graph` structure from the read data.
//!
//! The `retrieve_start_end_nodes` function takes a vector representing the expected shortest path and 
//! extracts the start and end nodes from it. It is used to validate and process the shortest path data read from the file.

use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

use crate::graph::Graph;
use crate::graph::edge::Edge;
use crate::graph::node::Node;

/// Load a graph and the shortest path to find from a file.
///
/// # Arguments
///
/// * `path` - A path to the file to load the graph from.
///
/// # Returns
///
/// * `Graph` - The loaded graph.
/// * `Vec<usize>` - The list of Edges of the shortest path to find.
///
/// # Errors
///
/// This function will return an error if the file cannot be opened, the file format is invalid, or the data is invalid.
pub fn load_from_file(path: &Path) -> io::Result<(Graph, Vec<usize>)> {
    // Open the file.
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Initialize the nodes, edges, and shortest path.
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    let mut shortest_path = Vec::new();

    // Read each line of the file.
    let mut section = None;
    for line in reader.lines() {
        let line = line?;
        // If the line starts with "#", it's a section header.
        if line.starts_with('#') {
            section = match line.as_str() {
                "# Nodes" => Some("nodes"),
                "# Edges" => Some("edges"),
                "# ShortestPath" => Some("shortest_path"),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Invalid section")),
            };
        } else if let Some(section) = section {
            match section {
                "nodes" => {
                    // Parse the node id.
                    let id: usize = line.parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid node id"))?;
                    nodes.push(Node { id });
                }
                "edges" => {
                    // Split the line into parts.
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    // Check if the line has the correct number of parts.
                    if parts.len() != 3 {
                        return Err(Error::new(ErrorKind::InvalidData, "Invalid edge data"));
                    }
                    // Parse the nodes and the weight of the edge.
                    let node1: usize = parts[0].parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid node id"))?;
                    let node2: usize = parts[1].parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid node id"))?;
                    let weight: usize = parts[2].parse().map_err(|_| Error::new(ErrorKind::InvalidData, "Invalid weight"))?;
                    edges.push(Edge { node1, node2, weight });
                }
                "shortest_path" => {
                    shortest_path = line.split_whitespace()
                        .map(|s| s.parse().unwrap())
                        .collect();
                }
                _ => {}
            }
        }
    }

    // Return the graph and the shortest path.
    Ok((Graph { nodes, edges }, shortest_path))
}

/// Retrieve the start and end nodes from a given shortest path.
///
/// This function retrieves the start and end nodes from the provided shortest path. The start node is considered to be the first element in the vector, and the end node is the last element in the vector.
///
/// # Arguments
///
/// * `expected_shortest_path` - A vector of `usize` representing the expected shortest path.
///
/// # Returns
///
/// * `Ok((usize, usize))` - If the expected_shortest_path vector has at least one element, returns a tuple of two `usize` values representing the start and end nodes respectively.
/// * `Err(Error)` - If the expected_shortest_path vector is empty (i.e., no start or end node is provided), returns an `Err` with `ErrorKind::InvalidData`.
///
/// # Errors
///
/// This function will return an error if the expected_shortest_path vector is empty.
///
/// # Example
///
/// ```
/// let expected_shortest_path = vec![1, 2, 3, 4, 5];
///
/// match retrieve_start_end_nodes(expected_shortest_path) {
///     Ok((start, end)) => println!("Start node: {}, End node: {}", start, end),
///     Err(e) => println!("An error occurred: {}", e),
/// }
/// ```
pub fn retrieve_start_end_nodes(expected_shortest_path: Vec<usize>) -> io::Result<(usize, usize)> {
    let start_node = match expected_shortest_path.first() {
        Some(start) => *start,
        None => {
            return Err(Error::new(ErrorKind::InvalidData, "No start node provided"));
        },
    };

    let end_node = match expected_shortest_path.last() {
        Some(end) => *end,
        None => {
            return Err(Error::new(ErrorKind::InvalidData, "No end node provided"));
        },
    };

    Ok((start_node, end_node))
}
