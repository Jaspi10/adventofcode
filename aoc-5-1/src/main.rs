use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let mut maze = br.lines().map(|l| l.unwrap().parse::<isize>().unwrap()).collect::<Vec<isize>>();
    let mut steps = 0;
    let mut pos = 0;

    loop {
        if pos >= maze.len() {
            break;
        }

        let distance = maze[pos];
        maze[pos] += 1;
        pos = ((pos as isize) + distance) as usize;
        steps += 1;
    }

    println!("{}", steps);
}
