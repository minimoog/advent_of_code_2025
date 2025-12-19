use std::{fs::File, io::{self, BufRead}};

#[derive(Copy, Clone)]
struct Interval {
    lower: usize,
    higher: usize,
    ops: char,
}

fn main() -> std::io::Result<()> {
    let file = File::open("test.input")?;
    let mut reader = io::BufReader::new(file);

    let mut line1 = String::new();
    let mut line2 = String::new();
    let mut line3 = String::new();
    let mut ops_line = String::new();

    let _ = reader.read_line(&mut line1).unwrap();
    let _ = reader.read_line(&mut line2).unwrap();
    let _ = reader.read_line(&mut line3).unwrap();
    let _ = reader.read_line(&mut ops_line).unwrap();

    println!("{}{}{}{}", line1, line2, line3, ops_line);

    let mut intervals: Vec<Interval> = Vec::new();

    let chars: Vec<char> = ops_line.chars().collect();

    let mut interval: Interval = Interval { lower: 0, higher: 0, ops: '+' };

    for i in 0..chars.len() {
        
        if chars[i] == '+' || chars[i] == '*' {
            interval.lower = i;
            interval.ops = chars[i];
        } else if i+1 == chars.len() || chars[i+1] == '+' || chars[i+1] == '*' {
            interval.higher = i - 1;

            intervals.push(interval);
        }
    }

    //last one fix
    if let Some(last) = intervals.last_mut() {
        last.higher += 1;
    }

    for i in intervals { println!("{} {} {}", i.lower, i.higher, i.ops); }

    Ok(())
}