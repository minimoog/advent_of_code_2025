use std::fs;

use good_lp::*;

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<i32>,
}

fn parse_input(line: &str) -> Machine {
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    // Parse lights [.##.]
    let lights_str = parts[0].trim_matches(|c| c == '[' || c == ']');
    let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();
    
    // Parse buttons (0,2,3)
    let mut buttons = Vec::new();
    let mut joltages_str = "";
    
    for part in &parts[1..] {
        if part.starts_with('(') {
            let button_str = part.trim_matches(|c| c == '(' || c == ')');
            let button: Vec<usize> = button_str
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            buttons.push(button);
        } else if part.starts_with('{') {
            joltages_str = part;
        }
    }
    
    // Parse joltages {7,5,12}
    let joltages_str = joltages_str.trim_matches(|c| c == '{' || c == '}');
    let joltages: Vec<i32> = joltages_str
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    
    Machine { lights, buttons, joltages }
}

fn solve_machine_ilp(machine: &Machine) -> Result<i32, String> {
    let num_buttons = machine.buttons.len();
    let num_joltages = machine.joltages.len();
    
    // Create variables for button presses
    let mut vars = Vec::new();
    let mut problem = ProblemVariables::new();
    
    for i in 0..num_buttons {
        vars.push(problem.add(variable().integer().min(0)));
    }
    
    // Objective: minimize total button presses
    let objective = vars.iter().sum::<Expression>();
    
    // Build the problem
    let mut solver_problem = problem.minimise(objective).using(default_solver);
    
    // Add constraints: for each joltage position, sum of presses must equal target
    for joltage_idx in 0..num_joltages {
        let target = machine.joltages[joltage_idx] as f64;
        
        // Sum contributions from all buttons that affect this joltage
        let constraint: Expression = machine.buttons.iter().enumerate()
            .filter(|(_, button)| button.contains(&joltage_idx))
            .map(|(btn_idx, _)| vars[btn_idx])
            .sum();
        
        solver_problem = solver_problem.with(constraint.eq(target));
    }
    
    // Solve
    match solver_problem.solve() {
        Ok(solution) => {
            let total: i32 = vars.iter()
                .map(|&var| solution.value(var) as i32)
                .sum();
            Ok(total)
        }
        Err(e) => Err(format!("Failed to solve: {:?}", e))
    }
}

fn main() {
    /*
    let input = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;
*/

    let input = fs::read_to_string("puzzle.input").expect("input not available");
    
    let mut total = 0;
    
    for (idx, line) in input.lines().enumerate() {
        let machine = parse_input(line);
        println!("Machine {}: {} lights, {} buttons, {} joltage targets", 
                 idx + 1, 
                 machine.lights.len(),
                 machine.buttons.len(),
                 machine.joltages.len());
        
        match solve_machine_ilp(&machine) {
            Ok(presses) => {
                println!("  Minimum presses: {}", presses);
                total += presses;
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
    }
    
    println!("\nTotal minimum presses across all machines: {}", total);
}
