use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
struct Group {
    direction: i32,
    position: i32,
    elements: u32,
    last_cost: u32,
}

impl Group {
    fn new(direction: i32, position: i32) -> Group {
        Group {
            direction,
            position,
            elements: 0,
            last_cost: 0,
        }
    }

    fn cost_of_move(&self) -> u32 {
        self.last_cost + self.elements
    }

    fn make_move(&mut self) {
        self.position += self.direction;
        self.last_cost += self.elements;
    }

    fn jon_group(&mut self) {
        self.elements += 1;
    }
}

fn main() -> Result<(), Error> {
    let mut input = BufReader::new(File::open("input")?);
    let mut input_str = String::new();
    input.read_line(&mut input_str).unwrap();
    let mut positions: Vec<i32> = input_str
        .trim()
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    positions.sort();
    let mut positions: VecDeque<i32> = positions.iter().cloned().collect();

    let mut cost = 0;
    let mut left = Group::new(1, positions[0] - 1);
    let mut right = Group::new(-1, positions[positions.len() - 1] + 1);

    while left.position != right.position {
        if left.cost_of_move() < right.cost_of_move() {
            cost += left.cost_of_move();
            left.make_move();
            while !positions.is_empty() && *positions.front().expect("") == left.position {
                left.jon_group();
                positions.pop_front();
            }
        } else {
            cost += right.cost_of_move();
            right.make_move();
            while !positions.is_empty() && *positions.back().expect("") == right.position {
                right.jon_group();
                positions.pop_back();
            }
        }
    }

    println!("Result: {}", cost);
    Ok(())
}
