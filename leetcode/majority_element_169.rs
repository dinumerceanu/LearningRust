use std::io;
use std::collections::HashMap;

fn majority_element(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut map = HashMap::new();

    if n == 1 {
        return nums[0];
    }

    for i in 0..n {
        if let Some(freq) = map.get_mut(&nums[i]) {
            *freq += 1;
            if *freq > n / 2 {
                return nums[i];
            }
        } else {
            map.insert(nums[i], 1);
        }
    }

    -1
}

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Eroare la citire");

    let nums = input.trim().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

    println!("{}", majority_element(nums));
}
