use std::fs;
use std::collections::BTreeSet;

fn main() -> std::io::Result<()> {
    // --- Step 1: Read input ---
    let contents = fs::read_to_string("puzzle.input")?;
    let mut points: Vec<(i32, i32)> = Vec::new();

    for line in contents.lines() {
        let mut parts = line.split(',');
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let y: i32 = parts.next().unwrap().parse().unwrap();
        points.push((x, y));
    }

    // --- Step 2: Coordinate compression ---
    let mut unique_coords: BTreeSet<i32> = BTreeSet::new();
    for &(x, y) in &points {
        unique_coords.insert(x);
        unique_coords.insert(y);
    }
    let unique: Vec<i32> = unique_coords.into_iter().collect();
    let coord_to_idx = |v: i32| unique.iter().position(|&u| u == v).unwrap();

    let mut x: Vec<usize> = points.iter().map(|p| coord_to_idx(p.0)).collect();
    let mut y: Vec<usize> = points.iter().map(|p| coord_to_idx(p.1)).collect();

    let max_x = *x.iter().max().unwrap();
    let max_y = *y.iter().max().unwrap();

    // --- Step 3: Initialize matrix ---
    let mut s = vec![vec![0u8; max_x + 1]; max_y + 1];
    for i in 0..points.len() {
        s[y[i]][x[i]] = 1;
    }

    // --- Step 4: Fill edges between points ---
    for i in 0..points.len() {
        for j in 0..points.len() {
            if i != j {
                if x[i] == x[j] {
                    let ys = if y[i] < y[j] { y[i]..=y[j] } else { y[j]..=y[i] };
                    for k in ys { s[k][x[i]] = 1; }
                }
                if y[i] == y[j] {
                    let xs = if x[i] < x[j] { x[i]..=x[j] } else { x[j]..=x[i] };
                    for k in xs { s[y[i]][k] = 1; }
                }
            }
        }
    }

    // --- Step 5: Fill interior rows between edges ---
    for i in 0..=max_y {
        let mut border: Vec<usize> = Vec::new();
        for j in 0..=max_x {
            if s[i][j] != 0 { border.push(j); }
        }
        if border.len() >= 2 {
            let start = *border.first().unwrap();
            let end = *border.last().unwrap();
            for j in start..=end {
                if s[i][j] == 0 { s[i][j] = 1; }
            }
        }
    }

    // --- Step 6: Brute-force rectangle search ---
    let mut rectangles: Vec<(usize, usize, usize, usize, u64)> = Vec::new();
    let n = points.len();
    for i in 0..n {
        for j in i+1..n {
            let x1 = x[i].min(x[j]);
            let x2 = x[i].max(x[j]);
            let y1 = y[i].min(y[j]);
            let y2 = y[i].max(y[j]);
            if x1 == x2 || y1 == y2 { continue; }

            let n_cells = (1u64 + unique[x2] as u64 - unique[x1] as u64)
                        * (1u64 + unique[y2] as u64 - unique[y1] as u64);
            rectangles.push((x1, y1, x2, y2, n_cells));
        }
    }

    rectangles.sort_by(|a, b| b.4.cmp(&a.4)); // sort by area descending

    // Maximum possible area (largest rectangle by coordinates)
    let p1 = rectangles[0].4;

    // --- Step 7: Check each rectangle in the matrix ---
    let mut p2: u64 = 0;
    for &(x1, y1, x2, y2, area) in &rectangles {
        let mut ok = true;
        for j in y1..=y2 {
            for i in x1..=x2 {
                if s[j][i] == 0 {
                    ok = false;
                    break;
                }
            }
            if !ok { break; }
        }
        if ok {
            p2 = area;
            break;
        }
    }

    println!("p1: {}, p2: {}", p1, p2);

    Ok(())
}
