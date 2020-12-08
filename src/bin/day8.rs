use aoc::read_lines;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

enum Instruction {
    Acc { delta: i64 },
    Jmp { delta: i64 },
    Nop,
}

impl FromStr for Instruction {
    type Err = std::num::ParseIntError;

    /// nop +0
    /// acc +1
    /// jmp +4
    fn from_str(s: &str) -> Result<Instruction, Self::Err> {
        let re = Regex::new(r"([a-z]+) (.\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let delta: i64 = caps.get(2).unwrap().as_str().parse()?;
        let ins = match caps.get(1).unwrap().as_str() {
            "nop" => Instruction::Nop,
            "acc" => Instruction::Acc { delta },
            "jmp" => Instruction::Jmp { delta },
            _ => panic!("unknown instruction"),
        };
        Ok(ins)
    }
}

struct Game {
    acc: i64,
    ip: usize, // instruction pointer
}

impl Game {
    fn process(&mut self, inst: &Instruction) -> &Game {
        match inst {
            Instruction::Nop => {
                self.ip += 1;
                self
            },
            Instruction::Acc { delta } => {
                self.acc += delta;
                self.ip += 1;
                self
            }
            Instruction::Jmp { delta } => {
                self.ip = if *delta < 0 { 
                    self.ip - delta.abs() as usize
                }
                else {
                    self.ip + *delta as usize
                };
                self
            }
        }
    }
}

fn main() {
    let filename = "input/day8.txt";
    let instructions: Vec<Instruction> = read_lines(filename)
        .map(|line| Instruction::from_str(line.as_ref().unwrap()))
        .filter_map(Result::ok)
        .collect();
    let mut seen: HashSet<usize> = HashSet::new();
    seen.insert(0);
    let mut game = Game { acc: 0, ip: 0 };
    loop {
        let inst = &instructions[game.ip];
        game.process(inst);
        if seen.contains(&game.ip) {
            println!("acc={}", game.acc);
            break;
        }
        seen.insert(game.ip);
    }

    // for ins in instjj
    //     println!("{}", ins);
    // }
}
