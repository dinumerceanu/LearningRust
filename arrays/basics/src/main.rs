fn print_array(v: &Vec<i32>) {
    println!("{:?}", v);
}

fn double_each_element(v: &mut Vec<i32>) {
    for elem in v.iter_mut() {
        *elem = *elem * 2;
    }
}

fn triple_each_element(v: &mut Vec<i32>) {
    for i in 0..v.len() {
        v[i] = v[i] * 3;
    }
}

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    print_array(&v2);
    print_array(&v2);

    v.push(1);
    v.push(2);
    v.push(3);
    double_each_element(&mut v);
    print_array(&v);
    triple_each_element(&mut v);
    print_array(&v);

    let mut zece_zero = vec![0; 10];
    print_array(&zece_zero);
    zece_zero.pop();
    print_array(&zece_zero);

    match v.get(0) {
        Some(x) => if *x > 0 {println!("pozitiv");} else {println!("negativ");},
        None => println!("doesn't exist"),
    }

}
