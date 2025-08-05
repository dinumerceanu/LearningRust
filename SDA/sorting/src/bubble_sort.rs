use std::mem::swap;

pub fn bubble_sort(a: &mut [i32]) {
    for i in 0..a.len() {
        let mut swapped = false;

        for j in 0..a.len() - 1 - i {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caz_normal() {
        let mut arr = [8, 2, 4, 9, 3, 6, 1, 5, 7];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_vector_gol() {
        let mut arr: [i32; 0] = [];
        bubble_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_un_singur_element() {
        let mut arr = [42];
        bubble_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_doua_elemente_neordonate() {
        let mut arr = [7, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_doua_elemente_ordonate() {
        let mut arr = [6, 7];
        bubble_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_vector_deja_sortat() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vector_sortat_invers() {
        let mut arr = [6, 5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cu_elemente_duplicate() {
        let mut arr = [4, 3, 3, 2, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 6]);
    }
    
    #[test]
    fn test_cu_multiple_duplicate() {
        let mut arr = [5, 2, 8, 2, 5, 8, 2, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [2, 2, 2, 5, 5, 5, 8, 8]);
    }

    #[test]
    fn test_toate_elementele_sunt_identice() {
        let mut arr = [5, 5, 5, 5, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_cu_numere_negative() {
        let mut arr = [-5, 8, -10, 0, 3, -1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [-10, -5, -1, 0, 3, 8]);
    }

    #[test]
    fn test_cu_pivot_la_margine() {
        let mut arr = [10, 1, 2, 3, 4, 5];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 10]);
    }
}
