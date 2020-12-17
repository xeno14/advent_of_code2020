use aoc::read_lines;

use std::path::Path;

// model implementation after https://rustwasm.github.io/book/game-of-life/implementing.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Inactive,
    Active,
}

pub struct Universe {
    nx: u32,
    ny: u32,
    nz: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: u32, y: u32, z:u32) -> usize {
        (z * self.nx * self.ny + self.nx * y + x) as usize
    }

    // pub fn dump(&self) -> () {
    //     for row in 0..self.height {
    //         for col in 0..self.width {
    //             let idx = self.get_index(row, col);
    //             let seat = self.seats[idx];
    //             let c = match seat {
    //                 Seat::Empty => 'L',
    //                 Seat::Occupied => '#',
    //                 Seat::Floor => '.',
    //             };
    //             print!("{}", c);
    //         }
    //         println!("");
    //     }
    // }

    fn active_neighbor_count(&self, x: u32, y: u32, z: u32) -> usize {
        let mut count = 0;
        for dz in [-1i32, 0, 1].iter().cloned() {
            for dy in [-1i32, 0, 1].iter().cloned() {
                for dx in [-1i32, 0, 1].iter().cloned() {
                    // exclude myself
                    if dx == 0 && dy == 0 && dz == 0 {
                        continue;
                    }
                    let x2: u32 = x.wrapping_add(dx as u32);
                    let y2: u32 = y.wrapping_add(dy as u32);
                    let z2: u32 = z.wrapping_add(dz as u32);
                    // out of range
                    if x2 >= self.nx|| y2 >= self.ny || z2 >= self.nz {
                        break;
                    }
                    let idx = self.get_index(x2, y2, z2);
                    count += if self.cells[idx] == Cell::Active {
                        1
                    } else {
                        0
                    };
                }
            }
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for z in 1..(self.nz-1) {
            for y in 1..(self.ny-1) {
                for x in 1..(self.nx-1) {
                    let idx = self.get_index(x, y, z);
                    let cell = self.cells[idx];
                    let n = self.active_neighbor_count(x,y,z);

                    let next_cell = match (cell, n) {
                        (Cell::Active, x) => {
                            if x == 2 || x==3 {
                                Cell::Active
                            } else {
                                Cell::Inactive
                            }
                        }
                        (Cell::Inactive, 3) => {
                            Cell::Active
                        }
                        // remains the same
                        (_, _) => Cell::Inactive,
                    };
                    next[idx] = next_cell;
                }
            }
        }
        self.cells = next;
    }

    fn parse<P>(filename: P, t: u32) -> Universe
    where
        P: AsRef<Path>,
    {
        let l: u32 = (t+1) * 2 + 8;
        let offset = t+1;
        let mut universe = Universe{nx: l, ny:l, nz:l, cells:Vec::new()};
        let mut cells :Vec<Cell> = vec![Cell::Inactive; l as usize*l as usize*l as usize];
        let mut y=offset;
        let z = offset;
        for line in read_lines(filename).into_iter().map(|line| line.unwrap()) {
            let mut x = offset;
            for c in line.chars() {
                if c == '#' {
                    let idx = universe.get_index(x, y, z);
                    cells[idx] = Cell::Active;
                }
                x += 1;
            }
            y += 1;
        }
        Universe {
            cells,
            nx: universe.nx,
            ny: universe.ny,
            nz: universe.nz
        }
    }
}

fn main() {
    // let filename = "input/day17-example.txt";
    let filename = "input/day17.txt";
    let mut universe = Universe::parse(filename, 6);
    for _ in 0..6 {
        universe.tick();
    }
    let ans: u32 = universe
        .cells
        .iter()
        .map(|&cell| match cell {
            Cell::Active => 1,
            _ => 0,
        })
        .sum();
    println!("{}", ans);
}
