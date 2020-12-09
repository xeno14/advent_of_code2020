use aoc::read_lines;

use std::collections::HashSet;
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    // let filename = "input/day9-example.txt";
    // let preamble = 5;
    let filename = "input/day9.txt";
    let preamble = 25;

    let vec = read_integers(filename);
    let ans = find_invalid(vec.iter(), preamble).unwrap();
    println!("{}", ans);
}

fn find_invalid<'a, I>(iter: I, preamble: usize) -> Result<i64, String>
where I: Iterator<Item = &'a i64>
{
    let mut dq = VecDeque::new();
    for &x in iter {
        if dq.len() < preamble {
            dq.push_back(x);
            continue;
        }
        if !two_sum(dq.iter(), x) {
            return Ok(x);
        }
        dq.pop_front();
        dq.push_back(x);
    }
    Err("not found".to_owned())
}

fn two_sum<'a, I>(iter: I, target: i64) -> bool
where I: Iterator<Item = &'a i64>{
    let mut set:HashSet<i64> = HashSet::new();
    for x in iter {
        let y = target - *x;
        if set.contains(&y) {
            return true;
        }
        set.insert(*x);
    }
    return false;
}

// read integers or die
fn read_integers<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let vec: Vec<i64> = read_lines(filename)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    vec
}