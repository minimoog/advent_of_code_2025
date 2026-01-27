use std::collections::{HashMap};
struct Tree {
    graph: HashMap<String, Vec<String>>,
}

impl Tree {
    fn from_string(input: &str) -> Tree {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for line in input.lines() {
            let parts: Vec<&str> = line.split(':').collect();

            let node = parts[0].trim().to_string();

            let neighbors = parts[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            graph.insert(node, neighbors);
        }


        Tree { graph }
    }

    fn count_paths(&self, current: &str, to: &str) -> usize {
        if current == to {
            return 1;
        }

        if let Some(neighbors) = self.graph.get(current) {
            return neighbors.iter()
                .map(|n| self.count_paths(n, to) )
                .sum()

        } else {
            0
        }
    }

    fn count_paths_required(&self, current: &str, to: &str, required: &[&str], visited_required: Vec<bool>) -> usize {
        // Mark current node if it's required
        let mut visited = visited_required;

        for (i, &req) in required.iter().enumerate() {
            if current == req {
                visited[i] = true;
            }
        }
        
        // If we reached target, check if all required nodes were visited
        if current == to {
            return if visited.iter().all(|&v| v) { 1 } else { 0 };
        }
        
        // Continue DFS
        if let Some(neighbors) = self.graph.get(current) {
            neighbors.iter()
                .map(|neighbor| {
                    self.count_paths_required(
                        neighbor, 
                        to, 
                        required, 
                        visited.clone()
                    )
                })
                .sum()
        } else {
            0
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle.input").expect("input missing");

    let graph = Tree::from_string(&input);
    //let count = graph.count_paths("you", "out");

    let required = vec!["dac", "fft"];
    let initial_visited = vec![false; required.len()];
    
    let count2 = graph.count_paths_required(
        "svr", 
        "out", 
        &required, 
        initial_visited
    );

    println!("path count: {}", count2);
}
