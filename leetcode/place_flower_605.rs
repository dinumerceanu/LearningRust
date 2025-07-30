use std::fs;

fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
    let len = flowerbed.len() as usize;

    if len == 0 {
        return false;
    }

    if len == 1 && flowerbed[0] == 0 {
        return true;
    }
    
    for i in 0..len {
        if n == 0 {
            return true;
        }
        if flowerbed[i] == 0 {
            if i > 0 && i < len - 1 {
                if flowerbed[i - 1] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            } else {
                if i == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
                if i == len - 1 && flowerbed[i - 1] == 0 {
                    flowerbed[i] = 1;
                    n -= 1;
                }
            }
        }
    }

    if n == 0 {
        return true;
    }
    return false;
}

fn main() {
    let file_path = "file1.in";

    let contents = fs::read_to_string(file_path);

    match contents {
        Ok(text) => {
            let mut arr: Vec<i32> = text.trim().split(' ').map(|x| x.parse::<i32>().unwrap()).collect();
            let mut n: i32 = arr.pop().unwrap();

            println!("{}", can_place_flowers(arr, n));
        },
        Err(e) => {
            println!("Eroare: {}", e);
        },
    }
}
