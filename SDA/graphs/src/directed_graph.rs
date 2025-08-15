use std::collections::{HashSet, VecDeque};

use crate::edge::Edge;

pub fn bfs_dg(graph: &[Vec<Edge>], start: usize) {
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut visited: HashSet<usize> = HashSet::new();

    queue.push_back(start);
    visited.insert(start);
    
    loop {
        if let Some(current) = queue.pop_front() {
            print!("{} ", current);
            for edge in &graph[current] {
                if visited.insert(edge.v) {
                    queue.push_back(edge.v);
                }
            }
        } else {
            break;
        }
    }
    println!();
}

pub fn dfs_dg(graph: &[Vec<Edge>], start: usize) {
    let mut visited: HashSet<usize> = HashSet::new();
    dfs_dg_aux(graph, start, &mut visited);
    println!();
}

fn dfs_dg_aux(graph: &[Vec<Edge>], start: usize, visited: &mut HashSet<usize>) {
    print!("{} ", start);
    visited.insert(start);

    for edge in &graph[start] {
        if !visited.contains(&edge.v) {
            dfs_dg_aux(graph, edge.v, visited);
        }
    }
}

fn dfs_topsort(graph: &[Vec<Edge>], i: usize, visited: &mut Vec<bool>, visited_nodes_dfs: &mut Vec<usize>) {
    visited[i] = true;

    for edge in &graph[i] {
        if !visited[edge.v] {
            dfs_topsort(graph, edge.v, visited, visited_nodes_dfs);
        }
    }

    visited_nodes_dfs.push(i);
}

pub fn topsort_dfs(graph: &[Vec<Edge>]) -> VecDeque<usize> {
    let n = graph.len();
    let mut visited = vec![false; n];
    let mut res: VecDeque<usize> = VecDeque::new();

    for i in 0..n {
        if !visited[i] {
            let mut visited_nodes_dfs: Vec<usize> = Vec::new();
            dfs_topsort(graph, i, &mut visited, &mut visited_nodes_dfs);
            for node in visited_nodes_dfs {
                res.push_front(node);
            }
        }
    }

    res
}

pub fn topsort_khan_bfs(graph: &[Vec<Edge>]) -> Option<Vec<usize>> {
    let n = graph.len();
    let mut internal_degrees = vec![0; n];
    let mut queue: VecDeque<usize> = VecDeque::new();
    let mut res: Vec<usize> = Vec::new();

    for i in 0..n {
        for j in 0..graph[i].len() {
            internal_degrees[graph[i][j].v] += 1;
        }
    }

    for i in 0..n {
        if internal_degrees[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(node) = queue.pop_front() {
        res.push(node);
        if graph[node].len() > 0 {
            for i in 0..graph[node].len() {
                internal_degrees[graph[node][i].v] -= 1;
                if internal_degrees[graph[node][i].v] == 0 {
                    queue.push_back(graph[node][i].v);
                }
            }
        }
    }

    if res.len() != n {
        return None;
    }

    Some(res)
}
