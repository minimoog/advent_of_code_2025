fn largest_joltage(bank: &str, num_digits: usize) -> u64 {
    let digits: Vec<u32> = bank.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    
    if digits.len() < num_digits {
        return 0;
    }
    
    // For part 1 (2 digits), brute force all pairs
    if num_digits == 2 {
        let mut max_val = 0u64;
        for i in 0..digits.len() {
            for j in (i+1)..digits.len() {
                let val = digits[i] as u64 * 10 + digits[j] as u64;
                max_val = max_val.max(val);
            }
        }
        return max_val;
    }
    
    // For larger num_digits, use monotonic stack approach
    // This finds the lexicographically largest subsequence
    let mut stack: Vec<u32> = Vec::new();
    let mut to_remove = digits.len() - num_digits;
    
    for &digit in &digits {
        // Pop smaller digits if we have removal budget
        while !stack.is_empty() && *stack.last().unwrap() < digit && to_remove > 0 {
            stack.pop();
            to_remove -= 1;
        }
        stack.push(digit);
    }
    
    // Take only the first num_digits
    let result: u64 = stack.iter()
        .take(num_digits)
        .fold(0, |acc, &d| acc * 10 + d as u64);
    
    result
}

fn solve_part1(input: &str) -> u64 {
    input.lines()
        .map(|line| largest_joltage(line.trim(), 2))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input.lines()
        .map(|line| largest_joltage(line.trim(), 12))
        .sum()
}

fn main() {
    /* 
    let example_input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
    
    println!("=== Example Input ===");
    println!("{}\n", example_input);
    
    // Part 1: Pick 2 batteries
    println!("=== Part 1: Pick 2 batteries ===");
    for line in example_input.lines() {
        let joltage = largest_joltage(line, 2);
        println!("{} -> {}", line, joltage);
    }
    let part1_total = solve_part1(example_input);
    println!("Part 1 Total: {}\n", part1_total);
    
    // Part 2: Pick 12 batteries
    println!("=== Part 2: Pick 12 batteries ===");
    for line in example_input.lines() {
        let joltage = largest_joltage(line, 12);
        println!("{} -> {}", line, joltage);
    }
    let part2_total = solve_part2(example_input);
    println!("Part 2 Total: {}\n", part2_total);
    
    // For your actual puzzle input, replace example_input with your input
    println!("=== To use with your puzzle input ===");
    println!("Replace 'example_input' with your actual input from adventofcode.com");
    */
    
    // Uncomment and paste your input here:
    let my_input = std::fs::read_to_string("puzzle.input").expect("wrong imput");
    // ... your input here ...
    // "#;
    println!("Part 1 Answer: {}", solve_part1(&my_input));
    println!("Part 2 Answer: {}", solve_part2(&my_input));
}
