use std::io;
use std::io::{Read};

fn main() {
    let mut string = String::new();
    match io::stdin().read_to_string(&mut string) {
        Err(_) => println!("Problem reading from stdin"),
        Ok(_) => {
            let mut matrix: Vec<Vec<u64>> = Vec::new();
            for row_str in string.split('\n') {
                matrix.push(Vec::new());
                for col_str in row_str.split_whitespace() {
                    match (matrix.last_mut(), col_str.parse::<u64>().ok()) {
                        (Some(row), Some(val)) => row.push(val),
                        _ => ()
                    }
                }
            }
        },
    }
}
