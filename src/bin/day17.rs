use aoc::read_lines;

use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Inactive,
    Active,
}

pub struct Universe {
    dim: Vec<u32>,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, x: &Vec<u32>) -> usize {
        let mut d = 1;
        let mut idx: usize = 0;
        // x0 + x1 * dim0 + x2 * dim0 * dim1 + ...
        for i in 0..self.dim.len() {
            idx += x[i] as usize * d as usize;
            d *= self.dim[i] as usize;
        }
        idx
    }

    fn index_to_pos(&self, idx: usize) -> Vec<u32> {
        // x0 = idx % dim0
        // x1 = idx / dim0 % dim1
        // ...
        let mut idx = idx;
        let mut x: Vec<u32> = vec![0; self.dim.len()];
        for i in 0..self.dim.len() {
            x[i] = (idx % self.dim[i] as usize) as u32;
            idx /= self.dim[i] as usize;
        }
        return x;
    }

    fn backtrack(out: &mut Vec<Vec<i32>>, v: &mut Vec<i32>, dim: usize) {
        if v.len() == dim {
            if v.iter().all(|&x| x == 0) {
                return;
            }
            out.push(v.clone());
            return;
        }

        for dx in [-1, 0, 1].iter().cloned() {
            v.push(dx);
            Self::backtrack(out, v, dim);
            v.pop();
        }
    }

    fn neiber_pos(x: &Vec<u32>) -> Vec<Vec<u32>> {
        let mut dx_list: Vec<Vec<i32>> = Vec::new();
        let mut dx: Vec<i32> = Vec::new();
        Self::backtrack(&mut dx_list, &mut dx, x.len());

        let res: Vec<Vec<u32>> = dx_list
            .into_iter()
            .map(|dx| {
                let mut x2 = vec![0; x.len()];
                for i in 0..x.len() {
                    x2[i] = x[i].wrapping_add(dx[i] as u32);
                }
                x2
            })
            .collect();
        res
    }

    fn active_neighbor_count(&self, x: &Vec<u32>) -> usize {
        let mut count = 0;
        for x2 in Self::neiber_pos(x) {
            // out of range
            if (0..self.dim.len()).any(|i| x2[i] >= self.dim[i]) {
                break;
            }
            let idx = self.get_index(&x2);
            count += if self.cells[idx] == Cell::Active {
                1
            } else {
                0
            };
        }
        count
    }

    fn tick(&mut self) {
        let mut next = self.cells.clone();

        for idx in 0..self.cells.len() {
            let x = self.index_to_pos(idx);
            // check range
            if (0..self.dim.len()).any(|i| x[i] == 0 || x[i] > self.dim[i] - 2) {
                continue;
            }
            let cell = self.cells[idx];
            let next_cell = match (cell, self.active_neighbor_count(&x)) {
                (Cell::Active, n) => {
                    if n == 2 || n == 3 {
                        Cell::Active
                    } else {
                        Cell::Inactive
                    }
                }
                (Cell::Inactive, 3) => Cell::Active,
                // remains the same
                (_, _) => Cell::Inactive,
            };
            next[idx] = next_cell;
        }
        self.cells = next;
    }

    fn parse<P>(filename: P, t: u32, d: usize) -> Universe
    where
        P: AsRef<Path>,
    {
        let l: u32 = (t + 1) * 2 + 8;
        let offset = t + 1;
        let dim = vec![l; d];
        let universe = Universe {
            dim,
            cells: Vec::new(),
        };
        let total_size: usize = universe.dim.iter().map(|&x| x as usize).product();
        let mut cells: Vec<Cell> = vec![Cell::Inactive; total_size];
        let mut x = vec![offset; universe.dim.len()];
        for line in read_lines(filename).into_iter().map(|line| line.unwrap()) {
            x[0] = offset;
            for c in line.chars() {
                if c == '#' {
                    let idx = universe.get_index(&x);
                    cells[idx] = Cell::Active;
                }
                x[0] += 1;
            }
            x[1] += 1;
        }
        Universe {
            cells,
            dim: universe.dim,
        }
    }
}

fn main() {
    // let filename = "input/day17-example.txt";
    let filename = "input/day17.txt";

    // part1
    let mut universe = Universe::parse(filename, 6, 3);
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

    // part2
    let mut universe = Universe::parse(filename, 6, 4);
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
