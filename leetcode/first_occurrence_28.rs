use std::fs;
use std::io::{self, Read};

fn str_str(haystack: String, needle: String) -> i32 {
    if let Some(index) = haystack.find(&needle) {
        return index as i32;
    } else {
        return -1;
    }
}

fn main() {
    let file_path = "file1.in";

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            let input = text.split(' ').collect::<Vec<_>>();
            println!("{}", str_str(input[0].to_string(), input[1].to_string()));
        },
        Err(e) => {
            eprintln!("Eroare la citirea fișierului '{}': {}", file_path, e);
        } 
    }

}
