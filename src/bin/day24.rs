use aoc::read_lines;
use std::collections::HashSet;
// use ndarray::prelude::*;

fn walk(s: &str) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    let mut t = s;
    while t.len() > 0 {
        let (dx, dy) = if t.starts_with("ne") {
            t = &t[2..];
            (1, 1)
        } else if t.starts_with("nw") {
            t = &t[2..];
            (0, 1)
        } else if t.starts_with("sw") {
            t = &t[2..];
            (-1, -1)
        } else if t.starts_with("se") {
            t = &t[2..];
            (0, -1)
        } else if t.starts_with("e") {
            t = &t[1..];
            (1, 0)
        } else if t.starts_with("w") {
            t = &t[1..];
            (-1, 0)
        } else {
            panic!();
        };
        x += dx;
        y += dy;
    }
    (x, y)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Inactive, // white
    Active,   // black
}

struct Universe {
    dim: Vec<u32>,
    cells: Vec<Cell>,
}

impl Universe {
    fn new(dim: Vec<u32>) -> Universe {
        let size = (dim.iter().map(|x| *x).product::<u32>()) as usize;
        let cells = vec![Cell::Inactive; size];
        Universe { dim, cells }
    }

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

    fn set(&mut self, x: &Vec<u32>, cell: Cell) {
        let idx = self.get_index(x);
        self.cells[idx] = cell;
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

    fn neiber_pos(x: &Vec<u32>) -> Vec<Vec<u32>> {
        if x.len() != 2 {
            panic!("implemented")
        }
        let dx_list: Vec<Vec<i32>> = vec![
            vec![1, 1],
            vec![0, 1],
            vec![-1, -1],
            vec![0, -1],
            vec![1, 0],
            vec![-1, 0],
        ];
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
                    if n == 0 || n > 2 {
                        Cell::Inactive
                    } else {
                        Cell::Active
                    }
                }
                (Cell::Inactive, 2) => Cell::Active,
                // remains the same
                (_, _) => Cell::Inactive,
            };
            next[idx] = next_cell;
        }
        self.cells = next;
    }

    fn from_walks(blacks: &HashSet<(i32, i32)>) -> Universe {
        let dim: (u32, u32) = (300, 300);
        let offset: (u32, u32) = (150, 150);
        let mut univ: Universe = Universe::new(vec![dim.0, dim.1]);
        for (x, y) in blacks.iter() {
            let pos = vec![
                offset.0.wrapping_add(*x as u32),
                offset.1.wrapping_add(*y as u32),
            ];
            univ.set(&pos, Cell::Active);
        }
        univ
    }

    fn count_active(&self) -> usize {
        self.cells
            .iter()
            .filter(|&cell| *cell == Cell::Active)
            .count()
    }
}

fn main() {
    // let filename = "input/day24-example.txt";
    let filename = "input/day24.txt";
    let walks: Vec<(i32, i32)> = read_lines(filename)
        .into_iter()
        .map(|line| walk(&line.unwrap()))
        .collect();
    let mut s: HashSet<(i32, i32)> = HashSet::new();
    for dst in walks {
        println!("{:?}", dst);
        if s.contains(&dst) {
            s.remove(&dst);
        } else {
            s.insert(dst.clone());
        }
    }
    let ans = s.len();
    println!("{:?}", ans);

    let mut universe = Universe::from_walks(&s);
    let count = universe.count_active();
    println!("Day {}: {}", 0, count);
    for day in 1..101 {
        universe.tick();
        let count = universe.count_active();
        println!("Day {}: {}", day, count);
    }
}
