use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let br = BufReader::new(File::open("input").unwrap());

    let mut garbage = 0;
    let mut is_garbage = false;
    let mut skip = false;
    for c in br.lines().next().unwrap().unwrap().chars() {
        if skip {
            skip = false;
            continue;
        }

        if c == '!' {
            skip = true;
            continue;
        }

        if is_garbage {
            if c == '>' {
                is_garbage = false;
                continue;
            }
            garbage += 1;
            continue;
        }

        if c == '<' {
            is_garbage = true;
        }
    }

    println!("{}", garbage);
}
