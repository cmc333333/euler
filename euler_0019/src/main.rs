use std::ops::Add;

// All values are one indexed
struct Date {
    year: u32,
    month: u32,
    day: u32
}

impl Date {
    fn comparable(&self) -> u32 {
        self.day + self.month*100 + self.year*10000
    }

    fn add(self, days:u32) -> Date {
        if days == 0 {
            self
        } else if days != 1 {
            self.add(1).add(days - 1)
        } else if self.month == 12 && self.day == 31 {
            Date{ year: self.year + 1, month: 1, day: 1 }
        } else if self.day == 31 {
            Date{ year: self.year, month: self.month + 1, day: 1 }
        } else if self.day == 30 && [4, 6, 9, 11].contains(&self.month) {
            Date{ year: self.year, month: self.month + 1, day: 1 }
        } else if self.day == 29 && self.month == 2 {
            Date{ year: self.year, month: 3, day: 1 }
        } else if self.day == 28 && self.month == 2
                  && (self.year % 4 != 0 || self.year % 400 == 0) {
            Date{ year: self.year, month: 3, day: 1 }
        } else {
            Date{ year: self.year, month: self.month, day: self.day + 1 }
        }
    }
}

fn main() {
    let mut date = Date{ year: 1900, month: 1, day: 1 };
    let begin = Date { year: 1901, month: 1, day: 1 };
    let end = Date { year: 2000, month: 12, day: 1 };

    let mut count = 0;
    while date.comparable() <= end.comparable() {
        if date.comparable() >= begin.comparable() && date.day == 1 {
            count += 1;
        }
        date = date.add(7);
    }
    println!("{}", count);
}
