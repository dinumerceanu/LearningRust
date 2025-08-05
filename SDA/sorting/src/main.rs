mod merge_sort;

fn main() {
    let a = vec![100, 4, 67, 46, 91, 0, 2];
    let res = merge_sort::merge_sort(&a);

    println!("{:?}", res);

}
