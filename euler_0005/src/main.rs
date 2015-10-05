use std::collections::HashMap;
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

fn factors(product: u64) -> HashMap<u64, u32> {
    let mut remaining = product;
    let mut factors_map = HashMap::new();
    loop {
        let factor = first_factor(remaining);
        *factors_map.entry(factor).or_insert(0) += 1;
        remaining = remaining / factor;
        if factor == 1 {
            break;
        }
    }

    factors_map
}

fn lcm_up_to(upper_bound: u64) -> u64 {
    let mut max_factors = HashMap::new();
    for i in 2..(upper_bound + 1) {
        for (factor, count) in factors(i) {
           let entry = max_factors.entry(factor).or_insert(1);
           if count > *entry {
               *entry = count;
           }
        }
    }
    let mut accumulator = 1u64;
    for (factor, count) in max_factors {
        accumulator *= factor.pow(count);
    }
    accumulator
}

fn main() {
    match env::args().nth(1) {
        None => println!("Needs a first argument"),
        Some(string) => match string.parse::<u64>() {
            Ok(upper_bound) => println!("{}", lcm_up_to(upper_bound)),
            Err(_) => println!("First argument must be an int"),
        }
    }
}
