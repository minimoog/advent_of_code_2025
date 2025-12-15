use std::{fs::File, io::{self, BufRead}};

mod matrix;

use matrix::Matrix;

const ROWS: usize = 137;
const COLS: usize = 137;

fn main() -> std::io::Result<()> {
    let mut count1 = 0;

    let file = File::open("puzzle.input")?;

    let reader = io::BufReader::new(file);

    let mut grid = Matrix::<char>::new(ROWS, COLS);

    for (idy, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                //println!("{}, {}", idx, content);

                for (idx, c) in content.chars().enumerate() {
                    grid[(idx, idy)] = c;
                }

            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    for y in 0..ROWS {
        for x in 0..COLS {
            if grid[(x, y)] == '.' { continue; }

            let counter = grid.neighbors8(x, y).into_iter()
                .filter(|&x| *x.2 == '@')
                .count();

            if counter < 4 { count1 += 1; }
        }

    }

    println!("{}", count1);

    Ok(())
}