fn valid_mountain_array(arr: Vec<i32>) -> bool {
    let len = arr.len() as usize;

    if len < 3 {
        return false;
    }

    let mut i = 0;

    while i < len - 1 && arr[i] < arr[i + 1] {
        i += 1;
    }
    if i == 0 || i == len - 1 {
        return false;
    }

    while i < len - 1 && arr[i] > arr[i + 1] {
        i += 1;
    }
    if i == len - 1 {
        return true;
    }
    return false;
}

fn main() {
    println!("{}", valid_mountain_array(vec![0, 2, 3, 3, 5, 2, 1, 0]));
}
