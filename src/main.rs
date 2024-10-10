use std::io;
use substring::Substring;

fn main() {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("Failed to read line");

    let answer = find_max_palindrome(&str);
    println!("{}", answer);
}

fn find_max_palindrome(str: & String) -> String {
    let str_len: i16 = str.len() as i16;
    let mut max_palindrome_len: i16 = 0;
    let mut max_palindrome_left_border = 0;
    let mut max_palindrome_right_border = 0;

    for i in 0..str_len {
        let mut palindrome_odd_len = 1;
        let mut left_border = i - palindrome_odd_len;
        let mut right_border = i + palindrome_odd_len;

        while left_border >= 0 && right_border < str_len && str.chars().nth(left_border as usize) == str.chars().nth(right_border as usize) {
            palindrome_odd_len += 1;
            left_border = i - palindrome_odd_len;
            right_border = i + palindrome_odd_len;
        }

        if palindrome_odd_len > max_palindrome_len {
            max_palindrome_len = palindrome_odd_len;
            max_palindrome_left_border = left_border;
            max_palindrome_right_border = right_border;
        }
    }

    for i in 0..str_len {
        let mut palindrome_even_len = 1;
        let mut left_border = i - palindrome_even_len;
        let mut right_border = i + palindrome_even_len - 1;

        while left_border >= 0 && right_border < str_len && str.chars().nth(left_border as usize) == str.chars().nth(right_border as usize) {
            palindrome_even_len += 1;
            left_border = i - palindrome_even_len;
            right_border = i + palindrome_even_len - 1;
        }

        if palindrome_even_len > max_palindrome_len {
            max_palindrome_len = palindrome_even_len;
            max_palindrome_left_border = left_border;
            max_palindrome_right_border = right_border;
        }
    }

    str.substring((max_palindrome_left_border + 1) as usize, (max_palindrome_right_border) as usize).to_string()
}
