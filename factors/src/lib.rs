use std::collections::HashMap;

pub struct SourcePrimes {
    primes: Vec<u64>,
    until: u64
}

pub struct Factors {
    primes: Vec<u64>,
    counts: Vec<u64>
}

impl Factors {
    pub fn new(factor_map: &HashMap<u64, u64>) -> Factors {
        let (primes, counts) = factor_map.iter().unzip();
        Factors { primes: primes, counts: counts }
    }

    pub fn product(&self) -> u64 {
        let mut product = 1u64;
        for (prime, count) in self.primes.iter().zip(self.counts.iter()) {
            for _ in 0..*count {
                product = product * prime;
            }
        }
        product
    }

    pub fn proper_factors(&self) -> Vec<u64> {
        let mut factors:Vec<u64> = Vec::new();
        let mut iterator = Factors{
            primes: self.primes.to_vec(),
            counts: self.counts.iter().map(|_| 0u64).collect()
        };
        while self.product() != iterator.product() {
            factors.push(iterator.product());
            iterator = self.increment(&iterator);
        }
        factors
    }

    fn increment(&self, other: &Factors) -> Factors {
        let mut new_counts:Vec<u64> = Vec::new();
        let mut carry = 1u64;

        for (cap, count) in self.counts.iter().zip(other.counts.iter()) {
            let new_sum = count + carry;
            new_counts.push(new_sum % (cap + 1));
            carry = new_sum / (cap + 1);
        }
        Factors{ primes: other.primes.to_vec(), counts: new_counts}
    }
}

impl SourcePrimes {
    pub fn new() -> SourcePrimes {
        let mut vec:Vec<u64> = Vec::new();
        vec.push(2);
        SourcePrimes { primes: vec, until: 2 }
    }

    fn fill_next_prime(&mut self) {
        let mut potential_prime = self.until + 1;
        while self.primes.iter().any(|p| potential_prime % p == 0) {
            potential_prime += 1;
        }
        self.primes.push(potential_prime);
        self.until = potential_prime;
    }

    pub fn is_prime(&mut self, n:u64) -> bool {
        let sqrt = ((n as f64).sqrt() as u64) + 1;
        while self.until < sqrt {
            self.fill_next_prime();
        }
        !self.primes.iter().any(|p| n % p == 0)
    }

    pub fn factorize(&mut self, n:u64) -> Factors {
        let mut remaining = n;
        let mut factors_map:HashMap<u64, u64> = HashMap::new();
        while !self.is_prime(remaining) {
            for prime_ref in self.primes.iter() {
                let prime = *prime_ref;
                if remaining % prime == 0 {
                    *factors_map.entry(prime).or_insert(0u64) += 1;
                    remaining = remaining / prime;
                    break;
                }
            }
        }
        if remaining > 1 {
            *factors_map.entry(remaining).or_insert(0u64) += 1;
        }
        Factors::new(&factors_map)
    }
}
