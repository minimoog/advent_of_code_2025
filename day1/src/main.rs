use std::{fs::File, io::{self, BufRead}};

fn main() {
    let mut dial: i32 = 50;
    let mut count = 0;
    let mut count2 = 0;
    let file = File::open("puzzle.txt").expect("Failed to open file");

    let reader = io::BufReader::new(file);

    for (idx, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => {
                let first_char = content.chars().next().unwrap(); //unsafe

                let len = first_char.len_utf8();
                let rest = &content[len..];

                let number = rest.parse::<i32>().ok().unwrap();
                let delta = match first_char { 'R' => number, 'L' => -number, _ => 0 };

                let before = dial.div_euclid(100);
                let after = (dial + delta).div_euclid(100);
                count2 += (after - before).abs();

                dial = (dial + delta).rem_euclid(100);

                if dial == 0 { count += 1; }
            },
            Err(e) => eprintln!("Error on line {}: {}", idx + 1, e),
        }
    }

    println!("{} {}", count, count2);
}
