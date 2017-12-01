use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());
    let mut line = String::new();

    br.read_line(&mut line).unwrap();

    let mut res = 0;
    let len = line.len();

    let mut last = line.chars().rev().nth(2).unwrap().to_digit(10).unwrap();

    for d in line.chars().map(|c| c.to_digit(10)).filter(|o| o.is_some()).map(|o| o.unwrap()) {
        if d == last {
            res += d;
        }

        last = d;
    }

    println!("{}", res);
}
