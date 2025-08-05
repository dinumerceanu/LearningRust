pub fn qs(a: &mut[i32]) {
    if a.len() <= 1 {
        return;
    }

    if a.len() == 2 {
        if a[0] <= a[1] {
            return;
        } else {
            a.swap(0, 1);
        }
    }

    let pivot = a.len() / 2;

    a.swap(pivot, a.len() - 1);

    let mut i = 0 as usize;
    let mut j = a.len() - 2;
    let pivot_value = a[a.len() - 1];

    loop {
        loop {
            if i >= a.len() {
                break;
            }
            if a[i] < pivot_value {
                i += 1;
            } else {
                break;
            }
        }

        loop {
            if a[j] > pivot_value && j > 0 {
                j -= 1;
            } else if a[j] > pivot_value && j == 0 {
                break;
            } else {
                break;
            }
        }

        if i > j {
            a.swap(i, a.len() - 1);
            break;
        } else {
            a.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
            
        }
    }

    qs(&mut a[i..]);
    qs(&mut a[..i]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caz_normal() {
        let mut arr = [8, 2, 4, 9, 3, 6, 1, 5, 7];
        qs(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_vector_gol() {
        let mut arr: [i32; 0] = [];
        qs(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_un_singur_element() {
        let mut arr = [42];
        qs(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_doua_elemente_neordonate() {
        let mut arr = [7, 6];
        qs(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_doua_elemente_ordonate() {
        let mut arr = [6, 7];
        qs(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_vector_deja_sortat() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        qs(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vector_sortat_invers() {
        let mut arr = [6, 5, 4, 3, 2, 1];
        qs(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cu_elemente_duplicate() {
        let mut arr = [4, 3, 3, 2, 6];
        qs(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 6]);
    }
    
    #[test]
    fn test_cu_multiple_duplicate() {
        let mut arr = [5, 2, 8, 2, 5, 8, 2, 5];
        qs(&mut arr);
        assert_eq!(arr, [2, 2, 2, 5, 5, 5, 8, 8]);
    }

    #[test]
    fn test_toate_elementele_sunt_identice() {
        let mut arr = [5, 5, 5, 5, 5];
        qs(&mut arr);
        assert_eq!(arr, [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_cu_numere_negative() {
        let mut arr = [-5, 8, -10, 0, 3, -1];
        qs(&mut arr);
        assert_eq!(arr, [-10, -5, -1, 0, 3, 8]);
    }

    #[test]
    fn test_cu_pivot_la_margine() {
        let mut arr = [10, 1, 2, 3, 4, 5];
        qs(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 10]);
    }
}
