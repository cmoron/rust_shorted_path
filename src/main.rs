//! Main module.
//!
//! This module contains the main function which creates a graph with nodes and edges,
//! and prints it to the console.

// Import our graph and algorithms modules.
mod graph;
mod algorithms;

// Use the Node, Edge and Graph structures from our graph module.
//use crate::graph::{Graph, Node, Edge};
use crate::algorithms::{dijkstra::Dijkstra, shortest_path::ShortestPathAlgorithm};
use crate::graph::graph_loader::load_from_file;
use crate::graph::graph_loader::retrieve_start_end_nodes;
use std::path::Path;
use std::env;

fn main() {

    // Get the arguments passed to the program. The first argument is the name of the program itself.
    let args: Vec<String> = env::args().collect();

    // Check if a file name was passed as an argument
    if args.len() < 2 {
        eprintln!("Please provide a file name");
        std::process::exit(1);
    }

    // Load the graph and the expected shortest path from the file.
    let (graph, expected_shortest_path) = load_from_file(Path::new(&args[1])).unwrap();
    let (start_node, end_node) = retrieve_start_end_nodes(expected_shortest_path.clone()).unwrap();

    // Print our graph to the console.
    println!("{}", graph);
    println!("{:?}", expected_shortest_path.clone());

    // Create a Dijkstra object
    let dijkstra = Dijkstra {};


    // Find the shortest path from node1 to node3
    if let Some(path) = dijkstra.find_shortest_path(&graph, start_node, end_node) {
        println!("Shortest path from node {} to node {}: {:?}", start_node, end_node, path);
    } else {
        println!("No path from node {} to node {}.", start_node, end_node);
    }
}

