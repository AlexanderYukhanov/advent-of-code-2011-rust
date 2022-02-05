use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    usize,
};

use regex::Regex;

type Image = [[u32; 1024]; 1024];

fn delta(v0: usize, v1: usize) -> isize {
    if v1 == v0 {
        0
    } else if v1 > v0 {
        1
    } else {
        -1
    }
}

fn change(v: usize, dv: isize) -> usize {
    (v as isize + dv) as usize
}

fn draw(image: &mut Image, x0: usize, y0: usize, x1: usize, y1: usize) {
    if x1 == x0 && y1 == y0 {
        image[y0][x0] += 1;
        return;
    }

    let dx = delta(x0, x1);
    let dy = delta(y0, y1);
    let mut x = x0;
    let mut y = y0;
    while x != x1 || y != y1 {
        image[y][x] += 1;
        x = change(x, dx);
        y = change(y, dy);
    }
    image[y1][x1] += 1;
}

fn get(cap: &regex::Captures, i: usize) -> usize {
    cap.get(i).map_or(0, |v| v.as_str().parse().unwrap())
}

fn main() -> Result<(), Error> {
    let mut image = [[0; 1024]; 1024];
    let re = Regex::new(r"(.*),(.*) -> (.*),(\d*)").unwrap();
    let input = BufReader::new(File::open("input")?);
    for line in input.lines() {
        let ln = line.unwrap();
        let cap = re.captures(&ln).unwrap();
        draw(
            &mut image,
            get(&cap, 1),
            get(&cap, 2),
            get(&cap, 3),
            get(&cap, 4),
        );
    }

    let mut cnt = 0;
    for i in 0..1024_usize {
        for j in 0..1024_usize {
            if image[i][j] > 1 {
                cnt += 1;
            }
        }
    }

    println!("Result: {}", cnt);
    Ok(())
}
