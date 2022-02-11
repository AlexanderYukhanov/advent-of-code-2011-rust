use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug, Clone)]
struct Digit {
    value: usize,
    indicators: HashSet<char>,
}

fn unk() -> Digit {
    Digit::new(42, "")
}

impl Digit {
    fn new(value: usize, indicators: &str) -> Digit {
        Digit {
            value,
            indicators: HashSet::from_iter(indicators.chars()),
        }
    }

    fn contains(&self, other: &Digit) -> bool {
        other.indicators.difference(&self.indicators).count() == 0
    }

    fn is(&self, s: &str) -> bool {
        HashSet::from_iter(s.chars()).eq(&self.indicators)
    }

    fn change(&self, other: &Digit) -> [usize; 2] {
        [
            other.indicators.difference(&self.indicators).count(),
            self.indicators.difference(&other.indicators).count(),
        ]
    }
}

fn to_digit(signal: &str, digits: &[Digit; 10]) -> Digit {
    let mut unknown = Digit::new(42, signal);
    unknown.value = match signal.len() {
        2 => 1,
        3 => 7,
        4 => 4,
        5 => {
            if unknown.contains(&digits[1]) {
                3
            } else if unknown.change(&digits[4]) == [2, 3] {
                2
            } else {
                5
            }
        }
        6 => {
            if unknown.contains(&digits[3]) {
                9
            } else if unknown.contains(&digits[7]) {
                0
            } else {
                6
            }
        }
        7 => 8,
        _ => 99,
    };
    unknown
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input")?);
    let mut result = 0_usize;
    for line in input.lines() {
        let line = line.unwrap();
        let left_right = line.split('|').collect::<Vec<&str>>();
        let mut digits: [Digit; 10] = [
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
            unk(),
        ];
        let mut signals = left_right[0].trim().split(' ').collect::<Vec<&str>>();
        signals.sort_by(|a, b| a.len().cmp(&b.len()));
        for signal in signals {
            let digit = to_digit(signal, &digits);
            digits[digit.value] = digit.clone();
        }
        let mut num = 0_usize;
        for s in left_right[1].trim().split(" ") {
            let ind = digits.iter().position(|d| d.is(s)).unwrap();
            num = num * 10 + ind;
        }
        result += num;
    }
    println!("Result: {}", result);
    Ok(())
}
