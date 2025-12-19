use std::{fs::File, io::{self, BufRead}};

enum Operation {
    Add,
    Multiply
}

fn main() -> std::io::Result<()> {
    let file = File::open("test.input")?;
    let reader = io::BufReader::new(file);

    let mut numbers: Vec<Vec<i64>> = vec![];
    let mut ops: Vec<Operation> = vec![];

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let numbers_line: Vec<i64> = content
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();

                let numbers_count = numbers_line.len();

                numbers_line.iter().for_each(|s| println!("{}", s) );

                if numbers_count == 0 {
                    ops = content
                        .split_whitespace()
                        .map(|s| if s == "+" {
                            Operation::Add
                        } else {
                            Operation::Multiply
                        })
                        .collect();

                    //ops.iter().for_each(|o| match o {
                    //    Operation::Add => println!("Add"),
                    //    Operation::Multiply => println!("Mult")
                    //});

                    continue;
                }

                numbers.push(numbers_line);
                
            },

            Err(e) => println!("Error parsing line {}", e)
        }
    }

    let mut part1: i64 = 0;

    println!("{} {}", numbers.len(), numbers[0].len());

    for i in 0..numbers[0].len() {
        match ops[i] {
            Operation::Add => {
                let mut result = 0;

                for j in 0..numbers.len() {
                    print!("add {} {} {};  ", i, j, numbers[j][i]);

                    result += numbers[j][i];
                }

                println!("");

                part1 += result;
            },
            Operation::Multiply => {
                let mut result = 1;

                for j in 0..numbers.len() {
                    print!("mult {} {} {};  ", i, j, numbers[j][i]);

                    result *= numbers[j][i];
                }

                println!("");

                part1 += result;
            }
        }
    }

    println!("{}", part1);

    Ok(())
}
