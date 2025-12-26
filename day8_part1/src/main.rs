use std::{collections::HashSet, fs};

struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

struct JunctionDistance {

}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("test.input")?;

    let junctionBoxes: Vec<JunctionBox> = input.lines().map(|s| {
        let splitted: Vec<&str> = s.split(",").collect();

        let x = splitted[0].parse::<u32>().unwrap();
        let y = splitted[1].parse::<u32>().unwrap();
        let z = splitted[2].parse::<u32>().unwrap();
        
        JunctionBox{x, y, z}

    })
    .collect();

    //make sets
    let mut junctionSets: Vec<HashSet<usize>> = junctionBoxes
        .iter()
        .enumerate()
        .map(|(i, _)| {
            HashSet::from([i])
        })
        .collect();

    Ok(())
}
