use aoc::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

/// value:  10101011
/// mask:   XXXXX1X0
///
/// mask1:  11111010
/// &:      10101010
///
/// mask2:  00000100
/// |:      10101110
struct Mask {
    mask1: u64, // 1 if X
    mask2: u64, // 1 if 1
}

impl Mask {
    pub fn new() -> Mask {
        Mask {
            mask1: -1i64 as u64,
            mask2: 0,
        }
    }

    pub fn apply(&self, value: u64) -> u64 {
        // set all the bits affected by the mask to 0
        let mut result = value & self.mask1;
        // set mask bits
        result |= self.mask2;
        result
    }
}

impl FromStr for Mask {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask1: u64 = 0;
        let mut mask2: u64 = 0;
        for c in s.chars() {
            mask1 = mask1 << 1;
            mask2 = mask2 << 1;
            match c {
                'X' => mask1 |= 1,
                '1' => mask2 |= 1,
                _ => (),
            };
        }
        Ok(Mask { mask1, mask2 })
    }
}

enum Input {
    Mask { mask: Mask },
    Mem { addr: usize, value: u64 },
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input: Input = if s.starts_with("mask") {
            let caps = Regex::new(r"mask = (.+)")
                .map_err(|e| e.to_string())?
                .captures(s)
                .ok_or(format!("unable to capture mask '{}'", s))?;
            let mask: Mask = caps
                .get(1)
                .ok_or(format!("unable to get mask '{}'", s))?
                .as_str()
                .parse()?;
            Input::Mask { mask }
        } else if s.starts_with("mem") {
            let caps = Regex::new(r"mem\[(\d+)\] = ([\d]+)")
                .map_err(|e| e.to_string())?
                .captures(s)
                .ok_or(format!("unable to capture mem'{}'", s))?;
            let addr: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let value: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            Input::Mem { addr, value }
        } else {
            return Err(format!("unexpected string {}", s));
        };
        Ok(input)
    }
}

fn main() {
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
        .parse::<Mask>()
        .unwrap();
    assert_eq!(73, mask.apply(11));
    assert_eq!(101, mask.apply(101));
    assert_eq!(64, mask.apply(0));

    // let filename = "input/day14-example.txt";
    let filename = "input/day14.txt";
    let inputs: Vec<Input> = read_lines(filename)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = Mask::new();
    for input in inputs {
        match input {
            Input::Mask { mask: mask_ } => {
                mask = mask_;
            }
            Input::Mem { addr, value } => {
                memory.insert(addr, mask.apply(value));
            }
        }
    }
    let ans: u64 = memory.values().sum();
    println!("{}", ans);
}
