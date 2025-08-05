mod merge_sort;
mod quick_sort;
mod bubble_sort;
mod insertion_sort;

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

    let mut d = vec![8, 2, 4, 9, 3, 6, 1, 5, 7];
    insertion_sort::insertion_sort(&mut d);

    println!("{:?}", d);

}
