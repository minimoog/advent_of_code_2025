use std::fs;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("puzzle.input")?;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    //start
    let start_column = grid[0]
    .iter()
    .position(|&c| c == 'S');

    if let Some(start) = start_column {
        let mut current = vec![0u64; cols];
        current[start] = 1;

        for r in 0..rows {
            let mut next = vec![0u64; rows];

            for c in 0..cols {
                let count = current[c];

                if count == 0 { continue; }

                match grid[r][c] {
                    '^' => {
                        if c > 0 {
                            next[c-1] += count;
                        }

                        if c + 1 < cols {
                            next[c+1] += count;
                        }
                    },
                    _ => {
                        next[c] += count;
                    }
                }
            }

            current = next;

            for c in &current {
                print!("{c} ");
            }
            println!("");

        }

        let sum: u64 = current.iter().sum();

        println!("part2: {}", sum);

    }



    Ok(())
}
