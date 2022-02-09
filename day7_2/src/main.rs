use std::{
    fs::File,
    io::{BufRead, BufReader, Error}
};

fn cost_for_group(group: &Vec<i32>) -> i32 {
    group.iter().fold(0, |t, f| t + f + 1)
}

fn move_group(group: &mut Vec<i32>) {
    *group = group.iter().map(|v| v + 1).collect();
}

fn join_left_group(positions: &Vec<i32>, pos: i32, group: &mut Vec<i32>, mut consumed: usize) -> usize {

    while consumed < positions.len() &&  positions[consumed] == pos {
        consumed += 1;
        group.push(0);
    }
    consumed
}

fn join_right_group(positions: &Vec<i32>, pos: i32, group: &mut Vec<i32>, mut consumed: usize) -> usize {
    while consumed < positions.len() &&  positions[positions.len() - consumed - 1] == pos {
        consumed += 1;
        group.push(0);
    }
    consumed
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

    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    let mut lpos = positions[0] - 1;
    let mut rpos = positions[positions.len() - 1] + 1;
    let mut lconsumed: usize = 0;
    let mut rconsumed: usize = 0;
    let mut cost = 0;

    while lpos != rpos {
        let lprice = cost_for_group(&left);
        let rprice = cost_for_group(&right);
        if lprice < rprice {
            cost += lprice;
            move_group(&mut left);
            lpos += 1;
            lconsumed = join_left_group(&positions, lpos, &mut left, lconsumed);
        } else {
            cost += rprice;
            move_group(&mut right);
            rpos -= 1;
            rconsumed = join_right_group(&positions, rpos, &mut right, rconsumed);
        }
    }
    println!("Result: {}", cost);

    Ok(())
}
