use std::cmp;pub struct Heap {
    arr: Vec<i32>
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            arr: Vec::new()
        }
    }

    pub fn parent(&self, index: usize) -> usize {
        if index > 0 {
            (index - 1) / 2
        } else {
            index
        }
    }

    pub fn left_child(&self, index: usize) -> usize {
        2 * index + 1
    }

    pub fn right_child(&self, index: usize) -> usize {
        2 * index + 2
    }

    pub fn insert(&mut self, val: i32) {
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

    pub fn delete(&mut self) {
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


    pub fn print_heap(&self) {
        for i in 0..self.arr.len() {
            print!("{} ", self.arr[i]);
        }
        println!();
    }

    pub fn is_empty(&self) -> bool {
        self.arr.is_empty()
    }

    pub fn peek(&self) -> Option<&i32> {
        self.arr.first()
    }

    pub fn build_heap(&mut self, arr: Vec<i32>) {
        self.arr = arr;

        if self.arr.is_empty() {
            return;
        }

        let last_parent = self.arr.len() / 2;

        for i in (0..=last_parent).rev() {
            self.heapify_down(i);
        }
    }

    pub fn drop_heap(&mut self) {
        self.arr = Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_and_children_indices() {
        let heap = Heap::new();
        assert_eq!(heap.parent(1), 0);
        assert_eq!(heap.parent(2), 0);
        assert_eq!(heap.parent(3), 1);
        assert_eq!(heap.left_child(0), 1);
        assert_eq!(heap.right_child(0), 2);
        assert_eq!(heap.left_child(1), 3);
        assert_eq!(heap.right_child(1), 4);
    }

    #[test]
    fn test_insert_single_element() {
        let mut heap = Heap::new();
        heap.insert(42);
        assert_eq!(heap.peek(), Some(&42));
        assert_eq!(heap.arr, vec![42]);
    }

    #[test]
    fn test_insert_multiple_elements_heapify_up() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(20);
        heap.insert(5);
        heap.insert(30);

        assert_eq!(heap.peek(), Some(&30));
        assert_eq!(heap.arr, vec![30, 20, 5, 10]);
    }

    #[test]
    fn test_delete_from_empty_heap() {
        let mut heap = Heap::new();
        heap.delete();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_delete_single_element() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.delete();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_delete_multiple_elements_in_order() {
        let mut heap = Heap::new();
        heap.insert(10);
        heap.insert(50);
        heap.insert(20);
        heap.insert(5);
        heap.insert(100);

        assert_eq!(heap.peek(), Some(&100));
        heap.delete();
        assert_eq!(heap.peek(), Some(&50));
        heap.delete();
        assert_eq!(heap.peek(), Some(&20));
        heap.delete();
        assert_eq!(heap.peek(), Some(&10));
        heap.delete();
        assert_eq!(heap.peek(), Some(&5));
        heap.delete();
        assert!(heap.is_empty());
    }

    #[test]
    fn test_heapify_down_logic() {
        let mut heap = Heap::new();
        heap.arr = vec![5, 10, 20, 1, 2]; 
        heap.delete();
        assert_eq!(heap.peek(), Some(&20));
        assert_eq!(heap.arr, vec![20, 10, 2, 1]);
    }
}
