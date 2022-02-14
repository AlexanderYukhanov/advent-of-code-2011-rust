use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn is_good(j: usize, i: usize, map: &Vec<Vec<char>>) -> bool {
    [(-1, 0), (1, 0), (0, -1), (0, 1)].iter().all(|&(dj, di)| {
        map.get((j as isize + dj) as usize)
            .and_then(|v| v.get((i as isize + di) as usize))
            .and_then(|v| Some(map[j][i] < *v))
            .unwrap_or(true)
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?);
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|v| v.unwrap())
        .map(|v| v.chars().collect())
        .collect();

    let result: u32 = map
        .iter()
        .enumerate()
        .map(|(j, line)| {
            line.iter()
                .enumerate()
                .map(|(i, v)| {
                    if is_good(j, i, &map) {
                        1 + v.to_digit(10).unwrap()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum();

    println!("Result: {}", result);
    Ok(())
}
