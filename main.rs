use petgraph::graph::{Graph, NodeIndex};
use petgraph::Undirected;
use plotters::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

// Plot degree distribution on a log-log scale
fn plot_degree_distribution(
    degree_counts: &HashMap<usize, usize>,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_degree = degree_counts.keys().copied().max().unwrap_or(1);
    let max_count = degree_counts.values().copied().max().unwrap_or(1);

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution (Log-Log Scale)", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            (1..=max_degree).log_scale(),
            (1..=max_count).log_scale(),
        )?;

    chart
        .configure_mesh()
        .disable_mesh()
        .x_desc("Degree")
        .y_desc("Frequency")
        .draw()?;

    chart.draw_series(
        degree_counts
            .iter()
            .map(|(&degree, &count)| Circle::new((degree, count), 5, BLUE.filled())),
    )?;

    Ok(())
}

fn main() {
    // Path to the `.edges` file for the chosen ego network
    let file_path = "path/to/0.edges";

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

    // Step 1: Load the graph
    let graph = load_edges(file_path);
    println!("Graph loaded with {} nodes and {} edges", graph.node_count(), graph.edge_count());

    // Step 2: Calculate degree distribution
    let degree_distribution = calculate_degree_distribution(&graph);
    println!("Degree distribution: {:?}", degree_distribution);

    // Step 3: Plot the degree distribution
    let output_file = "degree_distribution.png";
    if let Err(e) = plot_degree_distribution(&degree_distribution, output_file) {
        eprintln!("Error plotting degree distribution: {}", e);
    } else {
        println!("Degree distribution plotted to {}", output_file);
    }
}
