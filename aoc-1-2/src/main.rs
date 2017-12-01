use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut br = BufReader::new(stdin());
    let mut line = String::new();

    br.read_line(&mut line).unwrap();

    let digits = line.chars().map(|c| c.to_digit(10)).filter(|o| o.is_some()).map(|o| o.unwrap()).collect::<Vec<u32>>();

    let step = len / 2;

    let mut res = 0;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + step) % len] {
            res += digits[i];
        }
    }

    println!("{}", res);
}
