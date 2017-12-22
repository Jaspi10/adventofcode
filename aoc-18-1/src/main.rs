use std::io::{stdin, BufRead, BufReader};
use std::collections::HashMap;

struct CPU {
    memory: Vec<Command>,
    pc: isize,
    regs: HashMap<char, i64>,
    freq: i64,
}

impl CPU {
    fn new(memory: Vec<Command>) -> CPU {
        CPU {
            memory: memory,
            pc: 0,
            regs: HashMap::new(),
            freq: 0,
        }
    }

    fn execute(&mut self) -> i64 {
        loop {

            println!("{:?}", self.regs);

            match self.memory[self.pc as usize].clone() {
                Command::Snd(ref a1) => {
                    self.freq = self.get_value(&a1); 
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
                    if self.get_value(&a1) != 0 {
                        return self.freq;
                    }
                },

                Command::Jgz(ref a1, ref a2) => {
                    if self.get_value(&a1) > 0 {
                        self.pc += (self.get_value(&a2) - 1) as isize;
                    }
                },
            }

            self.pc += 1;
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

    let mut cpu = CPU::new(buf);

    println!("{}", cpu.execute());
}
