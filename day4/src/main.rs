use std::{fs::File, io::{self, BufRead}};

mod matrix;

use matrix::Matrix;

const ROWS: usize = 137;
const COLS: usize = 137;

fn removable_papers(grid: &Matrix<char>) -> Vec<(usize, usize)>
{
    let mut output: Vec<(usize, usize)> = Vec::new();

    for y in 0..ROWS {
        for x in 0..COLS {
            if grid[(x, y)] == '.' { continue; }

            let counter = grid.neighbors8(x, y).into_iter()
                .filter(|&x| *x.2 == '@')
                .count();

            if counter < 4 { 
                output.push((x, y));
            }
        }

    }

    output
}

fn remove_papers(grid: &mut Matrix<char>, toremove: Vec<(usize, usize)>)
{
    for (x, y) in toremove {
        grid[(x, y)] = '.';
    }
}

fn main() -> std::io::Result<()> {
    let mut count1 = 0;
    let mut count2 = 0;

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

    //part 1
    for y in 0..ROWS {
        for x in 0..COLS {
            if grid[(x, y)] == '.' { continue; }

            let counter = grid.neighbors8(x, y).into_iter()
                .filter(|&x| *x.2 == '@')
                .count();

            if counter < 4 { count1 += 1; }
        }

    }

    println!("part1: {}", count1);

    //part2

    loop {
        let removable = removable_papers(&grid);

        if removable.len() == 0 {
            break;
        }

        count2 += removable.len();

        remove_papers(&mut grid, removable);
    }

    println!("part2: {}", count2);

    Ok(())
}