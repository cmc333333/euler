use std::env;
use std::io::{self, Read};

fn shift(window: &mut Vec<u64>, next_val: u64, window_size: usize) {
    for i in 1..window_size {
        window[i-1] = window[i];
    }
    window[window_size - 1] = next_val;
}

fn product(window: &Vec<u64>) -> u64 {
    let mut p = 1u64;
    for el in window {
        p = p * el;
    }
    p
}

fn max_window(string: String, window_size: usize) -> u64 {
    let mut window = vec![1u64; window_size];
    let mut max_product = 1u64;
    for digit in string.chars().map(|c| c.to_digit(10)) {
        match digit {
            Some(d) => {
                shift(&mut window, d as u64, window_size);
                let prod = product(&window);
                if prod > max_product {
                    max_product = prod;
                }
            },
            None => (),
        }
    };
    max_product
}

fn main() {
    let mut string = String::new();
    match (io::stdin().read_to_string(&mut string), env::args().nth(1)) {
        (Err(_), _) => println!("Problem reading from stdin"),
        (_, None) => println!("Need a first argument"),
        (Ok(_), Some(param)) => match param.parse::<usize>() {
            Ok(size) => println!("{}", max_window(string.to_string(), size)),
            Err(_) => println!("First argument must be an int")
        },
    }
}
