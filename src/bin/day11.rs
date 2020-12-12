use aoc::read_lines;

use std::path::Path;

// model implementation after https://rustwasm.github.io/book/game-of-life/implementing.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

pub struct Ferry {
    width: u32,
    height: u32,
    seats: Vec<Seat>,
}

impl Ferry {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn dump(&self) -> () {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let seat = self.seats[idx];
                let c = match seat {
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                    Seat::Floor => '.',
                };
                print!("{}", c);
            }
            println!("");
        }
    }

    fn occupied_neighbor_count(&self, row: u32, col: u32) -> usize {
        let mut count = 0;
        for dr in [-1i32, 0, 1].iter().cloned() {
            for dc in [-1i32, 0, 1].iter().cloned() {
                // exclude myself
                if dr == 0 && dc == 0 {
                    continue;
                }
                let nrow: u32 = row.wrapping_add(dr as u32);
                let ncol: u32 = col.wrapping_add(dc as u32);
                if nrow >= self.height || ncol >= self.width {
                    break;
                }
                let idx = self.get_index(nrow, ncol);
                count += if self.seats[idx] == Seat::Occupied {
                    1
                } else {
                    0
                };
            }
        }
        count
    }

    fn tick(&mut self) -> bool {
        let mut next = self.seats.clone();
        let mut updated = false;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let seat = self.seats[idx];
                let occupied_neighbors = self.occupied_neighbor_count(row, col);

                let next_seat = match (seat, occupied_neighbors) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, x) => {
                        if x >= 4 {
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    (otherwise, _) => otherwise,
                };
                if !updated && seat != next_seat {
                    updated = true;
                }
                next[idx] = next_seat;
            }
        }
        self.seats = next;
        updated
    }

    fn occupied_visible_count(&self, row: u32, col: u32) -> usize {
        let mut count = 0;
        for dr in [-1i32, 0, 1].iter().cloned() {
            for dc in [-1i32, 0, 1].iter().cloned() {
                // exclude myself
                if dr == 0 && dc == 0 {
                    continue;
                }
                // loop until reaching a seat or out of range
                let mut i = 1;
                loop {
                    let nrow: u32 = row.wrapping_add((dr * i) as u32);
                    let ncol: u32 = col.wrapping_add((dc * i) as u32);
                    if nrow >= self.height || ncol >= self.width {
                        break;
                    }
                    let idx = self.get_index(nrow, ncol);
                    if self.seats[idx] == Seat::Occupied {
                        count += 1;
                        break;
                    } else if self.seats[idx] == Seat::Empty {
                        break;
                    }
                    i += 1;
                }
            }
        }
        count
    }

    fn tick2(&mut self) -> bool {
        let mut next = self.seats.clone();
        let mut updated = false;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let seat = self.seats[idx];
                let count = self.occupied_visible_count(row, col);

                let next_seat = match (seat, count) {
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, x) => {
                        if x >= 5 {
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                    (otherwise, _) => otherwise,
                };
                if !updated && seat != next_seat {
                    updated = true;
                }
                next[idx] = next_seat;
            }
        }
        self.seats = next;
        updated
    }

    fn parse<P>(filename: P) -> Ferry
    where
        P: AsRef<Path>,
    {
        let mut seats: Vec<Seat> = Vec::new();
        let mut height: u32 = 0;
        for line in read_lines(filename).into_iter().map(|line| line.unwrap()) {
            let mut vec: Vec<Seat> = line
                .chars()
                .map(|c| match c {
                    '.' => Seat::Floor,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => panic!("unable to parse"),
                })
                .collect();
            seats.append(&mut vec);
            height += 1;
        }
        let width = seats.len() as u32 / height;
        Ferry {
            seats,
            height,
            width,
        }
    }
}

fn main() {
    // let filename = "input/day11-example.txt";
    let filename = "input/day11.txt";
    let mut ferry = Ferry::parse(filename);
    while ferry.tick() {}
    // ferry.dump();
    // count occupied
    let ans: u32 = ferry
        .seats
        .iter()
        .map(|&s| match s {
            Seat::Occupied => 1,
            _ => 0,
        })
        .sum();
    println!("{}", ans);

    let mut ferry = Ferry::parse(filename);
    // ferry.dump();
    // println!("");
    while ferry.tick2() {
        // ferry.dump();
        // println!("");
    }
    // count occupied
    let ans: u32 = ferry
        .seats
        .iter()
        .map(|&s| match s {
            Seat::Occupied => 1,
            _ => 0,
        })
        .sum();
    println!("{}", ans);
}
