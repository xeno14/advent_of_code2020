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
        .map(|s| {
            let items: Vec<String> = s.split(" ").map(|t| t.to_owned()).collect();
            items
        })
        .map(count_all_common)
        .sum();
    println!("{}", ans);
}

fn count_all_common(items: Vec<String>) -> usize {
    if items.len() == 0 {
        return 0usize;
    }
    let mut set: HashSet<char> = items[0].chars().collect();
    for i in 1..items.len() {
        let t: HashSet<char> = items[i].chars().collect();
        set = set.intersection(&t).cloned().collect();
    }
    let count = set.len();
    count
}
