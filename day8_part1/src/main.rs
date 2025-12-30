use std::{collections::HashMap, fs};

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n]
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }

        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let ra = self.find(a);
        let rb = self.find(b);

        if ra == rb {
            return;
        }

        if self.rank[ra] < self.rank[rb] {
            self.parent[ra] = rb;
        } else if self.rank[ra] > self.rank[rb] {
            self.parent[rb] = ra;
        } else {
            self.parent[rb] = ra;
            self.rank[ra] += 1;
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("puzzle.input")?;

    let junction_boxes: Vec<JunctionBox> = input.lines().map(|s| {
        let splitted: Vec<&str> = s.split(",").collect();

        let x = splitted[0].parse::<i64>().unwrap();
        let y = splitted[1].parse::<i64>().unwrap();
        let z = splitted[2].parse::<i64>().unwrap();
        
        JunctionBox{x, y, z}

    })
    .collect();

    //make sets
    let mut junction_sets = DSU::new(junction_boxes.len());

    //find pair distances
    let mut distances: Vec<(i64, usize, usize)> = Vec::new();  //dist^2, index, index

    for i in 0..junction_boxes.len()-1 {
        for j in i+1..junction_boxes.len() {
            let dist = (junction_boxes[j].x - junction_boxes[i].x) * (junction_boxes[j].x - junction_boxes[i].x)
                          + (junction_boxes[j].y - junction_boxes[i].y) * (junction_boxes[j].y - junction_boxes[i].y)
                          + (junction_boxes[j].z - junction_boxes[i].z) * (junction_boxes[j].z - junction_boxes[i].z);
                        
            distances.push((dist, i, j));
        }
    }

    //sort distances
    distances.sort_by(|a, b| a.0.cmp(&b.0));

    //group them
    for i in 0..1000 {
        let first = distances[i].1;
        let second = distances[i].2;

        junction_sets.union(first, second);
    }

    //find three biggest
    let mut sizes: HashMap<usize, usize> = HashMap::new();

    for i in 0..junction_boxes.len() {
        let root = junction_sets.find(i);
        *sizes.entry(root).or_insert(0) += 1;
    }
    
    let mut size_vec: Vec<usize> = sizes.values().copied().collect();
    size_vec.sort_by(|a, b| b.cmp(a));

    let largest_three = size_vec.iter().take(3).copied().collect::<Vec<_>>();

    println!("{}", largest_three[0] * largest_three[1] * largest_three[2]);

    Ok(())
}
