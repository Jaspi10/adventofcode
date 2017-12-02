use std::io::{stdin, BufRead, BufReader};
use std::u32;
use std::cmp::{min, max};

fn main() {
    let br = BufReader::new(stdin());

    let mut sum = 0;

    for l in br.lines().map(|l| l.unwrap()) {
        let mut minimum = u32::MAX;
        let mut maximum : u32 = 0;
        for part in l.split('\u{0009}') {
            let number = part.parse::<u32>().unwrap();

            maximum = max(maximum, number);
            minimum = min(minimum, number);
        }

        println!("min: {}, max: {}", minimum, maximum);

        sum += maximum - minimum;

        println!("sum: {}", sum);
    }
}
