use std::io::{stdin, BufRead, BufReader};

struct CircularList {
    data: Vec<usize>,
    length: usize,
}

impl CircularList {
    fn new(length: usize) -> CircularList {
        CircularList {
            data: (0..length).collect(),
            length: length,
        }
    }

    fn reverse_sublist(&mut self, start: usize, length: usize) {
        for i in 0..length/2 {
            self.swap(start+i, (start+length)-i); 
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

fn main() {
    let br = BufReader::new(stdin());
    let input = br.lines().next().unwrap().unwrap();

    let mut list = CircularList::new(5);
    let mut skip = 0;
    let mut position = 0;
    for length in input.split(',').map(|p| p.parse::<usize>().unwrap()) {
        list.reverse_sublist(position, length);
        position += length + skip;

        skip += 1;

        println!("{:?}", list.data);
    }

    println!("{}", list.data[0] * list.data[1]);
}
