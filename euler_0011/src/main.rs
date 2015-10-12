use std::io;
use std::io::{Read};

fn max_horiz(matrix: &Vec<Vec<u64>>, size: usize) -> u64 {
    let mut max = 0u64;
    for row in matrix {
        for i in 0..(row.len() - size + 1) {
            let product = (0..size).fold(1, |prod, next| prod * row[i + next]);
            if product > max {
                max = product;
            }
        }
    }
    max
}

fn max_vert(matrix: &Vec<Vec<u64>>, size: usize) -> u64 {
    let mut max = 0u64;
    for (row_idx, row) in matrix.iter().enumerate() {
        if row_idx < matrix.len() - size {
            for col_idx in 0..row.len() {
                let product = (0..size).fold(
                    1, |prod, next| prod * matrix[next + row_idx][col_idx]);
                if product > max {
                    max = product;
                }
            }
        }
    }
    max
}

fn max_desc(matrix: &Vec<Vec<u64>>, size: usize) -> u64 {
    let mut max = 0u64;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if row_idx < matrix.len() - size
                && col_idx < row.len() - size {
                let product = (0..size).fold(
                    1, |prod, next| prod * matrix[next + row_idx][next + col_idx]);
                if product > max {
                    max = product;
                }
            }
        }
    }
    max
}

fn max_asc(matrix: &Vec<Vec<u64>>, size: usize) -> u64 {
    let mut max = 0u64;
    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, col) in row.iter().enumerate() {
            if row_idx >= size - 1 && col_idx < row.len() - size {
                let product = (0..size).fold(
                    1, |prod, next| prod * matrix[row_idx - next][next + col_idx]);
                if product > max {
                    max = product;
                }
            }
        }
    }
    max
}

fn main() {
    let mut string = String::new();
    match io::stdin().read_to_string(&mut string) {
        Err(_) => println!("Problem reading from stdin"),
        Ok(_) => {
            let problem = string.trim();
            let mut matrix: Vec<Vec<u64>> = Vec::new();
            for row_str in problem.split('\n') {
                matrix.push(Vec::new());
                for col_str in row_str.split_whitespace() {
                    match (matrix.last_mut(), col_str.parse::<u64>().ok()) {
                        (Some(row), Some(val)) => row.push(val),
                        _ => ()
                    }
                }
            };
            println!("{}", max_horiz(&matrix, 4));
            println!("{}", max_vert(&matrix, 4));
            println!("{}", max_asc(&matrix, 4));
            println!("{}", max_desc(&matrix, 4));
        },
    }
}
