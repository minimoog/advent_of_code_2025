use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let input: i64 = std::fs::read_to_string("puzzle.input")?
        .lines()
        .map(|x| {
            let splitted: Vec<&str> = x.split(",").collect();

            let x = splitted[0].parse::<i64>().unwrap();
            let y = splitted[1].parse::<i64>().unwrap();

            (x, y)
        })
        .tuple_combinations()
        .map(|((x1, y1), (x2, y2))| {
            let dx = (x1 - x2).abs() + 1;
            let dy = (y1 - y2).abs() + 1;

            dx * dy
        })
        .max()
        .unwrap();

    
    println!("{}", input);
    
    Ok(())
}
