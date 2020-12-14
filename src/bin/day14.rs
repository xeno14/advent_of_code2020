use aoc::read_lines;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

/// part1
/// value:  10101011
/// mask:   XXXXX1X0
///
/// mask1:  11111010
/// &:      10101010
///
/// mask2:  00000100
/// |:      10101110
#[derive(Debug, Clone, Copy, PartialEq)]
struct Mask {
    mask1: u64, // 1 if X in the mask
    mask2: u64, // 1 if 1 in the mask
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

    /// replace Xs in a mask with bits of n
    /// e.g. mask1=11001, n=010 => result=01000
    fn gen_mask(mask1: u64, n: u64) -> u64 {
        let mut result = 0;
        let mut base = 1;
        let mut mask = mask1;
        let mut n = n;
        while mask > 0 {
            if (mask & 1) == 1 {
                result += base * (n & 1);
                n >>= 1;
            }
            mask >>= 1;
            base <<= 1;
        }
        result
    }

    pub fn apply2(&self, addr: u64) -> Vec<u64> {
        // set 1 if 1 in the mask
        let addr = addr | self.mask2;

        // generate all the possible conbinations
        let k = self.mask1.count_ones(); // # of X
        let result: Vec<u64> = (0..(1 << k))
            .map(|i| {
                // clear bits to be masked
                let a = addr & (!self.mask1);
                // generate mask from i
                let mask = Self::gen_mask(self.mask1, i);
                let a = a | mask;
                a
            })
            .collect();
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

#[derive(Debug, Clone, Copy, PartialEq)]
enum Input {
    Mask { mask: Mask },
    Mem { addr: u64, value: u64 },
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
            let addr: u64 = caps.get(1).unwrap().as_str().parse().unwrap();
            let value: u64 = caps.get(2).unwrap().as_str().parse().unwrap();
            Input::Mem { addr, value }
        } else {
            return Err(format!("unexpected string {}", s));
        };
        Ok(input)
    }
}

fn main() {
    // part1
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
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask::new();
    for input in inputs.iter() {
        match input {
            Input::Mask { mask: mask_ } => {
                mask = *mask_;
            }
            Input::Mem { addr, value } => {
                memory.insert(*addr, mask.apply(*value));
            }
        }
    }
    let ans: u64 = memory.values().sum();
    println!("part1 {}", ans);

    // find maximum number of X
    // let max_X = inputs.iter().map(|input| match input {
    //     Input::Mask {mask: mask_} => {
    //         let n = mask_.mask1.count_ones();
    //         Some(n)
    //     }
    //     _ => None
    // })
    // .filter(|x| x.is_some())
    // .map(|x| x.unwrap())
    // .max();
    // println!("{:?}", max_X);

    // part2
    let mask = "000000000000000000000000000000X1001X"
        .parse::<Mask>()
        .unwrap();
    assert_eq!(vec![26, 27, 58, 59], mask.apply2(42));
    let mask = "00000000000000000000000000000000X0XX"
        .parse::<Mask>()
        .unwrap();
    assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], mask.apply2(26));

    // let filename = "input/day14-example2.txt";
    let filename = "input/day14.txt";
    let inputs: Vec<Input> = read_lines(filename)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask = Mask::new();
    for input in inputs.iter() {
        match input {
            Input::Mask { mask: mask_ } => {
                mask = *mask_;
            }
            Input::Mem { addr, value } => {
                let addrs = mask.apply2(*addr);
                for addr in addrs.into_iter() {
                    memory.insert(addr, *value);
                }
            }
        }
    }
    let ans: u64 = memory.values().sum();
    println!("part2 {}", ans);
}
