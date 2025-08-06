fn bucket_insertion_sort(a: &mut [f32]) {
    if a.len() <= 1 {
        return;
    }

    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            let mut current = a[i];
            let mut j = i;
            while j > 0 && current < a[j - 1] {
                a[j] = a[j - 1];
                j -= 1;
            }
            a[j] = current;
        }
    }
}

pub fn bucket_sort(a: &mut [f32]) {
    let mut buckets: Vec<Vec<f32>> = Vec::new();
    let mut res: Vec<f32> = Vec::new();
    
    for i in 0..10 {
        let bucket: Vec<f32> = Vec::new();
        buckets.push(bucket);
    }

    for i in 0..a.len() {
        let idx = ((a[i] * 10.0) as usize).min(9);
        buckets[idx].push(a[i]);
    }

    for i in 0..10 {
        bucket_insertion_sort(&mut buckets[i]);
        for j in 0..buckets[i].len() {
            res.push(buckets[i][j]);
        }
    }

    for i in 0..a.len() {
        a[i] = res[i];
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let mut arr: [f32; 0] = [];
        bucket_sort(&mut arr);
        let expected: [f32; 0] = [];
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_single_element() {
        let mut arr = [0.5];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.5]);
    }

    #[test]
    fn test_two_elements_sorted() {
        let mut arr = [0.1, 0.9];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.1, 0.9]);
    }
    
    #[test]
    fn test_two_elements_unsorted() {
        let mut arr = [0.9, 0.1];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.1, 0.9]);
    }

    #[test]
    fn test_uniform_distribution() {
        let mut arr = [0.8, 0.2, 0.5, 0.1, 0.9, 0.3];
        bucket_sort(&mut arr);
        let expected = [0.1, 0.2, 0.3, 0.5, 0.8, 0.9];
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_clustered_elements() {
        let mut arr = [0.1, 0.15, 0.12, 0.9, 0.95, 0.92];
        bucket_sort(&mut arr);
        let expected = [0.1, 0.12, 0.15, 0.9, 0.92, 0.95];
        assert_eq!(arr, expected);
    }
    
    #[test]
    fn test_all_elements_in_one_bucket() {
        let mut arr = [0.01, 0.05, 0.02, 0.08, 0.03];
        bucket_sort(&mut arr);
        let expected = [0.01, 0.02, 0.03, 0.05, 0.08];
        assert_eq!(arr, expected);
    }

    #[test]
    fn test_with_zero_and_one() {
        let mut arr = [0.5, 0.0, 1.0];
        bucket_sort(&mut arr);
        assert_eq!(arr, [0.0, 0.5, 1.0]);
    }

    #[test]
    fn test_with_duplicates() {
        let mut arr = [0.5, 0.1, 0.9, 0.5, 0.1];
        bucket_sort(&mut arr);
        let expected = [0.1, 0.1, 0.5, 0.5, 0.9];
        assert_eq!(arr, expected);
    }
}
