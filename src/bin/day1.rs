use aoc::read_lines;

use std::collections::HashSet;
use std::path::Path;

fn main() {
    let input = "input/day1-1.txt";
    let vec = read_integers(input);
    let ans = solve_part1(vec);
    println!("part1={}", ans);

    let input = "input/day1-2.txt";
    let vec = read_integers(input);
    let ans = solve_part2(vec);
    println!("part2={}", ans);
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

fn solve_part1(vec: Vec<i64>) -> i64 {
    // Part 1
    // HashSet
    // https://doc.rust-lang.org/std/collections/struct.HashSet.html
    let set: HashSet<i64> = vec.into_iter().collect();
    let target = 2020i64;
    for x in &set {
        let y = target - x;
        if set.contains(&y) {
            return x * y;
        }
    }
    panic!("not found")
}

fn solve_part2(vec: Vec<i64>) -> i64 {
    let set: HashSet<i64> = vec.into_iter().collect();
    let target = 2020i64;
    for x in &set {
        for y in &set {
            let z = target - x - y;
            if set.contains(&z) {
                return x * y * z;
            }
        }
    }
    panic!("not found")
}
