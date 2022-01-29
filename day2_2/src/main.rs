use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> Result<(), io::Error> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    let result = reader
        .lines()
        .map(
            |l| match l.unwrap().split(" ").collect::<Vec<&str>>().as_slice() {
                ["up", v] => (0, -1 * v.parse::<i32>().unwrap()),
                ["down", v] => (0, v.parse::<i32>().unwrap()),
                ["forward", v] => (v.parse::<i32>().unwrap(), 0),
                _ => panic!("Wrong input - [direction x] expected"),
            },
        )
        .fold((0, 0, 0), |a, b| (a.0 + b.0, a.1 + b.1, a.2 + b.0 * a.1));
    println!("Result: {}", result.0 * result.2);
    Ok(())
}
