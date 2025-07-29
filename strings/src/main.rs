fn print_text(s: &str) {
    println!("{}", s);
}

fn to_upper_case(s: &str) -> String {
    let mut res = String::new();
    for letter in s.chars() {
        res.push(letter.to_ascii_uppercase());
    }
    res
}

fn main() {
    let mut s = String::new();
    s.push('a');
    println!("{}", s);
    s.push_str("bam bam");
    println!("{}", s);

    let s = String::from("Here we go again!");
    println!("{}", s);
    
    let s1 = String::from('2');
    let s2 = 1000000;
    let s3 = format!("Am terminat anul {} cu media {}", s1, s2);
    println!("{}", s3);

    let salut = "salut".to_string();
    for c in salut.chars() {
        println!("{}", c);
    }

    let slice = &salut[1..3];
    println!("{}", slice);

    let s = "string literal";
    let s2 = String::from("String normal");
    print_text(s);
    print_text(&s2);

    let sir1 = "AbcD";
    let mut sir1_upper = to_upper_case(sir1);
    print_text(sir1);
    print_text(&sir1_upper);
}
