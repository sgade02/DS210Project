use petgraph::algo::dijkstra;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod stats;

fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}


// Load edges into a graph
fn load_edges(file_path: &str) -> Graph<(), (), Undirected> {
    let mut graph = Graph::new_undirected();
    let mut node_map = HashMap::new();

    let file = File::open(file_path).expect("Failed to open edges file");
    for line in io::BufReader::new(file).lines() {
        let line = line.expect("Failed to read line");
        let nodes: Vec<&str> = line.split_whitespace().collect();
        if nodes.len() == 2 {
            let u = *node_map.entry(nodes[0].to_string()).or_insert_with(|| graph.add_node(()));
            let v = *node_map.entry(nodes[1].to_string()).or_insert_with(|| graph.add_node(()));
            graph.add_edge(u, v, ());
        }
    }
    graph
}

fn count_lines(file_path: &str) -> io::Result<usize> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().count())
}

/// Analyzes `.edges`, `.circles`, and `.feat` files and returns their line counts.
fn analyze_files(directory: &str) -> io::Result<HashMap<String, usize>> {
    let mut counts = HashMap::new();
    let path = Path::new(directory);

    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let file_name = entry.file_name().into_string().unwrap_or_default();
            let file_path = entry.path().to_string_lossy().to_string();

            if file_name.ends_with(".edges") || file_name.ends_with(".circles") || file_name.ends_with(".feat") {
                let line_count = count_lines(&file_path)?;
                counts.insert(file_name, line_count);
            }
        }
    }
    Ok(counts)
}

// Calculate degree distribution
fn calculate_degree_distribution(graph: &Graph<(), (), Undirected>) -> HashMap<usize, usize> {
    let mut degree_counts = HashMap::new();
    for node in graph.node_indices() {
        let degree = graph.neighbors(node).count();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }
    degree_counts
}

fn print_sorted_degrees(degree_distribution: &HashMap<usize, usize>) {
    let mut degrees: Vec<usize> = degree_distribution.keys().cloned().collect();
    degrees.sort(); // Sort the degrees in ascending order

    println!("Degrees sorted from lowest to highest:");
    for degree in degrees {
        println!("Degree: {}, Count: {}", degree, degree_distribution[&degree]);
    }
}


fn main() {
    // Path to the `.edges` file for the chosen ego network
    let file_path = "0.edges";

    match analyze_files(file_path) {
        Ok(counts) => {
            println!("Preliminary Analysis:");
            for (file, count) in counts {
                if file.ends_with(".edges") {
                    println!("File: {} - Edges: {}", file, count);
                } else if file.ends_with(".circles") {
                    println!("File: {} - Circles: {}", file, count);
                } else if file.ends_with(".feat") {
                    println!("File: {} - Features: {}", file, count);
                }
            }
        }
        Err(e) => eprintln!("Error analyzing files: {}", e),
    }

    // Load the graph
    let graph = load_edges(file_path);
    println!("Graph loaded with {} nodes and {} edges", graph.node_count(), graph.edge_count());

    // Calculate degree distribution
    let degree_distribution = calculate_degree_distribution(&graph);
    println!("Degree distribution: {:?}", degree_distribution);

    // Print degrees in sorted order
    print_sorted_degrees(&degree_distribution);

    let mean_separation = crate::stats::calculate_mean_separation(&graph);
    println!("Mean separation: {:.2}", mean_separation);

    let std_dev_separation = crate::stats::calculate_standard_deviation_separation(&graph);
    println!("Standard deviation of separation: {:.2}", std_dev_separation);

    let median_separation = crate::stats::calculate_median_separation(&graph);
    println!("Median separation: {:.2}", median_separation);


}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_load_edges() {
        // Create a temporary `.edges` file
        let file_path = "test.edges";
        std::fs::write(file_path, "1 2\n2 3\n3 4\n").unwrap();

        // Load the graph
        let graph = load_edges(file_path);

        // Assert graph properties
        assert_eq!(graph.node_count(), 4); // 4 unique nodes
        assert_eq!(graph.edge_count(), 3); // 3 edges

        // Cleanup
        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_calculate_degree_distribution() {
        // Create a graph with known degrees
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let n0 = graph.add_node(()); // Node 0
        let n1 = graph.add_node(()); // Node 1
        let n2 = graph.add_node(()); // Node 2
        graph.add_edge(n0, n1, ()); // Edge 0-1
        graph.add_edge(n1, n2, ()); // Edge 1-2
        graph.add_edge(n2, n0, ()); // Edge 2-0

        let degree_distribution = calculate_degree_distribution(&graph);

        // Assert degree distribution
        assert_eq!(degree_distribution[&2], 3); // Each node has degree 2
    }

    #[test]
    fn test_calculate_mean_separation() {
        // Create a simple triangle graph
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let n0 = graph.add_node(()); // Node 0
        let n1 = graph.add_node(()); // Node 1
        let n2 = graph.add_node(()); // Node 2
        graph.add_edge(n0, n1, ()); // Edge 0-1
        graph.add_edge(n1, n2, ()); // Edge 1-2
        graph.add_edge(n2, n0, ()); // Edge 2-0

        let mean_separation = crate::stats::calculate_mean_separation(&graph);

        // Assert mean separation (all nodes are 1 or 2 steps apart)
        assert_eq!(mean_separation, 1.0);
    }

    #[test]
    fn test_calculate_standard_deviation_separation() {
        // Create a simple graph
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let n0 = graph.add_node(()); // Node 0
        let n1 = graph.add_node(()); // Node 1
        let n2 = graph.add_node(()); // Node 2
        let n3 = graph.add_node(()); // Node 3
        graph.add_edge(n0, n1, ()); // Edge 0-1
        graph.add_edge(n1, n2, ()); // Edge 1-2
        graph.add_edge(n2, n3, ()); // Edge 2-3

        let std_dev = crate::stats::calculate_standard_deviation_separation(&graph);

        // Assert standard deviation
        assert!(std_dev > 0.0); // Ensure variability is non-zero
    }

    #[test]
    fn test_calculate_median_separation() {
        // Create a simple graph
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let n0 = graph.add_node(()); // Node 0
        let n1 = graph.add_node(()); // Node 1
        let n2 = graph.add_node(()); // Node 2
        let n3 = graph.add_node(()); // Node 3
        graph.add_edge(n0, n1, ()); // Edge 0-1
        graph.add_edge(n1, n2, ()); // Edge 1-2
        graph.add_edge(n2, n3, ()); // Edge 2-3

        let median = crate::stats::calculate_median_separation(&graph);

        // Assert median separation
        assert_eq!(median, 1.5); // Median of [1, 1, 1, 2, 2, 3]
    }

    #[test]
    fn test_print_sorted_degrees() {
        // Create a graph
        let mut graph = Graph::<(), (), Undirected>::new_undirected();
        let n0 = graph.add_node(()); // Node 0
        let n1 = graph.add_node(()); // Node 1
        let n2 = graph.add_node(()); // Node 2
        graph.add_edge(n0, n1, ()); // Edge 0-1
        graph.add_edge(n1, n2, ()); // Edge 1-2

        let degree_distribution = calculate_degree_distribution(&graph);

        // Print sorted degrees
        print_sorted_degrees(&degree_distribution);

        // Validate sorted order (manually inspect or compare output programmatically)
    }
}
