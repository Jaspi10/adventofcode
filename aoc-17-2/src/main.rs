#![feature(inclusive_range_syntax)]

use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let step = br.lines().next().unwrap().unwrap().parse::<usize>().unwrap();

    let mut current_pos = 0;
    let mut len = 1;
    let mut res = 0;
    
    for i in 1..=50_000_000 {
        current_pos = ((current_pos + step) % len) + 1;
        len += 1;
        
        if current_pos == 1 {
            res = i;
        }
    }

    println!("{}", res);
}
