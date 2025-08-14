mod undirected_graph;
use undirected_graph::UG;
mod dsu;
use dsu::DSU;
mod edge;
use edge::Edge;

mod kruskal;

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
    let mut content = String::new();
    io::stdin().read_line(&mut content);

    let input = content.trim().split(' ').map(|s| s.parse::<i32>().expect("s nu e numar")).collect::<Vec<i32>>();
    let n = input[0] as usize;
    let m = input[1];
    let mut edges: Vec<Edge> = Vec::new();
    for _ in 0..m {
        let mut content = String::new();
        io::stdin().read_line(&mut content);
        let edge_arr = content.trim().split(' ').map(|s| s.parse::<i32>().expect("s nu e numar")).collect::<Vec<i32>>();
        let new_edge = Edge::new(edge_arr[0] as usize, edge_arr[1] as usize, edge_arr[2]);
        edges.push(new_edge);
    }

    println!("{:?}", kruskal::kruskal(&edges, &n));

}
