use aoc::read_lines;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Instruction {
    Acc { delta: i64 },
    Jmp { delta: i64 },
    Nop { delta: i64 },
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
            "nop" => Instruction::Nop { delta },
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
    fn step(&mut self, inst: &Instruction) -> &Game {
        match inst {
            Instruction::Nop { delta: _ } => {
                self.ip += 1;
                self
            }
            Instruction::Acc { delta } => {
                self.acc += delta;
                self.ip += 1;
                self
            }
            Instruction::Jmp { delta } => {
                self.ip = if *delta < 0 {
                    self.ip - delta.abs() as usize
                } else {
                    self.ip + *delta as usize
                };
                self
            }
        }
    }

    fn run(instructions: &Vec<Instruction>) -> Game {
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(0);
        let mut game = Game { acc: 0, ip: 0 };
        loop {
            let inst = &instructions[game.ip];
            game.step(inst);
            if seen.contains(&game.ip) || game.ip == instructions.len() {
                break;
            }
            seen.insert(game.ip);
        }
        game
    }
}

fn main() {
    // let filename = "input/day8-example.txt";
    let filename = "input/day8.txt";

    // part1
    let instructions: Vec<Instruction> = read_lines(filename)
        .map(|line| Instruction::from_str(line.as_ref().unwrap()))
        .filter_map(Result::ok)
        .collect();
    let game = Game::run(&instructions);
    println!("{}", game.acc);

    // part2
    let mut instructions: Vec<Instruction> = read_lines(filename)
        .map(|line| Instruction::from_str(line.as_ref().unwrap()))
        .filter_map(Result::ok)
        .collect();
    for i in 0..instructions.len() {
        let inst = instructions[i].clone();
        // replace
        instructions[i] = match inst {
            Instruction::Nop { delta } => Instruction::Jmp { delta },
            Instruction::Jmp { delta } => Instruction::Nop { delta },
            _ => {
                continue;
            }
        };
        // simulate
        let game = Game::run(&instructions);
        if game.ip == instructions.len() {
            break;
        }

        // revert
        instructions[i] = inst;
    }
    let game = Game::run(&instructions);
    println!("{}", game.acc);
}
