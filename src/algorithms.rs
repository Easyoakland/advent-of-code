use num_traits::Zero;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

/// Calculate the distance from start to end and the path of two nodes.
/// - The potential function must not overestimate distance between nodes and must not be negative.
/// - Edge weights must be positive.
/// - A potential of `|_| 0` is equivalent to Dijkstra
pub fn astar<Node, Distance, I>(
    start: Node,
    end: Node,
    neighbors: impl Fn(Node) -> I,
    potential: impl Fn(Node) -> Distance,
    neighbor_edge_weight: impl Fn(Node, Node) -> Distance,
) -> Option<Distance>
where
    Node: Eq + Hash + Copy,
    Distance: Zero + Add<Output = Distance> + Ord + Copy,
    I: Iterator<Item = Node>,
{
    // Dijkstra
    let mut boundary_nodes = HashSet::from([start]);
    let mut distances = HashMap::from([(start, Distance::zero())]);

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

        // If the end is reached return the distance to the end.
        if cur_node == end {
            return Some(distances[&end]);
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

    // If not found after full search then no valid path/distance.
    None
}
