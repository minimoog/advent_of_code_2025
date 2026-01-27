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

    fn count_paths_with_required(
        &self, 
        current: &str, 
        target: &str,
        required: &[&str],
        visited_mask: u64,
        memo: &mut HashMap<(String, u64), usize>,
    ) -> usize {
        // Check memo first
        let key = (current.to_string(), visited_mask);
        if let Some(&cached) = memo.get(&key) {
            return cached;
        }
        
        let mut mask = visited_mask;
        
        // Mark current node if it's required
        for (i, &req) in required.iter().enumerate() {
            if current == req {
                mask |= 1 << i;
            }
        }
        
        // If we reached target, check if all required nodes were visited
        if current == target {
            let all_required_mask = (1 << required.len()) - 1;
            let result = if mask == all_required_mask { 1 } else { 0 };
            memo.insert(key, result);
            return result;
        }
        
        // Continue DFS
        let result = if let Some(neighbors) = self.graph.get(current) {
            neighbors.iter()
                .map(|neighbor| {
                    self.count_paths_with_required(
                        neighbor, 
                        target, 
                        required, 
                        mask,
                        memo
                    )
                })
                .sum()
        } else {
            0
        };
        
        memo.insert(key, result);
        result
    }
}

fn main() {
    let input = std::fs::read_to_string("puzzle.input").expect("input missing");

    let graph = Tree::from_string(&input);
    //let count = graph.count_paths("you", "out");

    let required = vec!["dac", "fft"];
    let mut memo = HashMap::new();
    
    let count2 = graph.count_paths_with_required(
        "svr", 
        "out", 
        &required, 
        0,
        &mut memo
    );

    println!("path count: {}", count2);
}
