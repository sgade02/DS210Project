use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;

pub fn calculate_mean_separation(graph: &Graph<(), (), Undirected>) -> f64 {
    let mut total_distance = 0.0;
    let mut pair_count = 0;

    for start_node in graph.node_indices() {
        // Use Dijkstra's algorithm to calculate shortest paths from the start_node
        let distances = dijkstra(graph, start_node, None, |_| 1);

        for &distance in distances.values() {
            if distance > 0 {
                total_distance += distance as f64;
                pair_count += 1;
            }
        }
    }

    if pair_count == 0 {
        return 0.0; // Avoid division by zero
    }

    total_distance / pair_count as f64
}

pub fn calculate_standard_deviation_separation(graph: &Graph<(), (), Undirected>) -> f64 {
    let mut distances = vec![];

    // Collect all shortest path lengths
    for start_node in graph.node_indices() {
        let shortest_paths = dijkstra(graph, start_node, None, |_| 1);
        for &distance in shortest_paths.values() {
            if distance > 0 {
                distances.push(distance as f64);
            }
        }
    }

    // If there are no distances, standard deviation is undefined
    if distances.is_empty() {
        return 0.0;
    }

    // Calculate mean
    let mean: f64 = distances.iter().sum::<f64>() / distances.len() as f64;

    // Calculate variance
    let variance: f64 = distances
        .iter()
        .map(|&distance| (distance - mean).powi(2))
        .sum::<f64>()
        / distances.len() as f64;

    // Standard deviation is the square root of variance
    variance.sqrt()
}

pub fn calculate_median_separation(graph: &Graph<(), (), Undirected>) -> f64 {
    let mut distances = vec![];

    // Collect all shortest path lengths
    for start_node in graph.node_indices() {
        let shortest_paths = dijkstra(graph, start_node, None, |_| 1);
        for &distance in shortest_paths.values() {
            if distance > 0 {
                distances.push(distance as f64);
            }
        }
    }

    // If there are no distances, median is undefined
    if distances.is_empty() {
        return 0.0;
    }

    // Sort distances to find the median
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mid = distances.len() / 2;
    if distances.len() % 2 == 0 {
        // Even number of elements: median is the average of the two middle values
        (distances[mid - 1] + distances[mid]) / 2.0
    } else {
        // Odd number of elements: median is the middle value
        distances[mid]
    }
}


