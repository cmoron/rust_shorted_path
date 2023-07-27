//! Main module.
//!
//! This module contains the main function which creates a graph with nodes and edges,
//! and prints it to the console.

// Import our graph and algorithms modules.
mod graph;
mod algorithms;

// Use the Node, Edge and Graph structures from our graph module.
use crate::graph::{Graph, Node, Edge};
use crate::algorithms::{dijkstra::Dijkstra, shortest_path::ShortestPathAlgorithm};

fn main() {

    // Create nodes
    let node1 = Node { id: 1 };
    let node2 = Node { id: 2 };
    let node3 = Node { id: 3 };
    let node4 = Node { id: 4 };
    let node5 = Node { id: 5 };
    let node6 = Node { id: 6 };

    let edge1 = Edge { node1: 1, node2: 2, weight: 1 };
    let edge2 = Edge { node1: 1, node2: 3, weight: 3 };
    let edge3 = Edge { node1: 2, node2: 3, weight: 1 };
    let edge4 = Edge { node1: 3, node2: 4, weight: 2 };
    let edge5 = Edge { node1: 2, node2: 4, weight: 5 };
    let edge6 = Edge { node1: 4, node2: 5, weight: 1 };
    let edge7 = Edge { node1: 5, node2: 6, weight: 1 };
    let edge8 = Edge { node1: 1, node2: 6, weight: 10 };

    let graph = Graph {
        nodes: vec![node1.clone(), node2.clone(), node3.clone(), node4.clone(), node5.clone(), node6.clone()],
        edges: vec![edge1.clone(), edge2.clone(), edge3.clone(), edge4.clone(), edge5.clone(), edge6.clone(), edge7.clone(), edge8.clone()],
    };

    // Print our graph to the console.
    println!("{}", graph);

    // Create a Dijkstra object
    let dijkstra = Dijkstra {};

    // Find the shortest path from node1 to node3
    if let Some(path) = dijkstra.find_shortest_path(&graph, &node1, &node6) {
        println!("Shortest path from node {} to node {}: {:?}", node1.id, node6.id, path);
    } else {
        println!("No path from node {} to node {}.", node1.id, node6.id);
    }
}

