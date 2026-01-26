use regex::Regex;

struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>
}

impl Machine {
    fn parse(line: &str) -> Self {
        let lights_regex = Regex::new(r"\[([.#]+)\]").unwrap();

        let target = lights_regex
            .captures(line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .chars()
            .map(|c| c == '#')
            .collect();


        let buttons_regex = Regex::new(r"\(([0-9,]+)\)").unwrap();

        let buttons = buttons_regex
            .captures_iter(line)
            .map(|cap| {
                cap.get(1)
                    .unwrap()
                    .as_str()
                    .split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Machine {target, buttons }
    }


    fn solve(&self) -> Option<usize> {
        let num_lights = self.target.len();
        let num_buttons = self.buttons.len();

        // Augmented matrix [A | b]
        let mut matrix = vec![vec![0u8; num_buttons + 1]; num_lights];

        for (light_index, row) in matrix.iter_mut().enumerate() {
            for (button_index, button) in self.buttons.iter().enumerate() {
                if button.contains(&light_index) {
                    row[button_index] = 1;
                }
            }
            row[num_buttons] = self.target[light_index] as u8;
        }

        // Gaussian elimination over GF(2)
        let mut pivot_row = 0;
        let mut pivot_cols = Vec::new();

        for col in 0..num_buttons {
            let mut found = false;

            for row in pivot_row..num_lights {
                if matrix[row][col] == 1 {
                    matrix.swap(pivot_row, row);
                    found = true;
                    break;
                }
            }

            if !found {
                continue;
            }

            pivot_cols.push(col);

            for row in 0..num_lights {
                if row != pivot_row && matrix[row][col] == 1 {
                    for c in 0..=num_buttons {
                        matrix[row][c] ^= matrix[pivot_row][c];
                    }
                }
            }

            pivot_row += 1;
        }

        // Check inconsistency
        for row in pivot_row..num_lights {
            if matrix[row][num_buttons] == 1 {
                return None;
            }
        }

        // Base solution (free vars = 0)
        let mut base_solution = vec![0u8; num_buttons];
        for (row, &col) in pivot_cols.iter().enumerate() {
            base_solution[col] = matrix[row][num_buttons];
        }

        // Identify free variables
        let mut is_pivot = vec![false; num_buttons];
        for &c in &pivot_cols {
            is_pivot[c] = true;
        }

        let free_cols: Vec<usize> = (0..num_buttons)
            .filter(|&c| !is_pivot[c])
            .collect();

        // Build nullspace basis
        let mut nullspace = Vec::new();

        for &free_col in &free_cols {
            let mut v = vec![0u8; num_buttons];
            v[free_col] = 1;

            for (row, &pivot_col) in pivot_cols.iter().enumerate() {
                if matrix[row][free_col] == 1 {
                    v[pivot_col] = 1;
                }
            }

            nullspace.push(v);
        }

        // Enumerate all combinations to minimize presses
        let mut best = None;

        for mask in 0..(1usize << nullspace.len()) {
            let mut candidate = base_solution.clone();

            for i in 0..nullspace.len() {
                if (mask >> i) & 1 == 1 {
                    for j in 0..num_buttons {
                        candidate[j] ^= nullspace[i][j];
                    }
                }
            }

            let weight = candidate.iter().filter(|&&x| x == 1).count();
            best = Some(best.map_or(weight, |b: usize| b.min(weight)));
        }

        best
    }

}

fn solve(input: &str) -> usize {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let machine = Machine::parse(line);
            machine.solve().unwrap_or_else(|| {
                eprintln!("Warning: No solution for line: {}", &line[..line.len().min(50)]);
                0
            })
        })
        .sum()
}

fn main() {
    //let example = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
//[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    //let result = solve(example);
    //println!("Example result: {}", result);
    //println!("Expected: 7");

    let input = std::fs::read_to_string("puzzle.input").unwrap();
    let answer = solve(&input);
    println!("Answer: {}", answer);
}
