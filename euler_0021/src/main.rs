use std::collections::HashMap;
use std::env;
extern crate factors;

fn main() {
    let mut primes = factors::SourcePrimes::new();
    let mut d:HashMap<u64, u64> = HashMap::new();
    let mut amicable = 0u64;
    let max_str = env::args().nth(1).unwrap_or("10000".to_string());
    let max = max_str.parse::<u64>().ok().unwrap_or(10000);
    for i in 2..max {
        let factors = primes.factorize(i).proper_factors();
        d.insert(i, factors.iter().fold(0u64, |sofar, next| sofar + next));
    }
    for (left, right) in d.iter() {
        if *left == *d.get(&right).unwrap_or(&0u64) && *left < *right {
            println!("{}, {}", left, right);
            amicable = amicable + left + right;
        }
    }
    println!("amicable: {}", amicable);
}
