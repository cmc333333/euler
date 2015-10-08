fn main() {
    for a in 1..1000 {
        for b in (a+1)..(1000-a) {
            for c in (b+1)..(1000-b) {
                if a + b + c == 1000 && a*a + b*b == c*c {
                    println!("a={}, b={}, c={}, abc={}", a, b, c, a*b*c);
                }
            }
        }
    }
}
