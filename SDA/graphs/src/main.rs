mod undirected_graph;
use undirected_graph::UG;
use std::io;

fn main() -> io::Result<()> {
    let file_path = "src/file.in";

    let mut undirected_graph = UG::new();
    undirected_graph.read_build(file_path)?;
    
    undirected_graph.print_matrix();

    undirected_graph.dfs();
    undirected_graph.bfs_and_cc();
    
    Ok(())
}
