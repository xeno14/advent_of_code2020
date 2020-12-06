use aoc::read_items;

use std::collections::HashSet;

fn main() {
    // let filename = "input/day6-example.txt";
    let filename = "input/day6.txt";

    // part1
    let items = read_items(filename, "");
    let ans: usize = items
        .into_iter()
        .map(|s| {
            let set: HashSet<char> = s
                .chars()
                .into_iter()
                .filter(|c| match c {
                    // matching ranges
                    // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#matching-ranges-of-values-with-
                    'a'..='z' => true,
                    _ => false,
                })
                .collect();
            set.len()
        })
        .sum();
    println!("{}", ans);

    // part2
    let items = read_items(filename, " ");
    let ans: usize = items
        .into_iter()
        .map(|s| count_all_common(s.split(" ").map(|s| s.to_string())))
        .sum();
    println!("{}", ans);
}

fn count_all_common<I>(mut items: I) -> usize
where
    I: Iterator<Item = String>,
{
    let s = match items.next() {
        Some(s) => s,
        None => return 0,
    };
    let mut set: HashSet<char> = s.chars().collect();
    while let Some(s) = items.next() {
        let other: HashSet<char> = s.chars().collect();
        set = set.intersection(&other).cloned().collect();
    }
    let count = set.len();
    count
}
