use aoc::read_lines;
use regex::Regex;
use std::str::FromStr;

/// Action N means to move north by the given value.
/// Action S means to move south by the given value.
/// Action E means to move east by the given value.
/// Action W means to move west by the given value.
/// Action L means to turn left the given number of degrees.
/// Action R means to turn right the given number of degrees.
/// Action F means to move forward by the given value in the direction the ship is currently facing.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Action {
    North { value: u32 },
    South { value: u32 },
    East { value: u32 },
    West { value: u32 },
    Left { degree: u32 },
    Right { degree: u32 },
    Forward { value: u32 },
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, Self::Err> {
        let re = Regex::new(r"([A-Z])(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let value: u32 = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let a = match direction {
            "N" => Action::North { value },
            "S" => Action::South { value },
            "E" => Action::East { value },
            "W" => Action::West { value },
            "L" => Action::Left { degree: value },
            "R" => Action::Right { degree: value },
            "F" => Action::Forward { value },
            _ => panic!(format!("invalid action '{}'", direction)),
        };
        Ok(a)
    }
}

#[derive(Debug)]
struct Ship {
    x: i64,
    y: i64,
    u: i64,
    v: i64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            u: 1,
            v: 0,
        }
    }

    pub fn tick(&mut self, action: Action) {
        match action {
            Action::North { value } => {
                self.y += value as i64;
            }
            Action::South { value } => {
                self.y -= value as i64;
            }
            Action::East { value } => {
                self.x += value as i64;
            }
            Action::West { value } => {
                self.x -= value as i64;
            }
            Action::Left { degree } => {
                let n = degree / 90;
                for _ in 0..n {
                    let (u, v) = (-self.v, self.u);
                    self.u = u;
                    self.v = v;
                }
            }
            Action::Right { degree } => {
                let n = degree / 90;
                for _ in 0..n {
                    let (u, v) = (self.v, -self.u);
                    self.u = u;
                    self.v = v;
                    // (self.u, self.v) = (self.v, -self.u);
                }
            }
            Action::Forward { value } => {
                self.x += self.u * value as i64;
                self.y += self.v * value as i64;
            }
        }
    }
}

fn main() {
    // let filename = "input/day12-example.txt";
    let filename = "input/day12.txt";
    let actions: Vec<Action> = read_lines(filename)
        .map(|line| line.unwrap().parse::<Action>())
        .filter_map(Result::ok)
        .collect();
    let mut ship = Ship::new();
    for action in actions {
        ship.tick(action);
        println!("{:?}", ship);
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}
