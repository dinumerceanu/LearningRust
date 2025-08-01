use std::fs;

fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();

    let n = matrix.len();
    let m = matrix[0].len();

    for j in 0..m {
        let mut row: Vec<i32> = Vec::new();
        for i in 0..n {
            row.push(matrix[i][j]);
        }
        res.push(row);
    }
    
    res
}

fn main() {
    let file_path = "file1.in";

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            let matrix: Vec<Vec<i32>> = text
                .lines()
                .map(|line| line.split_whitespace().map(|c| c.parse().expect("BAD")).collect())
                .collect();
            println!("{:?}", transpose(matrix));
        },
        Err(e) => {
            println!("{}", e);
        },
    }
}