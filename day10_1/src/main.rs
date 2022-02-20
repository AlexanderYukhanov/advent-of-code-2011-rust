use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn price(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
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

fn price_of_line(s: &str) -> u32 {
    let mut stack = VecDeque::<char>::new();
    for c in s.chars() {
        if is_close(c) {
            if stack.is_empty() || !is_good(stack.pop_back().expect("not empty"), c) {
                return price(c);
            }
        } else {
            stack.push_back(c);
        }
    }
    0
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = BufReader::new(File::open("input")?);
    let result: u32 = input.lines().map(|v| price_of_line(&v.unwrap())).sum();
    println!("Result: {}", result);
    Ok(())
}
