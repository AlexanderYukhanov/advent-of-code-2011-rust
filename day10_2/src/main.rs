use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn price(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("unexpected closing bracket {:?}", c),
    }
}

fn is_close(c: char) -> bool {
    match c {
        ')' | ']' | '}' | '>' => true,
        _ => false,
    }
}

fn is_good(open: char, close: char) -> bool {
    (open == '<' && close == '>')
        || (open == '(' && close == ')')
        || (open == '[' && close == ']')
        || (open == '{' && close == '}')
}

fn price_of_line(s: &str) -> u64 {
    let mut stack = VecDeque::<char>::new();
    for c in s.chars() {
        if is_close(c) {
            if stack.is_empty() || !is_good(stack.pop_back().expect("not empty"), c) {
                return 0;
            }
        } else {
            stack.push_back(c);
        }
    }
    stack.iter().rev().fold(0_u64, |t, c| t * 5 + price(*c))
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?);
    let mut scores: Vec<u64> = input
        .lines()
        .map(|v| price_of_line(&v.unwrap()))
        .filter(|v| *v != 0)
        .collect();
    let mid = scores.len() / 2;
    scores.select_nth_unstable(mid);

    println!("Result: {}", scores[mid]);
    Ok(())
}
