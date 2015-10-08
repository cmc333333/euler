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

fn sum(primes: &Vec<u64>) -> u64 {
    primes.iter().fold(0, |acc, &i| acc + i)
}

fn main() {
    match env::args().nth(1) {
        None => println!("Needs a first argument"),
        Some(string) => match string.parse::<u64>() {
            Ok(max) => {
                let mut primes = Vec::new();
                let mut prime = 1u64;
                loop {
                    prime = next_prime(prime, &primes);
                    if prime >= max {
                        println!("{}", sum(&primes));
                        break;
                    }
                    primes.push(prime);
                }
            },
            Err(_) => println!("First argument must be an int"),
        }
    }
}
