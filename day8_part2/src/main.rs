use std::fs;

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
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }

        let mut node = x;
        while self.parent[node] != node {
            let next = self.parent[node];
            self.parent[node] = root;
            node = next;
        }

        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb { return; }

        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] {
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


    let mut last_edge = None;

    for &(_, i, j) in &distances {
        if junction_sets.find(i) != junction_sets.find(j) {
            junction_sets.union(i, j);

            last_edge = Some((i, j));
        }
    }

    let (a, b) = last_edge.unwrap();
    let answer = junction_boxes[a].x * junction_boxes[b].x;
    println!("{answer}");

    Ok(())
}
