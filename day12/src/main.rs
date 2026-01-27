#[derive(Debug, Clone)]
struct Shape {
    grid: Vec<Vec<bool>>,
}

impl Shape {
    fn from_lines(lines: &[&str]) -> Self {
        let grid = lines.iter()
            .map(|line| {
                line.chars()
                    .map(|c| c == '#')
                    .collect()
            })
            .collect();
        Shape { grid }
    }
    
    fn area(&self) -> usize {
        self.grid.iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell)
            .count()
    }
}

#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    present_counts: Vec<usize>,
}

fn parse_input(input: &str) -> (Vec<Shape>, Vec<Region>) {
    let lines: Vec<&str> = input.lines().collect();
    
    // Find where shapes end and regions begin
    let mut shapes_end = 0;
    for (i, line) in lines.iter().enumerate() {
        if line.contains('x') && line.contains(':') {
            shapes_end = i;
            break;
        }
    }
    
    // Parse shapes
    let mut shapes = Vec::new();
    let mut current_shape_lines = Vec::new();
    
    for line in &lines[..shapes_end] {
        if line.trim().is_empty() {
            if !current_shape_lines.is_empty() {
                shapes.push(Shape::from_lines(&current_shape_lines));
                current_shape_lines.clear();
            }
        } else if line.contains(':') && !line.contains('x') {
            // This is a shape index line, skip it
            continue;
        } else {
            current_shape_lines.push(*line);
        }
    }
    
    // Don't forget the last shape
    if !current_shape_lines.is_empty() {
        shapes.push(Shape::from_lines(&current_shape_lines));
    }
    
    // Parse regions
    let mut regions = Vec::new();
    for line in &lines[shapes_end..] {
        if line.trim().is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }
        
        // Parse dimensions (e.g., "4x4")
        let dims: Vec<&str> = parts[0].trim().split('x').collect();
        let width = dims[0].parse().unwrap();
        let height = dims[1].parse().unwrap();
        
        // Parse counts
        let present_counts: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        regions.push(Region {
            width,
            height,
            present_counts,
        });
    }
    
    (shapes, regions)
}

fn can_fit_simple(region: &Region, shapes: &[Shape]) -> bool {
    let region_area = region.width * region.height;
    
    // Calculate total area needed
    let needed_area: usize = shapes.iter()
        .zip(region.present_counts.iter())
        .map(|(shape, &count)| shape.area() * count)
        .sum();
    
    needed_area <= region_area
}

fn solve_part1(input: &str) -> usize {
    let (shapes, regions) = parse_input(input);
    
    regions.iter()
        .filter(|region| can_fit_simple(region, &shapes))
        .count()
}

fn main() {    
    let my_input = std::fs::read_to_string("puzzle.input").expect("missing file");
    
    println!("Part 1 Answer: {}", solve_part1(&my_input));
}
