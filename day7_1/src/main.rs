use std::{
    fs::File,
    io::{BufRead, BufReader, Error}
};

fn main() -> Result<(), Error> {
    let mut input = BufReader::new(File::open("input")?);
    let mut input_str = String::new();
    input.read_line(&mut input_str).unwrap();

    let mut positions: Vec<i32> = input_str
        .trim()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let middle = positions.len() / 2;
    let pos = *positions.select_nth_unstable(middle).1;
    let res = positions.iter().fold(0, |p, v| p + (*v - pos).abs());
    println!("Result: {}", res);

    Ok(())
}
