use std::io::{stdin, BufRead, BufReader};

struct Generator {
    state: u64,
    factor: u64,
}

impl Iterator for Generator {
    type Item = u16;

    fn next(&mut self) -> Option<u16> {
        self.state = (self.state * self.factor) % 2147483647;
        
        Some((self.state & 0xff_ff) as u16)
    } 
}

fn main() {
    let br = BufReader::new(stdin());
    let mut lines = br.lines();

    let init_a = lines.next().unwrap().unwrap().split(' ').nth(4).unwrap().parse::<u64>().unwrap();
    let fac_a = 16807;

    let init_b = lines.next().unwrap().unwrap().split(' ').nth(4).unwrap().parse::<u64>().unwrap();
    let fac_b = 48271;

    let gen_a = Generator {
        state: init_a,
        factor: fac_a,
    };

    let gen_b = Generator {
        state: init_b,
        factor: fac_b,
    };

    let res: u64 = gen_a.zip(gen_b).take(40_000_000).filter(|&(a, b)| a == b).map(|(_, _)| 1).sum();

    println!("{}",  res);
}