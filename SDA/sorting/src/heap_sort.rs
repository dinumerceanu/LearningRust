struct Heap {
    arr: Vec<i32>
}

impl Heap {
    fn new() -> Self {
        Heap {
            arr: Vec::new()
        }
    }

    fn parent(&self, index: usize) -> usize {
        if index > 0 {
            (index - 1) / 2
        } else {
            index
        }
    }

    fn left_child(&self, index: usize) -> usize {
        2 * index + 1
    }

    fn right_child(&self, index: usize) -> usize {
        2 * index + 2
    }

    fn insert(&mut self, val: i32) {
        self.arr.push(val);

        self.heapify_up(self.arr.len() - 1);
    }

    fn heapify_up(&mut self, mut index: usize) {
        loop {
            let parent_idx = self.parent(index);

            if index == 0 || self.arr[index] < self.arr[parent_idx] {
                break;
            } else {
                self.arr.swap(index, parent_idx);
                index = parent_idx;
            }
        }
    }

    fn delete(&mut self) {
        if self.arr.is_empty() {
            return;
        }

        let last = self.arr.len() - 1;

        self.arr.swap(0, last);
        self.arr.pop();

        if !self.arr.is_empty() {
            self.heapify_down(0);
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        let mut current_idx = idx;

        loop {
            let left_child_idx = self.left_child(current_idx);
            let right_child_idx = self.right_child(current_idx);

            let mut largest = current_idx;

            if left_child_idx < self.arr.len() && self.arr[left_child_idx] > self.arr[largest] {
                largest = left_child_idx;
            }
            if right_child_idx < self.arr.len() && self.arr[right_child_idx] > self.arr[largest] {
                largest = right_child_idx;
            }

            if largest != current_idx {
                self.arr.swap(current_idx, largest);
                current_idx = largest;
            } else {
                break;
            }
        }
    }


    fn print_heap(&self) {
        for i in 0..self.arr.len() {
            print!("{} ", self.arr[i]);
        }
        println!();
    }

    fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    fn peek(&self) -> Option<&i32> {
        self.arr.first()
    }

    fn build_heap(&mut self, arr: Vec<i32>) {
        self.arr = arr;

        if self.arr.is_empty() {
            return;
        }

        let last_parent = self.arr.len() / 2;

        for i in (0..=last_parent).rev() {
            self.heapify_down(i);
        }
    }

    fn drop_heap(&mut self) {
        self.arr = Vec::new();
    }
}

pub fn heap_sort(a: &mut [i32]) {
    let mut heap = Heap::new();
    heap.build_heap(a.to_vec());

    for i in (0..a.len()).rev() {
        a[i] = heap.peek().copied().unwrap();
        heap.delete();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caz_normal() {
        let mut arr = [8, 2, 4, 9, 3, 6, 1, 5, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_vector_gol() {
        let mut arr: [i32; 0] = [];
        heap_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn test_un_singur_element() {
        let mut arr = [42];
        heap_sort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn test_doua_elemente_neordonate() {
        let mut arr = [7, 6];
        heap_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_doua_elemente_ordonate() {
        let mut arr = [6, 7];
        heap_sort(&mut arr);
        assert_eq!(arr, [6, 7]);
    }

    #[test]
    fn test_vector_deja_sortat() {
        let mut arr = [1, 2, 3, 4, 5, 6];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_vector_sortat_invers() {
        let mut arr = [6, 5, 4, 3, 2, 1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_cu_elemente_duplicate() {
        let mut arr = [4, 3, 3, 2, 6];
        heap_sort(&mut arr);
        assert_eq!(arr, [2, 3, 3, 4, 6]);
    }
    
    #[test]
    fn test_cu_multiple_duplicate() {
        let mut arr = [5, 2, 8, 2, 5, 8, 2, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, [2, 2, 2, 5, 5, 5, 8, 8]);
    }

    #[test]
    fn test_toate_elementele_sunt_identice() {
        let mut arr = [5, 5, 5, 5, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, [5, 5, 5, 5, 5]);
    }

    #[test]
    fn test_cu_numere_negative() {
        let mut arr = [-5, 8, -10, 0, 3, -1];
        heap_sort(&mut arr);
        assert_eq!(arr, [-10, -5, -1, 0, 3, 8]);
    }

    #[test]
    fn test_cu_pivot_la_margine() {
        let mut arr = [10, 1, 2, 3, 4, 5];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 10]);
    }
}
