use std::collections::HashMap;
use std::env;

fn next_prime(start: u64, primes: &Vec<u64>) -> u64 {
    let mut next = start + 1;
    loop {
        let mut is_prime = true;
        for prime in primes {
            if next % prime == 0 {
                is_prime = false;
                break;
            } else if prime > &(((next as f64).sqrt() as u64) + 1) {
                break;
            }
        }

        if is_prime {
            return next;
        } else {
            next += 1;
        }
    }
}

fn num_factors(product: u64, primes: &Vec<u64>) -> u64 {
    let mut remaining = product;
    let mut factors_map = HashMap::new();
    let mut is_prime = false;
    while !is_prime {
        let root = ((remaining as f64).sqrt() as u64) + 1;
        //let root = product;
        is_prime = true;
        for factor in primes {
            if remaining % factor == 0 {
                *factors_map.entry(factor).or_insert(0u64) += 1;
                is_prime = false;
                remaining = remaining / factor;
                break;
            }
            if factor > &root {
                break;
            }
        }
    }
    if remaining > 1 {
        *factors_map.entry(&remaining).or_insert(0u64) += 1;
    }

    factors_map.values().fold(1u64, |so_far, next| so_far * (next + 1))
}

fn main() {
    let mut primes = Vec::new();
    let mut prime = 1u64;
    let mut triangle = 0u64;
    let mut idx = 0u64;

    let factor_count = match env::args().nth(1) {
        None => 5,
        Some(string) => match string.parse::<u64>() {
            Ok(fc) => fc,
            Err(_) => 5
        }
    };

    while num_factors(triangle, &primes) < factor_count {
        idx += 1;
        triangle += idx;
        while prime <= ((triangle as f64).sqrt() as u64) + 1 {
            prime = next_prime(prime, &primes);
            primes.push(prime);
        }
    }
    println!("{}:{}", idx, triangle);
}
