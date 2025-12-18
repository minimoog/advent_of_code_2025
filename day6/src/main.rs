use std::{fs::File, io::{self, BufRead}};

enum Operation {
    Add,
    Multiply
}

fn main() -> std::io::Result<()> {
    let file = File::open("test.input")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => {
                let numbers : Vec<i64> = content
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i64>().ok())
                    .collect();

                numbers.iter().for_each(|s| println!("{}", s) );

                if numbers.len() == 0 {
                    let ops : Vec<Operation> = content
                    .split_whitespace()
                    .map(|s| if s == "+" {
                        Operation::Add
                     } else {
                        Operation::Multiply
                    })
                    .collect();

                    ops.iter().for_each(|o| match o {
                        Operation::Add => println!("Add"),
                        Operation::Multiply => println!("Mult")
                    });
                }

                
            },

            Err(e) => println!("Error parsing line {}", e)
        }
    }


    println!("Hello, world!");

    Ok(())
}
