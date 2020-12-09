use aoc::read_lines;

use std::collections::HashSet;
use std::collections::VecDeque;
use std::path::Path;

fn main() {
    // let filename = "input/day9-example.txt";
    // let preamble = 5;
    let filename = "input/day9.txt";
    let preamble = 25;

    let vec = read_integers(filename);
    let (idx, ans) = find_invalid(vec.iter(), preamble).unwrap();
    println!("{}", ans);

    // part2
    let part_sum: Vec<i64> = vec
        .iter()
        .take(idx)
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    // part_sum[j] - part_sum[i] = a[i+1] + ... a[j]
    for i in 0..part_sum.len() {
        for j in i + 2..part_sum.len() {
            let sum = part_sum[j] - part_sum[i];
            if sum == ans {
                // find the min and max from a[i+1], ..., a[j]
                let subslice = vec.get((i + 1)..(j + 1)).unwrap();
                let min = subslice.iter().min().unwrap();
                let max = subslice.iter().max().unwrap();
                println!("{} + {} = {}", min, max, min + max);
                return;
            }
        }
    }
    panic!("answer not found");
}

fn find_invalid<'a, I>(iter: I, preamble: usize) -> Result<(usize, i64), String>
where
    I: Iterator<Item = &'a i64>,
{
    let mut dq = VecDeque::new();
    for (i, &x) in iter.enumerate() {
        if dq.len() < preamble {
            dq.push_back(x);
            continue;
        }
        if !two_sum(dq.iter(), x) {
            return Ok((i, x));
        }
        dq.pop_front();
        dq.push_back(x);
    }
    Err("not found".to_owned())
}

fn two_sum<'a, I>(iter: I, target: i64) -> bool
where
    I: Iterator<Item = &'a i64>,
{
    let mut set: HashSet<i64> = HashSet::new();
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
