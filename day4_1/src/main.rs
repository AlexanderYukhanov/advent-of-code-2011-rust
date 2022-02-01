use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    usize,
};

#[derive(Debug)]
struct Board {
    values: [[i32; 5]; 5],
}

impl Board {
    fn read_from<T: BufRead>(input: &mut T) -> Result<Board, Error> {
        let mut result = Board {
            values: [[0; 5]; 5],
        };
        for i in 0..5 {
            let mut line = String::new();
            input.read_line(&mut line)?;
            for (j, v) in line.split_whitespace().enumerate() {
                result.values[i][j] = v.parse().unwrap();
            }
        }
        Ok(result)
    }

    fn select(&mut self, value: i32) -> bool {
        for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] == value {
                    self.values[i][j] = -1;
                    return self.winner(i, j);
                }
            }
        }
        false
    }

    fn winner(&self, i: usize, j: usize) -> bool {
        let mut good_column = true;
        let mut good_row = true;
        for k in 0..5 {
            if self.values[k][j] != -1 {
                good_column = false;
            }
            if self.values[i][k] != -1 {
                good_row = false;
            }
        }
        good_column || good_row
    }

    fn value(&self) -> i32 {
        let mut result = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.values[i][j] != -1 {
                    result += self.values[i][j];
                }
            }
        }
        result
    }
}

fn main() -> Result<(), Error> {
    let mut input = BufReader::new(File::open("input")?);
    let mut str_selected: String = String::new();
    input.read_line(&mut str_selected).unwrap();
    let selected = str_selected.split(',').map(|l| l.parse::<i32>().unwrap());

    let mut boards: Vec<Board> = vec![];
    let mut skip = String::new();
    while input.read_line(&mut skip).unwrap() > 0 {
        boards.push(Board::read_from(&mut input)?);
    }

    for v in selected {
        for board in &mut boards {
            if board.select(v) {
                println!("Result: {}", v * board.value());
                return Ok(());
            }
        }
    }

    Ok(())
}
