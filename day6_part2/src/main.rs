use std::{fs::File, io::{self, BufRead}};

#[derive(Copy, Clone)]
struct Interval {
    lower: usize,
    higher: usize,
    ops: char,
}

fn main() -> std::io::Result<()> {
    let file = File::open("puzzle.input")?;
    let mut reader = io::BufReader::new(file);

    let mut line1_string = String::new();
    let mut line2_string = String::new();
    let mut line3_string = String::new();
    let mut line4_string = String::new();
    let mut ops_line = String::new();

    let _ = reader.read_line(&mut line1_string).unwrap();
    let _ = reader.read_line(&mut line2_string).unwrap();
    let _ = reader.read_line(&mut line3_string).unwrap();
    let _ = reader.read_line(&mut line4_string).unwrap();
    let _ = reader.read_line(&mut ops_line).unwrap();

    let line1 = line1_string.as_bytes();
    let line2 = line2_string.as_bytes();
    let line3 = line3_string.as_bytes();
    let line4 = line4_string.as_bytes();

    println!("{}{}{}{}", line1_string, line2_string, line3_string, line4_string);

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

    let mut part2 = 0;

    for interval in intervals {
        let mut sum_or_prod = 0;

        if interval.ops == '*' { sum_or_prod = 1; }

        for i in interval.lower..=interval.higher {
            let mut n: u64 = 0;


            if line1[i] != b' ' {
                println!("{}", line1[i]);
                n *= 10;
                n += (line1[i] - b'0') as u64;
            }

            if line2[i] != b' ' {
                n *= 10;
                n += (line2[i] - b'0') as u64;
            }

            if line3[i] != b' ' {
                n *= 10;
                n += (line3[i] - b'0') as u64;
            }

            if line4[i] != b' ' {
                n *= 10;
                n += (line4[i] - b'0') as u64;
            }

            if interval.ops == '+' {
                sum_or_prod += n;
            } else {
                sum_or_prod *= n;
            }
        }

        //println!("sum or prod {}", sum_or_prod);

        part2 += sum_or_prod;

    }

    println!("{}", part2);

    Ok(())
}