use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let mut nodes = Vec::new();

    for line in br.lines().map(|l| l.unwrap()) {
        let connected_to = line.split("> ").nth(1).unwrap();
        nodes.push(connected_to.split(", ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>());
    }

    let mut visited = vec![false; nodes.len()];
    let mut to_visit = Vec::new(); 
    
    let mut res = 0;
    for i in 0..visited.len() {
        if !visited[i] { 
            res += 1;

            to_visit.push(i);
            while !to_visit.is_empty() {
                let current_node = to_visit.pop().unwrap();
                
                for node in &nodes[current_node] {
                    if !visited[*node] {
                        visited[*node] = true;
                        to_visit.push(*node);
                    }
                }
            }
        }
    }

    println!("{}", res);
}
