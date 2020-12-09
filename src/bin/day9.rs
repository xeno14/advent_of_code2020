use aoc::read_lines;

use std::collections::HashSet;
use std::path::Path;
use std::collections::VecDeque;
use std::collections::vec_deque::Iter;

fn main() {
    // let filename = "input/day9-example.txt";
    // let preamble = 5;
    let filename = "input/day9.txt";
    let preamble = 5;

    let vec = read_integers(filename);
    let mut dq = VecDeque::new();
    for x in vec.into_iter() {
        if dq.len() < preamble {
            dq.push_back(x);
            continue;
        }
        if !two_sum(dq.iter(), x) {
            println!("{}", x);
            break;
        }
        dq.pop_front();
        dq.push_back(x);
    }
}

fn two_sum<'a>(iter: Iter<'a, i64>, target: i64) -> bool {
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

fn con_sum<'a>(iter: Iter<'a, i64>, target: i64) -> bool {
    iter.scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    })
    .map(|&x| x == target)
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