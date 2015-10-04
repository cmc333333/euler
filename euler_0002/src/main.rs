fn main() {
    let mut two_ago = 1u64;
    let mut one_ago = 2u64;
    let mut sum = 2u64;

    loop {
        let next = two_ago + one_ago;
        if next > 4000000 {
            break;
        }
        if next % 2 == 0 {
            sum += next;
        }
        two_ago = one_ago;
        one_ago = next;
    }
    println!("{}", sum);
}
