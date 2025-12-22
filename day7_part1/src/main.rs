use std::io;
use std::fs::File;
use std::io::BufRead;
use std::collections::VecDeque;


mod matrix;
use matrix::Matrix;

const ROWS: usize = 142;
const COLS: usize = 143;

#[derive(Debug, PartialEq)]
struct Beam {
    x: usize,
    y: usize,
}

fn main() -> std::io::Result<()> {
    let file = File::open("puzzle.input")?;
    let reader = io::BufReader::new(file);

    let mut start = (0, 0);
    let mut manifold: Matrix<char> = Matrix::new(ROWS, COLS);

    for (idy, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                println!("{}", content);

                for (idx, c) in content.chars().enumerate() {
                    if c == 'S' {
                        start = (idx, idy);
                        manifold[(idx, idy)] = c;

                        continue;
                    }

                    println!("{} {}", idx, idy);

                    manifold[(idx, idy)] = c;
                }

            },
            Err(e) => println!("Error reading line {}", e)
        }
    }

    println!("{} {}", start.0, start.1);

    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(Beam { x: start.0, y: start.1 });

    let mut hit = 0;

    for _ in 0..ROWS {
        let mut next_beams: VecDeque<Beam> = VecDeque::new();

        while let Some(beam) = beams.pop_front() {
            let possible = Beam{x: beam.x, y: beam.y + 1};

            if manifold[(possible.x, possible.y)] == '^' {
                //split
                let left = Beam{x: possible.x - 1, y: possible.y };
                let right = Beam{x: possible.x + 1, y: possible.y };

                if !next_beams.contains(&left) {
                    next_beams.push_back(left);
                }

                if !next_beams.contains(&right) {
                    next_beams.push_back(right);
                }

                hit += 1;

            } else { //continue
                if !next_beams.contains(&possible) {
                    next_beams.push_back(possible);
                }
            }
        }

        beams = next_beams;

    }

    println!("{}", hit);

    Ok(())
}
