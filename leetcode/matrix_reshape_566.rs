use std::fs;
use std::io;

fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut n = mat.len();

    if n == 0 {
        return mat;
    }

    let mut m = mat[0].len();

    if m * n != (r * c) as usize {
        return mat;
    }

    let mut flat: Vec<i32> = Vec::new();
    for i in 0..n {
        for j in 0..m {
            flat.push(mat[i][j]);
        }
    }

    for i in 0..r as usize {
        let mut row: Vec<i32> = Vec::new();
        for j in 0..c as usize {
            row.push(flat[i * (c as usize) + j]);
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
            let transformed: Vec<Vec<i32>> = text
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect();

            println!("r: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let r: i32 = input.trim().parse().expect("WORK");

            println!("c: ");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let c: i32 = input.trim().parse().expect("WORK");
            

            println!("{:?}", matrix_reshape(transformed, r, c));
        },
        Err(e) => {
            println!("Eroare: {}", e);
        },
    }

}