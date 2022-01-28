use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_values(path: &str) -> Result<Vec<i32>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    Ok(reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .collect())
}

fn main() -> Result<(), io::Error> {
    let values = read_values("input")?;
    let result: i32 = values
        .windows(3)
        .map(|f| f.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|f| if f[1] > f[0] { 1 } else { 0 })
        .sum();
    println!("Result: {}", result);
    Ok(())
}
