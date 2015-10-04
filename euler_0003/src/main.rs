use std::env;

fn first_factor(product: u64) -> u64 {
    let mut primes = Vec::new();
    for i in 2..(product + 1) {
        if !primes.iter().any(|p| i % p == 0) {    // is prime
            if product % i == 0 {
                return i;
            }
            primes.push(i);
        }
    }
    1
}

fn largest_prime(product: u64) -> u64 {
    let mut remaining = product;
    let mut primes = Vec::new();
    loop {
        let factor = first_factor(remaining);
        primes.push(factor);
        remaining = remaining / factor;
        if factor == 1 {
            break;
        }
    }

    let mut max_prime = 1;
    for p in primes {
        if p > max_prime {
            max_prime = p
        }
    }
    max_prime
}

fn main() {
    match env::args().nth(1) {
        None => println!("Needs a first argument"),
        Some(string) => match string.parse::<u64>() {
            Ok(product) => println!("{}", largest_prime(product)),
            Err(_) => println!("First argument must be an int"),
        }
    }
}
