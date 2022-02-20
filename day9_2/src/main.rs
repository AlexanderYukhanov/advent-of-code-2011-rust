use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

struct Position {
    j: usize,
    i: usize,
}

impl Position {
    fn new(j: usize, i: usize) -> Position {
        Position { j, i }
    }
}

fn is_good(j: usize, i: usize, map: &Vec<Vec<char>>) -> bool {
    map[j][i] != '9'
}

fn flood_fill(j: usize, i: usize, map: &mut Vec<Vec<char>>) -> u32 {
    if !is_good(j, i, map) {
        return 0;
    }

    let mut result = 0_u32;
    let mut candidates = VecDeque::<Position>::new();
    candidates.push_back(Position::new(j, i));
    while !candidates.is_empty() {
        let c = candidates.pop_front().expect("checked for is_empty above");
        if !is_good(c.j, c.i, map) {
            continue;
        }
        result += 1;
        map[c.j][c.i] = '9';
        if c.j > 0 {
            candidates.push_back(Position::new(c.j - 1, c.i));
        }
        if c.i > 0 {
            candidates.push_back(Position::new(c.j, c.i - 1));
        }
        if c.j + 1 < map.len() {
            candidates.push_back(Position::new(c.j + 1, c.i));
        }
        if c.i + 1 < map[c.j].len() {
            candidates.push_back(Position::new(c.j, c.i + 1));
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?);
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|v| v.unwrap())
        .map(|v| v.chars().collect())
        .collect();

    let mut areas = vec![0_u32];
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if is_good(j, i, &map) {
                areas.push(flood_fill(j, i, &mut map));
            }
        }
    }
    areas.select_nth_unstable_by(2, |a, b| b.cmp(a));

    println!("Result: {}", areas[0] * areas[1] * areas[2]);
    Ok(())
}
