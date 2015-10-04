fn is_palindrome(v:u64) -> bool {
    let v_str = v.to_string().into_bytes();
    let mut reversed = v_str.to_vec();
    reversed.reverse();
    for i in 0..v_str.len() {
        if v_str[i] != reversed[i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut max = 0u64;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            let product = i * j;
            if is_palindrome(product) {
                if product > max {
                    max = product;
                }
                break;
            }
        }
    }
    println!("{}", max);
}
