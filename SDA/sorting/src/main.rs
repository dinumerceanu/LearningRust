mod merge_sort;
mod quick_sort;

fn main() {
    // let a = vec![100, 4, 67, 46, 91, 0, 2];
    // let res = merge_sort::merge_sort(&a);

    // println!("{:?}", res);

    let mut b = vec![2, 1];
    let res = quick_sort::qs(&mut b);

    println!("{:?}", b);

}
