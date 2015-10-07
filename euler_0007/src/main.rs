use std::env;

fn next_prime(start: u64, primes: &Vec<u64>) -> u64 {
    let mut next = start + 1;
    loop {
        if primes.iter().any(|prime| next % prime == 0) {
            next += 1
        } else {
            return next
        }
    }
}

fn main() {
    match env::args().nth(1) {
        None => println!("Needs a first argument"),
        Some(string) => match string.parse::<u64>() {
            Ok(count) => {
                let mut max_prime = 1u64;
                let mut primes = Vec::new();
                for _ in 0..count {
                    max_prime = next_prime(max_prime, &primes);
                    primes.push(max_prime);
                }
                println!("{}", max_prime);
            },
            Err(_) => println!("First argument must be an int"),
        }
    }
}
