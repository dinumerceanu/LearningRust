use std::io;

fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = Vec::new();
    let mut N = n;
    let mut i = 2;

    while i * i <= N {
        while N % i == 0 {
            res.push(i);
            N = N / i;
        }
        i += 1;
    }

    if N > 1 {
        res.push(N);
    }

    return res;
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    if let Ok(num) = input.trim().parse::<u64>() {
        println!("{:?}", factors(num));
    } else {
        println!("Nu ai introdus un numar");
    }
}
