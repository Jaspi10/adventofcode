#![feature(inclusive_range_syntax)]

use std::io::{stdin, BufRead, BufReader};

struct CircularList {
    data: Vec<u8>,
    length: usize,
}

impl CircularList {
    fn new(length: usize) -> CircularList {
        CircularList {
            data: (0..=(length-1) as u8).collect(),
            length: length,
        }
    }

    fn reverse_sublist(&mut self, start: usize, length: usize) {
        for i in 0..length/2 {
            self.swap(start+i, (start+length-1)-i); 
        }
    }

    fn swap(&mut self, mut a: usize, mut b: usize) {
        a = a % self.length;
        b = b % self.length;

        let tmp = self.data[a];
        self.data[a] = self.data[b];
        self.data[b] = tmp; 
    }
}

struct KnotHash {
    state: CircularList,
    position: usize,
    skip: usize,
}

impl KnotHash {
    fn new() -> KnotHash {
        KnotHash {
            state: CircularList::new(256),
            position: 0,
            skip: 0,
        }
    }

    fn calculate_hash(input: String) -> Vec<u8> {
        let suffix: [u8; 5] = [17, 31, 73, 47, 23];
        let lengths = input.as_bytes().into_iter().chain(suffix.into_iter()).map(|a| *a).collect::<Vec<u8>>();

        let mut hash = KnotHash::new();
        for _ in 0..64 {
            hash.do_round(lengths.as_slice());
        }

        hash.get_dense_hash()
    }

    fn get_dense_hash(&self) -> Vec<u8>{
        let mut ret = Vec::new();

        for i in 0..16 {
            let mut block = 0_u8;
            for j in (i*16)..(i*16)+16 {
                block = block ^ self.state.data[j];
            }
            ret.push(block);
        }

        ret
    }

    fn do_round(&mut self, lengths: &[u8]) {
        for length in lengths {
            self.state.reverse_sublist(self.position, *length as usize);
            self.position += (*length as usize) + self.skip;

            self.skip += 1;
        }
    }
}

fn main() {
    let br = BufReader::new(stdin());
    
    let input = br.lines().next().unwrap().unwrap();

    let mut res = 0;

    for row in 0..128 {
        for segment in KnotHash::calculate_hash(format!("{}-{}", input, row)) {
            res += segment.count_ones();
        } 
    }

    println!("{}", res);
}
