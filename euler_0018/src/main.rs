use std::io;
use std::io::{Read};

fn main() {
    let mut last_row:Vec<i32> = Vec::new();
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).ok();
    for line in string.trim().split('\n') {
        let next_row:Vec<i32> = line.split(" ").map(
            |s| s.parse::<i32>().ok().unwrap_or(0)).collect();
        let mut left = last_row.to_vec();
        left.insert(0, 0);
        let mut right = last_row.to_vec();
        right.push(0);

        let mut sums = next_row.to_vec();
        for idx in 0..sums.len() {
            if left[idx] > right[idx] {
                sums[idx] += left[idx];
            } else {
                sums[idx] += right[idx];
            }
        }
        last_row = sums;
    }

    println!("{}", last_row.iter().max().unwrap_or(&0));
}
