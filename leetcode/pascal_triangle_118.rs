use std::fs;

fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut a: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
    
    if num_rows < 1 {
        return a;
    }

    let start: usize = 0;
    let stop: usize = num_rows as usize;

    for i in start..stop {
        let mut row: Vec<i32> = Vec::new();
            
        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            } else {
                let sum = a[i - 1][j - 1] + a[i - 1][j];
                row.push(sum)
            }
        }
        a.push(row);
    }

    a
}

fn main() {
    let file_path = "file1.in";

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            if let Ok(num) = text.trim().parse::<i32>() {
                println!("{:?}", generate(num));
            } else {
                eprintln!("Nu ai introdus un numar");
            }
        },
        Err(e) => {
            eprintln!("Eroare la citire {}", e);
        },
    } 
}