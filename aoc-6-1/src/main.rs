use std::io::{stdin, BufRead, BufReader};

#[derive(Clone, PartialEq, Eq)]
struct Memory {
    banks: Vec<u32>,
}

impl Memory {

    fn from_string(string: &str) -> Memory {
        Memory {
            banks: string.split('\u{0009}').map(|a| a.parse::<u32>().unwrap()).collect(),
        }
    }

    fn cycle(&mut self) {
        let mut highest_bank = 0;
        
        let len = self.banks.len();
        for i in 0..len {
            if self.banks[i] > self.banks[highest_bank] {
                highest_bank = i;
            }
        }

        let mut resources = self.banks[highest_bank];
        self.banks[highest_bank] = 0;

        let for_each: u32 = resources / len as u32;
        self.banks.iter_mut().for_each(|x| *x += for_each);
        resources -= for_each * len as u32;

        let mut current_bank = highest_bank;
        while resources > 0 {
            current_bank = (current_bank + 1) % len;
            self.banks[current_bank] += 1;
            resources -= 1;
        } 
    }

}

fn main() {
    let br = BufReader::new(stdin());

    let line = br.lines().next().unwrap().unwrap();
    let mut memory = Memory::from_string(line.as_str());

    let mut prior_memory = Vec::new();
    let mut cycles = 0;
    loop {
        memory.cycle();
        cycles += 1;

        if prior_memory.contains(&memory) {
            break;
        }
        prior_memory.push(memory.clone());
    }

    println!("{}", cycles);
}
