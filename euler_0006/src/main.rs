use std::env;

fn main() {
    let mut sum_of_squares = 0i64;
    let mut square_of_sums = 0i64;
    match env::args().nth(1) {
        None => println!("Needs a first argument"),
        Some(string) => match string.parse::<i64>() {
            Ok(upper_bound) => 
            {
                for i in 1..(upper_bound + 1) {
                    sum_of_squares += i*i;
                    square_of_sums += i;
                }
                square_of_sums = square_of_sums * square_of_sums;
                println!("{} - {} = {}", sum_of_squares, square_of_sums,
                         sum_of_squares - square_of_sums)
            },
            Err(_) => println!("First argument must be an int"),
        }
    }

}
