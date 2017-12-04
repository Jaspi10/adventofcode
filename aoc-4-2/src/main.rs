use std::io::{stdin, BufRead, BufReader};

fn main() {
    let br = BufReader::new(stdin());

    let mut res = 0;
    for l in br.lines().map(|l| l.unwrap()) {
        let mut words = l.split(' ').map(|w| {
            let mut char_vec = w.chars().collect::<Vec<char>>();
            char_vec.sort();
            char_vec
        }).collect::<Vec<_>>();
        

        let orig_len = words.len();

        words.sort();
        words.dedup_by(|a, b| a == b);
        
        if orig_len == words.len() {
            res += 1;
        }

        println!("{}", res);
    }
}