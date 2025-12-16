use std::{fs::File, io::{self, BufRead}};

#[derive(Debug)]
struct Range {
    lower: u64,
    higher: u64,
}

impl Range {
    fn in_range(&self, id: u64) -> bool {
        id >= self.lower && id <= self.higher
    }
}

fn main() -> std::io::Result<()> {
    let mut fresh_counter = 0;

    let file = File::open("puzzle.input")?;
    let reader = io::BufReader::new(file);

    let mut ranges: Vec<Range> = Vec::new();
    let mut ids: Vec<u64> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(content) => {
                if content.contains("-") {
                    let parts: Vec<_> = content.split("-").collect();
                    let left: u64 = parts[0].parse().unwrap(); // fix this
                    let right: u64 = parts[1].parse().unwrap(); // fix this

                    if parts.len() == 2 {
                        ranges.push(Range{ lower: left, higher: right});
                    }
                }
                else if content.len() == 0 {
                    continue;
                }
                else {
                    let id: u64 = content.parse().unwrap();

                    ids.push(id);
                }
            },
            Err(e) => {
                println!("{}", e);
            }
        }
    }

    ranges.iter().for_each(|r| println!("{} - {}", r.lower, r.higher));

    ids.iter().for_each(|r| println!("{}", r));

    for id in ids {
        let c = ranges.iter().filter(|r| r.in_range(id)).count();

        if c != 0 {
            fresh_counter += 1;
        }
    }

    println!("{}", fresh_counter);

    Ok(())
}
