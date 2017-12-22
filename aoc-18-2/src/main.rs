use std::io::{stdin, BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

struct CPU {
    memory: Vec<Command>,
    pc: isize,
    regs: HashMap<char, i64>,
    cycles: u64,
    sends: u64,
}

impl CPU {
    fn new(memory: Vec<Command>, id: i64) -> CPU {
        let mut hash_map = HashMap::new();
        hash_map.insert('p', id);
        
        CPU {
            memory: memory,
            pc: 0,
            regs: hash_map,
            cycles: 0,
            sends: 0,
        }
    }

    fn execute(&mut self, in_queue: &mut VecDeque<i64>, out_queue: &mut VecDeque<i64>) -> u64 {
        loop {

            println!("{:?}", self.regs);

            match self.memory[self.pc as usize].clone() {
                Command::Snd(ref a1) => {
                    out_queue.push_back(self.get_value(&a1));
                    self.sends += 1;
                },

                Command::Set(ref a1, ref a2) => {
                    let tmp = self.get_value(&a2);
                    self.set_value(&a1, tmp);
                },

                Command::Add(ref a1, ref a2) => {
                    let res = self.get_value(&a1) + self.get_value(&a2);
                    self.set_value(&a1, res);
                },

                Command::Mul(ref a1, ref a2) => {
                    let res = self.get_value(&a1) * self.get_value(&a2);
                    self.set_value(&a1, res);
                },

                Command::Mod(ref a1, ref a2) => {
                    let res = self.get_value(&a1) % self.get_value(&a2);
                    self.set_value(&a1, res);
                },

                Command::Rcv(ref a1) => {
                    if !in_queue.is_empty() {
                        self.set_value(&a1, in_queue.pop_front().unwrap());
                    } else {
                        return self.cycles;
                    }
                },

                Command::Jgz(ref a1, ref a2) => {
                    if self.get_value(&a1) > 0 {
                        self.pc += (self.get_value(&a2) - 1) as isize;
                    }
                },
            }

            self.pc += 1;
            self.cycles += 1;
        }
    }

    fn get_value(&self, arg: &Argument) -> i64 {
        match arg {
            &Argument::Register(c) => *self.regs.get(&c).unwrap_or(&0),
            &Argument::Value(v) => v, 
        }
    }

    fn set_value(&mut self, arg: &Argument, value: i64) {
        match arg {
            &Argument::Register(c) => self.regs.insert(c, value),
            &Argument::Value(_) => panic!(), 
        };
    }
}

#[derive(Clone, Copy)]
enum Command {
    Snd(Argument),
    Set(Argument, Argument),
    Add(Argument, Argument),
    Mul(Argument, Argument),
    Mod(Argument, Argument),
    Rcv(Argument),
    Jgz(Argument, Argument),
}

impl Command {
    fn parse(text: &str) -> Command {
        let mut split = text.split(' ');

        match split.next().unwrap() {
            "snd" => Command::Snd(Argument::parse(split.next().unwrap())),
            "set" => Command::Set(Argument::parse(split.next().unwrap()), Argument::parse(split.next().unwrap())),
            "add" => Command::Add(Argument::parse(split.next().unwrap()), Argument::parse(split.next().unwrap())),
            "mul" => Command::Mul(Argument::parse(split.next().unwrap()), Argument::parse(split.next().unwrap())),
            "mod" => Command::Mod(Argument::parse(split.next().unwrap()), Argument::parse(split.next().unwrap())),
            "rcv" => Command::Rcv(Argument::parse(split.next().unwrap())),
            "jgz" => Command::Jgz(Argument::parse(split.next().unwrap()), Argument::parse(split.next().unwrap())),
            &_ => panic!(),
        }
    }
}

#[derive(Clone, Copy)]
enum Argument {
    Register(char),
    Value(i64),
}

impl Argument {
    fn parse(text: &str) -> Argument {
        match text.parse::<i64>() {
            Ok(v) => Argument::Value(v),
            Err(_) => Argument::Register(text.chars().next().unwrap()),
        }
    }
}

fn main() {
    let br = BufReader::new(stdin());

    let mut buf = Vec::new();
    for line in br.lines().map(|l| l.unwrap()) {
        buf.push(Command::parse(&line));
    }

    let mut cpu0 = CPU::new(buf.clone(), 0);
    let mut cpu1 = CPU::new(buf, 1);

    let mut queue0 = VecDeque::new();
    let mut queue1 = VecDeque::new();

    let mut cpu0_alive = true;
    let mut cpu1_alive = true;

    while cpu0_alive || cpu1_alive {
        let cpu0_cycles = cpu0.cycles;
        cpu0_alive = cpu0.execute(&mut queue0, &mut queue1) != cpu0_cycles;

        let cpu1_cycles = cpu1.cycles;
        cpu1_alive = cpu1.execute(&mut queue1, &mut queue0) != cpu1_cycles;
    }

    println!("{}", cpu1.sends);
}
