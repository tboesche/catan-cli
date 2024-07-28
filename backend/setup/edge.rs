use std::collections::HashSet;
use std::collections::HashMap;

// Helper function to sort and create a tuple from two nodes
pub fn make_edge(a: u32, b: u32) -> (u32, u32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// Function to create unique list of edges from Vec<Vec<Option<u32>>>
pub fn create_unique_edges(node_adjacency: Vec<Vec<Option<u32>>>) -> Vec<(u32, u32)> {
    let mut edges = HashSet::new();

    for (i, neighbors) in node_adjacency.iter().enumerate() {
        for &neighbor in neighbors {
            if let Some(neighbor) = neighbor {
                edges.insert(make_edge(i as u32, neighbor));
            }
        }
    }

    edges.into_iter().collect()
}

// Function to return the number of unique edges
fn count_unique_edges(edges: &Vec<(u32, u32)>) -> usize {
    edges.len()
}

// Function to get a unique index for an edge
pub fn edge_index_map(edges: &Vec<(u32, u32)>) -> HashMap<(u32, u32), usize> {
    let mut edge_map = HashMap::new();
    for (index, &edge) in edges.iter().enumerate() {
        edge_map.insert(edge, index);
    }
    edge_map
}