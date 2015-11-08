use std::env;

fn as_word(i:u16) -> String {
    match (i % 10, (i/10) % 10, (i/100) % 10, (i/1000) % 10) {
        (0, 0, 0, 0) => "zero".to_string(),
        (1, 0, 0, 0) => "one".to_string(),
        (2, 0, 0, 0) => "two".to_string(),
        (3, 0, 0, 0) => "three".to_string(),
        (4, 0, 0, 0) => "four".to_string(),
        (5, 0, 0, 0) => "five".to_string(),
        (6, 0, 0, 0) => "six".to_string(),
        (7, 0, 0, 0) => "seven".to_string(),
        (8, 0, 0, 0) => "eight".to_string(),
        (9, 0, 0, 0) => "nine".to_string(),
        (0, 1, 0, 0) => "ten".to_string(),
        (1, 1, 0, 0) => "eleven".to_string(),
        (2, 1, 0, 0) => "twelve".to_string(),
        (3, 1, 0, 0) => "thirteen".to_string(),
        (5, 1, 0, 0) => "fifteen".to_string(),
        (8, 1, 0, 0) => "eighteen".to_string(),
        (ones, 1, 0, 0) => format!("{}teen", as_word(ones)),
        (ones, tens, 0, 0) => {
            let head = match tens {
                2 => "twenty".to_string(),
                3 => "thirty".to_string(),
                4 => "forty".to_string(),
                5 => "fifty".to_string(),
                8 => "eighty".to_string(),
                tens => format!("{}ty", as_word(tens))
            };
            match ones {
                0 => head,
                ones => format!("{}-{}", head, as_word(ones))
            }
        },
        (0, 0, hundreds, 0) => format!("{} hundred", as_word(hundreds)),
        (ones, tens, hundreds, 0) =>
            format!("{} hundred and {}", as_word(hundreds),
                    as_word(tens*10 + ones)),
        (0, 0, 0, 1) => "one thousand".to_string(),
        _ => format!("No resolution ({})", i)
    }
}

fn main() {
    let max = env::args().nth(1).and_then(
        |string| string.parse::<u16>().ok()).unwrap_or(5);
    let mut concatenated = "".to_string();
    for i in 1..(max+1) {
        let word = as_word(i).replace("-", "").replace(" ", "");
        concatenated = concatenated + &word;
    }
    println!("{}", concatenated);
    println!("Len: {}", concatenated.len());
}
