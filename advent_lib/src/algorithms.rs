use num_traits::{bounds::UpperBounded, SaturatingAdd, Zero};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

/// Calculate the distance from start to end and optionally the shortest path between nodes if a path exists.
/// # Notes
/// - The potential function must not overestimate distance between nodes and must not be negative.
/// - A potential of `|_| 0` is equivalent to Dijkstra
/// # Panics
/// - Edge weights must be positive.
pub fn astar<Node, Distance, I>(
    start: Node,
    end: Node,
    neighbors: impl Fn(Node) -> I,
    potential: impl Fn(Node) -> Distance,
    neighbor_edge_weight: impl Fn(Node, Node) -> Distance,
    reconstruct_path: bool,
) -> Option<(Distance, Option<Vec<Node>>>)>
where
    Node: Eq + Hash + Copy,
    Distance: Zero + Add<Output = Distance> + Ord + Copy,
    I: Iterator<Item = Node>,
{
    fn reconstruct_path(backtrack: HashMap<Node, Option<Node>>) -> Vec<Node> {
        todo!()
    }
    // Boundary nodes are the nodes at the edge to pick from to explore next.
    let mut boundary_nodes = HashSet::from([start]);
    // Distances is a map of the shortest known distance to any node from the start.
    let mut distances = HashMap::from([(start, Distance::zero())]);
    // Reconstructed path is the shortest path from the start to the end. Each node knows its parent and this means the path can be backtracked.
    let backtrack = if backtrack {
        Some(HashMap::from([(start, None)]))
    } else {
        None
    };

    while !boundary_nodes.is_empty() {
        // Remove closest node defined by distance + potential of node.
        let cur_node = *boundary_nodes
            .iter()
            .min_by_key(|&&x| {
                let potential = potential(x);
                assert!(
                    potential >= Distance::zero(),
                    "It is invalid to have a negative potential."
                );
                distances[&x] + potential
            })
            .unwrap();
        boundary_nodes.remove(&cur_node);

        // If the end is reached return the distance to the end and the path.
        if cur_node == end {
            return Some((distances[&end], backtrack.map(|x| reconstruct_path(x))));
        }

        // Increase scope of neighbors to neighbors of `cur_node`
        for neighbor in neighbors(cur_node) {
            let neighbor_edge_weight = neighbor_edge_weight(cur_node, neighbor);
            assert!(
                neighbor_edge_weight >= Distance::zero(),
                "It is invalid to have negative edge weights."
            );
            let proposed_distance = distances[&cur_node] + neighbor_edge_weight;

            // If don't already have a distance for the specified node or if the new distance is shorter
            // replace the new distance for the neighbor
            if !distances.contains_key(&neighbor) || proposed_distance < distances[&neighbor] {
                distances.insert(neighbor, proposed_distance);
                boundary_nodes.insert(neighbor);
            }
        }
    }

    // If not found after full search then no valid distance/path.
    None
}

/// Calculate the distance between all nodes to all other nodes.
/// [Floyd-Warshall](https://en.wikipedia.org/wiki/Floyd%E2%80%93Warshall_algorithm).
/// Assumes that all nodes are connected through some sequence of edges.
pub fn floyd_warshall<Node, Distance, I>(keys: I, distances: &mut BTreeMap<(Node, Node), Distance>)
where
    Node: Ord + Copy,
    Distance: UpperBounded + Ord + SaturatingAdd + Copy,
    I: Iterator<Item = Node> + Clone,
{
    for k in keys.clone() {
        for j in keys.clone() {
            for i in keys.clone() {
                // Initialize missing distances with infinity.
                let i_k = *distances.entry((i, k)).or_insert(Distance::max_value());
                let k_j = *distances.entry((k, j)).or_insert(Distance::max_value());
                let i_j = distances.entry((i, j)).or_insert(Distance::max_value());

                // Insert shorter distance from i to j through k distance if it is shorter than shortest from i to j.
                *i_j = (*i_j).min(i_k.saturating_add(&k_j));
            }
        }
    }
}
