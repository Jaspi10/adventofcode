use std::io::{stdin, BufRead, BufReader};
use std::cmp::min;
use std::u32;

fn main() {
    let br = BufReader::new(stdin());

    let input = br.lines().next().unwrap().unwrap().trim().parse::<u32>().unwrap();

    let ring = (0.5 + f64::sqrt(-0.25 + (input as f64/4.0))).floor() as u32;

    let start = (2*ring - 1) * (2*ring - 1) + 1;

    let mut m = start + ring - 1;
    let mut smallest_d = u32::MAX;

    for _ in 0..4 {
        let d = i32::abs(input as i32 - m as i32) as u32;
        smallest_d = min(smallest_d, d);
        m += 2*ring;
    }

    println!("{}", smallest_d+ring);
}
