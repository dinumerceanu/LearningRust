use std::{cmp::Reverse, collections::{BinaryHeap, HashSet, VecDeque}, usize, vec};

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

fn dfs_tarjan(
    graph: &[Vec<Edge>],
    at: usize,
    stack: &mut Vec<usize>,
    on_stack: &mut Vec<bool>,
    ids: &mut Vec<isize>,
    low: &mut Vec<usize>,
    id: &mut usize,
    scc_count: &mut usize,
) {
    stack.push(at);
    on_stack[at] = true;
    ids[at] = *id as isize;
    low[at] = *id;
    *id += 1;

    for edge in &graph[at] {
        let to = edge.v;
        if ids[to] == -1 {
            dfs_tarjan(graph, to, stack, on_stack, ids, low, id, scc_count);
            low[at] = low[at].min(low[to]);
        } else if on_stack[to] {
            low[at] = low[at].min(ids[to] as usize);
        }
    }

    if low[at] == ids[at] as usize {
        loop {
            let node = stack.pop().unwrap();
            on_stack[node] = false;
            low[node] = ids[at] as usize;
            if node == at {
                break;
            }
        }
        *scc_count += 1;
    }
}

pub fn tarjan_scc(graph: &[Vec<Edge>]) -> usize {
    let n = graph.len();
    let mut id = 0;
    let mut scc_count = 0;
    let mut ids = vec![-1; n];
    let mut low = vec![0; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();

    for i in 0..n {
        if ids[i] == -1 {
            dfs_tarjan(
                graph,
                i,
                &mut stack,
                &mut on_stack,
                &mut ids,
                &mut low,
                &mut id,
                &mut scc_count,
            );
        }
    }

    scc_count
}

pub fn dijkstra(graph: &[Vec<Edge>], start: usize) -> (Vec<usize>, Vec<usize>) {
    let n = graph.len();
    let mut dist= vec![usize::MAX; n];
    let mut visited = vec![false; n];
    let mut parents: Vec<usize> = (0..n).collect();
    dist[start] = 0;
    let mut priority_queue: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    priority_queue.push(Reverse((0, start)));

    while let Some(Reverse((distance, index))) = priority_queue.pop() {
        if visited[index] {
            continue;
        }

        visited[index] = true;

        for edge in &graph[index] {
            if (distance + edge.weight as usize) < dist[edge.v] {
                dist[edge.v] = distance + edge.weight as usize;
                priority_queue.push(Reverse((dist[edge.v], edge.v)));
                parents[edge.v] = index;
            }
        }
    }

    (dist, parents)
}
