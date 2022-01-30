use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = File::open("input")?;
    let reader = BufReader::new(input);
    let counters = reader
        .lines()
        .map(|line| line.unwrap())
        .fold([0; 12], |before, value| {
            let mut updated = before;
            for (i, c) in value.chars().enumerate() {
                if c == '0' {
                    updated[i] -= 1
                } else {
                    updated[i] += 1
                }
            }
            updated
        });
    let mut gamma = 0_i64;
    let mut epsilon = 0_i64;
    for v in counters {
        gamma *= 2;
        epsilon *= 2;
        if v > 0 {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }
    println!("Result: {}", gamma * epsilon);
    Ok(())
}
