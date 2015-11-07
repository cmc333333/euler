use std::env;

fn main() {
    let len = match env::args().nth(1) {
        None => 3usize,
        Some(string) => string.parse::<usize>().unwrap_or(3usize)
    };
    let mut prev_row = (0..len).map(|_| 1u64).collect::<Vec<u64>>();
    for _ in 1..len {
        let mut row = vec![1u64];
        for i in 1..len {
            let val = row[i-1] + prev_row[i];
            row.push(val);
        }
        prev_row = row;
    }
    println!("{}", prev_row[len-1]);
}
