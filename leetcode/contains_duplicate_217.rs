use std::io;
use std::collections::HashSet;

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for i in 0..nums.len() {
        if set.contains(&nums[i]) {
            return true;
        } else {
            set.insert(nums[i]);
        }
    }

    false
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    let nums: Vec<i32> = input.trim().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

    println!("{}", contains_duplicate(nums));
}
