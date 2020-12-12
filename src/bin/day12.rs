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
    North { value: i32 },
    South { value: i32 },
    East { value: i32 },
    West { value: i32 },
    Left { degree: i32 },
    Right { degree: i32 },
    Forward { value: i32 },
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Action, Self::Err> {
        let re = Regex::new(r"([A-Z])(\d+)").unwrap();
        let caps = re.captures(s).unwrap();
        let direction = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
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
    // ship's position
    x: i64,
    y: i64,
    // ship's direction
    u: i64,
    v: i64,
    // relatrive positon to waypoint
    wx: i64,
    wy: i64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            x: 0,
            y: 0,
            u: 1,
            v: 0,
            wx: 10,
            wy: 1,
        }
    }

    pub fn tick(&mut self, action: Action) {
        match action {
            Action::North { value } => {
                self.y += value as i64;
            }
            Action::South { value } => {
                self.tick(Action::North { value: -value });
            }
            Action::East { value } => {
                self.x += value as i64;
            }
            Action::West { value } => {
                self.tick(Action::East { value: -value });
            }
            Action::Left { degree } => {
                let n = degree.abs() / 90;
                let rotate = |u: i64, v: i64| {
                    if degree.is_negative() {
                        (v, -u)
                    } else { 
                        (-v, u)
                    }
                };
                for _ in 0..n {
                    let (u, v) = rotate(self.u, self.v);
                    self.u = u;
                    self.v = v;
                }
            }
            Action::Right { degree } => {
                self.tick(Action::Left { degree: -degree });
            }
            Action::Forward { value } => {
                self.x += self.u * value as i64;
                self.y += self.v * value as i64;
            }
        }
    }

    /// delta of position: waypoint - ship
    pub fn delta(&self) -> (i64, i64) {
        (self.wx - self.x, self.wy - self.y)
    }

    pub fn tick2(&mut self, action: Action) {
        match action {
            Action::North { value } => {
                self.wy += value as i64;
            }
            Action::South { value } => {
                self.tick2(Action::North { value: -value });
            }
            Action::East { value } => {
                self.wx += value as i64;
            }
            Action::West { value } => {
                self.tick2(Action::East { value: -value });
            }
            Action::Left { degree } => {
                let n = degree.abs() / 90;
                let rotate = |u: i64, v: i64| {
                    if degree.is_negative() {
                        (v, -u)
                    } else { 
                        (-v, u)
                    }
                };
                for _ in 0..n {
                    let (u, v) = rotate(self.wx, self.wy);
                    self.wx = u;
                    self.wy = v;
                }
            }
            Action::Right { degree } => {
                self.tick2(Action::Left { degree: -degree });
            }
            Action::Forward { value } => {
                self.x += self.wx * value as i64;
                self.y += self.wy * value as i64;
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

    // part1
    let mut ship = Ship::new();
    for action in actions.iter() {
        ship.tick(action.clone());
        println!("{:?}", ship);
    }
    println!("{}", ship.x.abs() + ship.y.abs());

    // part2
    let mut ship = Ship::new();
    for action in actions.iter() {
        ship.tick2(action.clone());
        println!("{:?}", ship);
    }
    println!("{}", ship.x.abs() + ship.y.abs());
}
