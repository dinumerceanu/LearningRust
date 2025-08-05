pub fn merge(a: &Vec<i32>, b: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    let mut i = 0 as usize;
    let mut j = 0 as usize;
    
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            res.push(a[i]);
            i += 1;
        } else if a[i] > b[j] {
            res.push(b[j]);
            j += 1;
        } else {
            res.push(a[i]);
            res.push(b[j]);
            i += 1;
            j += 1;
        }
    }

    while i < a.len() {
        res.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        res.push(b[j]);
        j += 1;
    }

    res
}

pub fn merge_sort(a: &[i32]) -> Vec<i32> {
    if a.len() <= 1 {
        return a.to_vec();
    }

    let m = a.len() / 2;
    let left = merge_sort(&a[..m]); 
    let right =  merge_sort(&a[m..]);

    merge(&left, &right)
}
