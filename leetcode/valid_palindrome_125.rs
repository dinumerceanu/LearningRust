use std::fs;

fn is_palindrome(s: String) -> bool {
    let mut clean_text: Vec<char> = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lowered_clean_text: Vec<char> = clean_text.into_iter().map(|c| c.to_ascii_lowercase()).collect();

    if lowered_clean_text.is_empty() || lowered_clean_text.len() == 1 {
        return true;
    }

    let mut left = 0;
    let mut right = lowered_clean_text.len() - 1;

    while left < right {
        if lowered_clean_text[left] != lowered_clean_text[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    return true;
}

fn main() {
    let file_path = "file1.in";

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            println!("{}", text);
            println!("{}", is_palindrome(text));
        },
        Err(e) => {
            println!("{}", e);
        }
    }
}