//! Module pour l'algorithme de Dijkstra.
//!
//! Ce module contient la structure Dijkstra et son implémentation.

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

use crate::graph::{Graph, Node};
use crate::algorithms::shortest_path::ShortestPathAlgorithm;

// Représente l'état d'un noeud dans l'algorithme de Dijkstra,
// contenant l'identifiant du noeud et le coût pour y parvenir.
#[derive(Eq, PartialEq)]
struct State {
    node_id: usize,
    cost: u32,
}

// Implémente Ord pour State, ce qui nous permet de les comparer
// en fonction de leur coût pour déterminer quel noeud a le coût le plus bas.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Représente l'algorithme de Dijkstra.
pub struct Dijkstra {}

impl ShortestPathAlgorithm for Dijkstra {
    // Implémente l'algorithme de Dijkstra pour trouver le chemin le plus court entre deux noeuds.
    fn find_shortest_path(&self, graph: &Graph, start: &Node, end: &Node) -> Option<Vec<Node>> {
        // Initialise les distances et les noeuds précédents.
        let mut dist: HashMap<usize, u32> = HashMap::new();
        let mut prev: HashMap<usize, usize> = HashMap::new();

        // Stocke les noeuds pour reconstruire le chemin à la fin.
        let mut nodes: HashMap<usize, &Node> = HashMap::new();

        // Utilise un tas binaire pour stocker les noeuds par coût.
        let mut heap = BinaryHeap::new();

        // Ajoute le noeud de départ au tas avec un coût de 0.
        dist.insert(start.id, 0);
        heap.push(State { node_id: start.id, cost: 0 });

        // Associe chaque identifiant de noeud à son objet Node correspondant.
        for node in &graph.nodes {
            nodes.insert(node.id, node);
        }

        // Parcourt tous les noeuds dans le tas.
        while let Some(State { node_id, cost }) = heap.pop() {
            // Ignore les noeuds dont le coût est plus élevé que le coût stocké.
            if cost > *dist.get(&node_id).unwrap() { continue; }

            // Parcourt tous les voisins du noeud actuel.
            for edge in &graph.edges {
                if edge.node1 == node_id {
                    // Crée un nouvel état pour le voisin avec le coût du chemin jusqu'à ce point.
                    let next_node_id = edge.node2;
                    let next_cost = cost + edge.weight;

                    let next = State { node_id: next_node_id, cost: next_cost };

                    // Si le coût du voisin est inférieur au coût stocké, met à jour le coût et ajoute le voisin au tas.
                    if next.cost < *dist.get(&next.node_id).unwrap_or(&u32::MAX) {
                        heap.push(next);
                        dist.insert(next_node_id, next_cost);
                        prev.insert(next_node_id, node_id);
                    }
                }
            }
        }

        // Reconstruit le chemin en partant du noeud final.
        let mut path = Vec::new();
        let mut current = end.id;

        while current != start.id {
            if let Some(prev_node) = prev.get(&current) {
                path.push(nodes[&current].clone());
                current = *prev_node;
            } else {
                // Si aucun chemin n'a été trouvé, renvoie None.
                return None;
            }
        }

        path.push(nodes[&start.id].clone());
        path.reverse();

        // Renvoie le chemin en tant que vecteur de noeuds.
        Some(path)
    }
}

