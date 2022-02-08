use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    usize,
};

fn main() -> Result<(), Error> {
    let mut input = BufReader::new(File::open("input")?);
    let mut input_str = String::new();
    input.read_line(&mut input_str).unwrap();

    let mut prev = [1_u64; 9];
    for _ in 0..256 {
        let mut cur = [0_u64; 9];
        cur[0] = prev[6] + prev[8];
        for i in 1..9 {
            cur[i] = prev[i - 1];
        }
        prev = cur;
    }

    let result = input_str
        .trim()
        .split(",")
        .map(|v| v.parse::<usize>().unwrap())
        .fold(0_u64, |v, i| v + prev[i]);

    println!("Result: {}", result);
    Ok(())
}
