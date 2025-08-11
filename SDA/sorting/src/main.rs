mod merge_sort;
mod quick_sort;
mod heap_sort;

mod bubble_sort;
mod insertion_sort;
mod selection_sort;

mod counting_sort;
mod radix_sort;
mod bucket_sort;

fn main() {
    // let a = vec![100, 4, 67, 46, 91, 0, 2];
    // let res = merge_sort::merge_sort(&a);

    // println!("{:?}", res);

    // let mut b = vec![2, 1];
    // quick_sort::qs(&mut b);

    // println!("{:?}", b);

    // let mut c = vec![8, 2, 4, 9, 3, 6, 1, 5, 7];
    // bubble_sort::bubble_sort(&mut c);

    // println!("{:?}", c);

    // let mut d = vec![8, 2, 4, 9, 3, 6, 1, 5, 7];
    // insertion_sort::insertion_sort(&mut d);

    // println!("{:?}", d);

    // let mut e = vec![8, 2, 4, 9, 3, 6, 1, 5, 7];
    // selection_sort::selection_sort(&mut e);

    // println!("{:?}", e);

    // let mut f = vec![5, 2, 1, 4, 0, 1, 3, 2, 2];
    // counting_sort::counting_sort(&mut f);

    // println!("{:?}", f);

    // let mut g = vec![170, 45, 75, 90, 802, 24, 2, 66];
    // radix_sort::radix_sort(&mut g);

    // println!("{:?}", g);

    // let mut h = vec![170, 45, 75, 90, 802, 24, 2, 66];
    // radix_sort::radix_sort(&mut h);

    // println!("{:?}", h);

    // let mut i = vec![0.5, 0.0, 1.0];
    // bucket_sort::bucket_sort(&mut i);

    // println!("{:?}", i);

    let mut j = vec![2, 8, 5, 3, 9, 1];
    heap_sort::heap_sort(&mut j);

    println!("{:?}", j);

}
