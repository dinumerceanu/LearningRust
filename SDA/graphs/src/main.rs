mod undirected_graph;
use undirected_graph::UG;
mod dsu;
use dsu::DSU;
use std::io;

// fn main() -> io::Result<()> {
//     let file_path = "src/file.in";

//     let mut undirected_graph = UG::new();
//     undirected_graph.read_build(file_path)?;
    
//     undirected_graph.print_matrix();

//     undirected_graph.dfs();
//     undirected_graph.bfs_and_cc();
    
//     Ok(())
// }

fn main() {
    let mut dsu = dsu::DSU::new(7);

    dsu.union(0, 3);
    println!("{}", dsu.find(3));
    dsu.union(3, 4);
    println!("{}", dsu.find(3));
    dsu.print();
}
