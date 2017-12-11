use std::io::{BufRead, BufReader};
use std::fs::File;
use std::cmp::{max, min};

fn main() {
    let br = BufReader::new(File::open("input").unwrap());

    let mut n: i32 = 0;
    let mut se: i32 = 0;
    let mut sw: i32 = 0;

    let mut res = 0;

    for direction in br.lines().next().unwrap().unwrap().split(',') {
        match direction {
            "n" => n += 1,
            "s" => n -= 1,
            "se" => se += 1,
            "nw" => se -= 1,
            "sw" => sw += 1,
            "ne" => sw -= 1,
            _ => panic!(),
        }

        let r1 = (se - n).abs() + (sw - n).abs();
        let r2 = (n - se).abs() + (sw - se).abs();
        let r3 = (n - sw).abs() + (se - sw).abs();

        res = max(min(min(r1, r2), r3), res);
    }

    println!("{}", res);
}
