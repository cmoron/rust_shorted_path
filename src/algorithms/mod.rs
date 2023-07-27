/// This module contains the different path finding algorithms.
pub mod shortest_path;
pub mod dijkstra;

/// Re-export the `ShortestPathAlgorithm` trait at the root of the module for easy access.
pub use shortest_path::ShortestPathAlgorithm;
pub use dijkstra::Dijkstra;
