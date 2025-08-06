use std::{io, collections::HashMap};

fn is_isomorphic(s: String, t: String) -> bool {
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();
    
    if s_chars.len() != t_chars.len() {
        return false;
    }

    let mut map_s_to_t: HashMap<char, char> = HashMap::new();
    let mut map_t_to_s: HashMap<char, char> = HashMap::new();

    for i in 0..s_chars.len() {
        let char_s = s_chars[i];
        let char_t = t_chars[i];

        if let Some(&mapped_t) = map_s_to_t.get(&char_s) {
            if mapped_t != char_t {
                return false;
            }
        } else {
            map_s_to_t.insert(char_s, char_t);
        }

        if let Some(&mapped_s) = map_t_to_s.get(&char_t) {
            if mapped_s != char_s {
                return false;
            }
        } else {
            map_t_to_s.insert(char_t, char_s);
        }
    }

    true
}

fn main() {
    let mut s = String::new();
    let mut t = String::new();

    io::stdin().read_line(&mut s).expect("Eroare la citirea lui S");
    io::stdin().read_line(&mut t).expect("Eroare la citirea lui T");

    let _ = s.trim();
    let _ = t.trim();

    println!("{}", is_isomorphic(s, t));
}
