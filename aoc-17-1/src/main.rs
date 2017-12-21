#![feature(inclusive_range_syntax)]

use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let step = br.lines().next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut current_pos = 0;
    let mut spinlock = Vec::new();
    spinlock.push(0);
    
    for i in 1..=2017 {
        current_pos = ((current_pos + step) % i) + 1;
        spinlock.insert(current_pos, i);
    }

    println!("{}", spinlock[current_pos + 1 % 2018]);
}
