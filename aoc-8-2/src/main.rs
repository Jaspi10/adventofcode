use std::io::{stdin, BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::max;

enum Condition {
    LT,
    LE,
    GT,
    GE,
    EQ,
    NQ,
}

impl Condition {
    fn parse(input: &str) -> Condition {
        let mut chars = input.chars();

        match chars.next() {
            Some('<') => match chars.next() {
                Some('=') => Condition::LE,
                None => Condition::LT,
                _ => panic!(),
            },

            Some('>') => match chars.next() {
                Some('=') => Condition::GE,
                None => Condition::GT,
                _ => panic!(),
            },

            Some('=') => match chars.next() {
                Some('=') => Condition::EQ,
                _ => panic!(),
            },

            Some('!') => match chars.next() {
                Some('=') => Condition::NQ,
                _ => panic!(),
            },

            _ => panic!(),
        }
    }
}

struct Registers {
    regs: HashMap<String, i32>,
}

impl Registers {
    fn get_reg_value(&self, reg: &String) -> i32 {
        match self.regs.get(reg) {
            Some(a) => *a,
            None => 0,
        }
    }

    fn set_reg_value(&mut self, reg: &String, value: i32) {
        self.regs.insert(reg.clone(), value);
    }

    fn check_condition(&self, reg: &String, condition: Condition, imm: i32) -> bool {
        let reg_value = self.get_reg_value(reg);

        match condition {
            Condition::LT => reg_value < imm,
            Condition::LE => reg_value <= imm,
            Condition::GT => reg_value > imm,
            Condition::GE => reg_value >= imm,
            Condition::EQ => reg_value == imm,
            Condition::NQ => reg_value != imm,
        }
    }
}

fn main() {
    let br = BufReader::new(stdin());

    let mut regs = Registers {
        regs: HashMap::new(),
    };

    let mut largest = 0;
    for line in br.lines().map(|l| l.unwrap()) {
        let parts = line.split(' ').map(|s| s.to_string()).collect::<Vec<String>>();

        let cond_reg = &parts[4];
        let cond_imm = parts[6].parse().unwrap();
        let cond = Condition::parse(&parts[5]);

        if regs.check_condition(cond_reg, cond, cond_imm) {
            let sign = if parts[1] == "inc" {
                1
            } else {
                -1
            };

            let reg = &parts[0];
            let reg_value = regs.get_reg_value(reg);
            let imm: i32 = parts[2].parse().unwrap();

            regs.set_reg_value(reg, reg_value + imm*sign);

            largest = max(reg_value + imm*sign, largest);
        }
    }

    println!("{}", largest);
}
