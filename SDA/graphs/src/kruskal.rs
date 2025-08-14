use crate::dsu::DSU;
use crate::edge::{self, Edge};

pub fn kruskal(edges: &[Edge], n: &usize) -> Vec<Edge> {
    let mut sorted_edges = edges.to_vec();
    sorted_edges.sort();
    let mut dsu = DSU::new(*n);
    let mut min_span_tree: Vec<Edge> = Vec::new();

    for edge in sorted_edges {
        if dsu.union(edge.u, edge.v) {
            min_span_tree.push(
                Edge {
                    u: edge.u,
                    v: edge.v,
                    weight: edge.weight,
                }
            );
        }
    }

    min_span_tree
}
