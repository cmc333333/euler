use std::env;

type BigInt = Vec<u64>;

fn mult(left:&BigInt, right:u64) -> BigInt {
    let mut carry = 0u64;
    let mut result = BigInt::new();

    for digit in left {
        let next_val = digit * right + carry;
        result.push(next_val % 10);
        carry = next_val / 10;
    }

    while carry > 0u64 {
        result.push(carry % 10);
        carry = carry / 10;
    }
    result
}

fn print(val:&BigInt) {
    for digit in val {
        print!("{}", digit);
    }
    println!("");
}

fn main() {
    let factor = env::args().nth(1).and_then(
        |string| string.parse::<u64>().ok()).unwrap_or(2);
    let times = env::args().nth(2).and_then(
        |string| string.parse::<u64>().ok()).unwrap_or(15);
    let mut result = BigInt::new();
    result.push(1u64);
    
    for _ in 0..times {
        result = mult(&result, factor);
    }
    
    result.reverse();
    print(&result);
    println!("{}", result.iter().fold(0u64, |sum, x| sum + x));
}
