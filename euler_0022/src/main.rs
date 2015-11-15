use std::io;
use std::io::{Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    
    let mut names:Vec<&str> = Vec::new();
    for name in input.trim().split(',') {
        names.push(name);
    }
    names.sort();

    let mut sum = 0u64;
    for (idx, name) in names.iter().enumerate() {
        let mut sub_sum = 0u64;
        for c in name.chars() {
            sub_sum = sub_sum + match c {
                'A' => 1, 'B' => 2, 'C' => 3, 'D' => 4, 'E' => 5, 'F' => 6,
                'G' => 7, 'H' => 8, 'I' => 9, 'J' => 10, 'K' => 11, 'L' => 12,
                'M' => 13, 'N' => 14, 'O' => 15, 'P' => 16, 'Q' => 17,
                'R' => 18, 'S' => 19, 'T' => 20, 'U' => 21, 'V' => 22,
                'W' => 23, 'X' => 24, 'Y' => 25, 'Z' => 26, 
                _ => 0
            }
        }
        sum = sum + (idx as u64 + 1) * sub_sum;
    }
    println!("{}", sum);
}
