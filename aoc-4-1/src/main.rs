use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let mut res = 0;
    for l in br.lines().map(|l| l.unwrap()) {
        let mut words = l.split(' ').collect::<Vec<_>>();

        let orig_len = words.len();

        words.sort();
        words.dedup_by(|a, b| a == b);
        
        if orig_len == words.len() {
            res += 1;
        }

        println!("{}", res);
    }
}