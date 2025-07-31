use std::fs;

fn add_binary(a: String, b: String) -> String {
    let s: Vec<char> = a.chars().rev().collect();
    let t: Vec<char> = b.chars().rev().collect();
    
    println!("s: {:?}\nt: {:?}", s, t);

    let mut carry = 0;
    let mut i = 0;
    let mut j = 0;
    let mut res: Vec<char> = Vec::new();

    while i < s.len() && j < t.len() {
        match (s[i], t[j], carry) {
            ('0', '0', 0) => {
                res.push('0');
            },
            ('0', '0', 1) => {
                res.push('1');
                carry = 0;
            },
            ('0', '1', 0) => {
                res.push('1');
            },
            ('0', '1', 1) => {
                res.push('0');
                carry = 1;
            },
            ('1', '0', 0) => {
                res.push('1');
            },
            ('1', '0', 1) => {
                res.push('0');
                carry = 1;
            },
            ('1', '1', 0) => {
                res.push('0');
                carry = 1;
            },
            ('1', '1', 1) => {
                res.push('1');
                carry = 1;
            },
            (_, _, _) => break,
        }
        i += 1;
        j += 1;
    }

    while i < s.len() {
        match (s[i], carry) {
            ('1', 0) => res.push('1'),
            ('1', 1) => {
                res.push('0');
                carry = 1;
            },
            ('0', 0) => res.push('0'),
            ('0', 1) => {
                res.push('1');
                carry = 0;
            },
            (_, _) => break,
        }
        i += 1;
    }

    while j < t.len() {
        match (t[j], carry) {
            ('1', 0) => res.push('1'),
            ('1', 1) => {
                res.push('0');
                carry = 1;
            },
            ('0', 0) => res.push('0'),
            ('0', 1) => {
                res.push('1');
                carry = 0;
            },
            (_, _) => break,
        }
        j += 1;
    }

    if carry == 1 {
        res.push('1');
    }

    res.into_iter().rev().collect()
}

fn main() {
    let file_path = "file1.in";
    
    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            let str_nums: Vec<&str> = text.split(' ').collect::<Vec<_>>();
            println!("{:?}", add_binary(str_nums[0].to_string(), str_nums[1].to_string()));
        },
        Err(e) => {
            println!("Eroare: {}", e);
        },
    }

}
