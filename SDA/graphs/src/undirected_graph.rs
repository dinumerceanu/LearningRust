use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub struct UG {
    adj_matrix: Vec<Vec<i32>>
}

impl UG {
    pub fn new() -> Self {
        UG {
            adj_matrix: Vec::new()
        }
    }

    pub fn print_matrix(&self) {
        for i in 0..self.adj_matrix.len() {
            println!("{:?}", self.adj_matrix[i]);
        }
    }

    pub fn read_build(&mut self, file_path: &str) -> io::Result<()> {
        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        let mut first_line = String::new();
        let bytes_read = reader.read_line(&mut first_line)?;

        let n: u32;
        let m: u32;

        if bytes_read > 0 {
            let input: Vec<u32> = first_line.trim().split(' ').map(|s| s.parse::<u32>().expect("s nu e numar")).collect::<Vec<u32>>();
            println!("{:?}", input);
            n = input[0];
            m = input[1];
            println!("n: {} m: {}", n, m);
        } else {
            println!("Fișierul este gol.");
            return Ok(());
        }
        
        let mut adj_matrix: Vec<Vec<i32>> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            let row = vec![0; n as usize];
            adj_matrix.push(row);
        }
        
        for linie in reader.lines() {
            let good_line = linie?;
            let edge: Vec<usize> = good_line.trim().split(' ').map(|s| s.parse::<usize>().expect("s nu e numar")).collect::<Vec<usize>>();
            adj_matrix[edge[0]][edge[1]] = 1;
            adj_matrix[edge[1]][edge[0]] = 1;
        }

        self.adj_matrix = adj_matrix;


        Ok(())
    }

    pub fn dfs(&self) {
        let mut visited = vec![false; self.adj_matrix.len()];
        let mut all_done = false;

        loop {
            if all_done {
                break;
            }

            all_done = true;

            for i in 0..self.adj_matrix.len() {
                if !visited[i] {
                    all_done = false;
                    self.dfs_aux(i, &mut visited);
                    println!();
                }
            }
        }

    }

    fn dfs_aux(&self, start: usize, visited: &mut Vec<bool>) {
        if !visited[start] {
            print!("{} ", start);
            visited[start] = true;
            for j in 0..self.adj_matrix[start].len() {
                if self.adj_matrix[start][j] == 1 {
                    self.dfs_aux(j, visited);
                }
            }
        }
    }

    pub fn bfs_and_cc(&self) {
        let mut visited = vec![false; self.adj_matrix.len()];
        let mut all_done = false;
        let mut cc = 0;

        loop {
            if all_done {
                break;
            }

            all_done = true;

            for i in 0..self.adj_matrix.len() {
                if !visited[i] {
                    all_done = false;
                    self.bfs_aux(i, &mut visited);
                    cc += 1;
                    println!();
                }
            }
        }
        println!("CC: {}", cc);
    }

    fn bfs_aux(&self, start: usize, visited: &mut Vec<bool>) {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(start);
        visited[start] = true;

        while !queue.is_empty() {
            let mut node_opt = queue.pop_front();
            let node = node_opt.unwrap();

            print!("{} ", node);

            for i in 0..self.adj_matrix.len() {
                if self.adj_matrix[node][i] == 1 && !visited[i] {
                    queue.push_back(i);
                    visited[i] = true;
                }
            }

        }

    }
}
