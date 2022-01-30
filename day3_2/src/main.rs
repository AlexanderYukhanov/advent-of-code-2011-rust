use std::{
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader, Error},
    usize,
};

const BINARY_NUMBER_LENGTH: usize = 12;
type BinaryNumber = [char; BINARY_NUMBER_LENGTH];

fn read_input() -> Result<Vec<BinaryNumber>, Error> {
    let input = BufReader::new(File::open("input")?);
    Ok(input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .collect::<Vec<char>>()
                .as_slice()
                .try_into()
                .unwrap()
        })
        .collect())
}

fn most_common(input: &[BinaryNumber], pos: usize) -> char {
    if input
        .iter()
        .map(|v| if v[pos] == '0' { -1 } else { 1 })
        .sum::<i32>()
        >= 0
    {
        '1'
    } else {
        '0'
    }
}

fn least_common(input: &[BinaryNumber], pos: usize) -> char {
    if most_common(input, pos) == '1' {
        '0'
    } else {
        '1'
    }
}

fn filter(input: &[BinaryNumber], pos: usize, value: char) -> Vec<BinaryNumber> {
    input
        .iter()
        .filter(|v| v[pos] == value)
        .copied()
        .collect::<Vec<BinaryNumber>>()
}

fn select(input: &[BinaryNumber], f: fn(&[BinaryNumber], usize) -> char) -> BinaryNumber {
    let mut rem: Vec<BinaryNumber> = input.to_vec();
    for i in 0..BINARY_NUMBER_LENGTH {
        if rem.len() == 1 {
            break;
        }
        let v = f(&rem, i);
        rem = filter(&rem, i, v);
    }
    rem[0]
}

fn input_to_value(v: &BinaryNumber) -> u32 {
    let mut r = 0_u32;
    for c in v {
        r *= 2;
        if *c == '1' {
            r += 1;
        }
    }
    r
}

fn main() -> Result<(), Error> {
    let input = read_input()?;
    let oxygen = select(&input, most_common);
    let co2 = select(&input, least_common);
    println!("Result: {}", input_to_value(&oxygen) * input_to_value(&co2));

    Ok(())
}
