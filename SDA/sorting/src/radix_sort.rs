use crate::counting_sort;

fn digits(num: i32) -> usize {
    let s = num.to_string();

    if num < 0 {
        s.len() - 1
    } else {
        s.len()
    }
}

fn radix_counting_sort(a: &mut [i32], exp: i32) {
    let mut count_arr = vec![0; 10];
    let n = a.len();
    let mut res = vec![0; n];

    for i in 0..n {
        let digit = (a[i] / exp) % 10;
        count_arr[digit as usize] += 1;
    }

    for i in 1..10 {
        count_arr[i] += count_arr[i - 1];
    }

    for i in (0..n).rev() {
        let digit = (a[i] / exp) % 10;
        let pos = count_arr[digit as usize] - 1;
        res[pos as usize] = a[i];
        count_arr[digit as usize] -= 1;
    }

    for i in 0..n {
        a[i] = res[i];
    }
}

pub fn radix_sort(a: &mut [i32]) {
    let mut max_value = 0;

    for i in 0..a.len() {
        if a[i] > max_value {
            max_value = a[i];
        }
    }

    let iterations = digits(max_value);

    let mut exp = 1;
    for i in 0..iterations {
        radix_counting_sort(a, exp);
        exp *= 10;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caz_normal() {
        let mut arr = [8, 2, 4, 9, 3, 6, 1, 5, 7];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_vector_gol() {
        let mut arr: [i32; 0] = [];
        radix_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_un_singur_element() {
        let mut arr = [42];
        radix_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_doua_elemente_neordonate() {
        let mut arr = [7, 6];
        radix_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_doua_elemente_ordonate() {
        let mut arr = [6, 7];
        radix_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_vector_deja_sortat() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vector_sortat_invers() {
        let mut arr = [6, 5, 4, 3, 2, 1];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cu_elemente_duplicate() {
        let mut arr = [4, 3, 3, 2, 6];
        radix_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 6]);
    }
    
    #[test]
    fn test_cu_multiple_duplicate() {
        let mut arr = [5, 2, 8, 2, 5, 8, 2, 5];
        radix_sort(&mut arr);
        assert_eq!(arr, [2, 2, 2, 5, 5, 5, 8, 8]);
    }

    #[test]
    fn test_toate_elementele_sunt_identice() {
        let mut arr = [5, 5, 5, 5, 5];
        radix_sort(&mut arr);
        assert_eq!(arr, [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_cu_pivot_la_margine() {
        let mut arr = [10, 1, 2, 3, 4, 5];
        radix_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 10]);
    }
}
