use aoc::read_lines;
use std::collections::HashSet;

fn walk(s: &str) -> (i64, i64) {
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

fn main() {
    // let filename = "input/day24-example.txt";
    let filename = "input/day24.txt";
    let walks: Vec<(i64, i64)> = read_lines(filename)
        .into_iter()
        .map(|line| walk(&line.unwrap()))
        .collect();
    
    let mut s: HashSet<(i64, i64)> = HashSet::new();
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
}
