use std::cmp::Reverse;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use crate::edge::Edge;

pub fn prim(graph: &[Vec<Edge>], n: usize, m: usize) -> Vec<Edge> {
    let mut priority_queue: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let mut min_span_tree: Vec<Edge> = Vec::new();

    let start = 0;
    visited.insert(start as usize);
    for edge in &graph[start] {
        priority_queue.push(Reverse(edge.clone()));
    }

    loop {
        if let Some(Reverse(edge)) = priority_queue.pop() {
            if visited.insert(edge.v) {
                min_span_tree.push(edge.clone());
                for other_edge in &graph[edge.v] {
                    if !visited.contains(&other_edge.v) {
                        priority_queue.push(Reverse(other_edge.clone()));
                    }
                }
            }
        } else {
            break;
        }
    }

    min_span_tree
}
