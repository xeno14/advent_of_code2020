use aoc::read_integers;
use std::path::Path;
use counter::Counter;

fn read_sorted_integers<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let mut vec = read_integers(filename);
    vec.sort();
    vec
}

fn main() {
    // let filename = "input/day10-example.txt";
    let filename = "input/day10.txt";
    let vec = read_sorted_integers(filename);
    // https://doc.rust-lang.org/std/primitive.slice.html#method.windows
    let diff: Vec<i64> = vec.windows(2).map(|slice| {
        slice[1]-slice[0]
    }).collect();
    let counter = diff.into_iter().collect::<Counter<_>>();
    let n3 = counter[&3] + if vec[0] == 3 {1} else {0} + 1;
    let n1 = counter[&1] + if vec[0] == 1 {1} else {0};
    let ans = n1 * n3;
    println!("{}", ans);
}
