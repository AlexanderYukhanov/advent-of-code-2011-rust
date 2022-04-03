use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn fill(j: usize, i: usize, map: &mut Vec<Vec<char>>) -> usize {
    let mut total = 1_usize;
    let mut pending: Vec<(usize, usize)> = vec![];
    pending.push((j, i));
    map[j][i] = '9';
    while !pending.is_empty() {
        let (y, x) = pending.pop().unwrap();
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .for_each(|(dy, dx)| {
                let y = (y as isize + dy) as usize;
                let x = (x as isize + dx) as usize;
                if '9'
                    != map
                        .get(y)
                        .and_then(|v| v.get(x))
                        .and_then(|v| Some(*v))
                        .unwrap_or('9')
                {
                    pending.push((y, x));
                    map[y][x] = '9';
                    total += 1;
                }
            });
    }
    total
}

fn main() -> Result<(), Error> {
    let input = BufReader::new(File::open("input")?);
    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|v| v.unwrap())
        .map(|v| v.chars().collect())
        .collect();

    let mut results = vec![0];
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] != '9' {
                results.push(fill(j, i, &mut map));
            }
        }
    }

    results.select_nth_unstable_by(2, |a, b| b.cmp(a));
    println!("Result: {}", results.iter().take(3).product::<usize>());
    Ok(())
}
