use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod undirected_graph;
use undirected_graph::UG;
mod directed_graph;
use directed_graph::*;
mod dsu;
use dsu::DSU;
mod edge;
use edge::Edge;

mod kruskal;
mod prim;

// fn main() -> io::Result<()> {
//     let file_path = "src/file.in";

//     let mut undirected_graph = UG::new();
//     undirected_graph.read_build(file_path)?;
    
//     undirected_graph.print_matrix();

//     undirected_graph.dfs();
//     undirected_graph.bfs_and_cc();
    
//     Ok(())
// }

// fn main() {
//     let mut content = String::new();
//     io::stdin().read_line(&mut content);

//     let input = content.trim().split(' ').map(|s| s.parse::<i32>().expect("s nu e numar")).collect::<Vec<i32>>();
//     let n = input[0] as usize;
//     let m = input[1];
//     let mut edges: Vec<Edge> = Vec::new();
//     for _ in 0..m {
//         let mut content = String::new();
//         io::stdin().read_line(&mut content);
//         let edge_arr = content.trim().split(' ').map(|s| s.parse::<i32>().expect("s nu e numar")).collect::<Vec<i32>>();
//         let new_edge = Edge::new(edge_arr[0] as usize, edge_arr[1] as usize, edge_arr[2]);
//         edges.push(new_edge);
//     }

//     println!("{:?}", kruskal::kruskal(&edges, &n));

// }

// fn main() -> io::Result<()> {
//     let file = File::open("src/file.in")?;
//     let mut reader = BufReader::new(file);

//     let mut first_line = String::new();
//     reader.read_line(&mut first_line)?;
//     let input: Vec<i32> = first_line
//         .trim()
//         .split_whitespace()
//         .map(|s| s.parse::<i32>().expect("Nu e numar"))
//         .collect();

//     let n = input[0] as usize;
//     let m = input[1] as usize;

//     let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n];

//     for _ in 0..m {
//         let mut line = String::new();
//         reader.read_line(&mut line)?;
//         let edge_arr: Vec<i32> = line
//             .trim()
//             .split_whitespace()
//             .map(|s| s.parse::<i32>().expect("Nu e numar"))
//             .collect();

//         let new_edge1 = Edge::new(edge_arr[0] as usize, edge_arr[1] as usize, edge_arr[2]);
//         let new_edge2 = Edge::new(edge_arr[1] as usize, edge_arr[0] as usize, edge_arr[2]);

//         graph[edge_arr[0] as usize].push(new_edge1);
//         graph[edge_arr[1] as usize].push(new_edge2);
//     }

//     println!("{:?}", prim::prim(&graph, n, m));

//     Ok(())
// }
fn main() -> io::Result<()> {
    let file = File::open("src/file.in")?;
    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let input: Vec<i32> = first_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Nu e numar"))
        .collect();

    let n = input[0] as usize;
    let m = input[1] as usize;

    let mut graph: Vec<Vec<Edge>> = vec![Vec::new(); n];

    for _ in 0..m {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let edge_arr: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Nu e numar"))
            .collect();

        let new_edge = Edge::new(edge_arr[0] as usize, edge_arr[1] as usize, edge_arr[2]);

        graph[edge_arr[0] as usize].push(new_edge);
    }

    // bfs_dg(&graph, 0);
    // dfs_dg(&graph, 0);
    // println!("{:?}", topsort_dfs(&graph));
    // println!("{:?}", topsort_khan_bfs(&graph));
    // println!("{:?}", tarjan_scc(&graph));
    // println!("{:?}", dijkstra(&graph, 0));
    // println!("{:?}", bellman_ford(&graph, 0));
    println!("{:?}", roy_floyd_warshall(&graph));

    Ok(())
}
