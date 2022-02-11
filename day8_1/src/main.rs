use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input")?);
    let result: u32 = input
        .lines()
        .map(|v| v.unwrap())
        .map(|v| v.split('|').nth(1).expect("").to_string())
        .map(|v| {
            v.split(" ")
                .map(|v| match v.len() {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum();

    println!("Result: {}", result);
    Ok(())
}
