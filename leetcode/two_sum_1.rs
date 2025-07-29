use std::io;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();

    let mut value_index_pairs: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(index, value)| (value, index)).collect();
    value_index_pairs.sort_by_key(|&(value, _index)| value);


    let mut left: usize = 0;
    let mut right: usize = value_index_pairs.len() - 1;

    while left < right {
        if value_index_pairs[left].0 + value_index_pairs[right].0 == target {
            res.push(value_index_pairs[left].1 as i32);
            res.push(value_index_pairs[right].1 as i32);
            left = right;
        } else if value_index_pairs[left].0 + value_index_pairs[right].0 > target {
            right -= 1;
        } else {
            left += 1;
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Eroare la citire");
    
    let mut numbers: Vec<i32> = input.trim().split_whitespace().filter_map(|s| s.parse().ok()).collect();
    let target = numbers.pop().unwrap();

    println!("{:?} {}", numbers, target);

    let res = two_sum(numbers, target);
    println!("{:?}", res);
}
