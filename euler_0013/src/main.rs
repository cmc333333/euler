use std::io;
use std::io::{Read};

fn char_at(string:&str, idx:usize) -> u64 {
    match string.chars().nth(idx) {
        Some(c) => c.to_digit(10).unwrap_or(0) as u64,
        None => 0u64
    }
}

fn main() {
    let mut string = String::new();
    match io::stdin().read_to_string(&mut string) {
        Err(_) => println!("Problem reading from stdin"),
        Ok(_) => {
            let numbers: Vec<&str> = string.trim().split('\n').collect();
            let mut nums: Vec<u64> = Vec::new();
            let mut carry = 0u64;
            let num_len = numbers[0].len();
            for i in 0..num_len {
                carry = numbers.iter().fold(
                    carry,
                    |so_far, next| so_far + char_at(next, num_len - i - 1));
                nums.push(carry % 10);
                carry = carry / 10;
            }
            print!("{}", carry);
            nums.reverse();
            for num in nums {
                print!("{}", num.to_string());
            }
            println!("");
        },
    }
}
