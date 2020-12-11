use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub mod day4;

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

/// read line separated items combining into a single line separated by a space.
pub fn read_items<P>(filename: P, sep: &str) -> Vec<String>
where
    P: AsRef<Path>,
{
    // std::fs::read_to_string
    // https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    let input = match std::fs::read_to_string(filename) {
        Ok(input) => input,
        Err(_) => panic!("input file missing"),
    };
    let input = input.trim();

    let mut items: Vec<String> = Vec::new();
    let mut iter = input.split("\n").into_iter();
    while let Some(line) = iter.next() {
        let mut item: String = line.trim().to_string();
        if item.len() == 0 {
            continue;
        }
        while let Some(line) = iter.next() {
            let line = line.trim();
            if line.len() == 0 {
                break;
            }
            item.push_str(sep);
            item.push_str(line);
        }
        items.push(item);
    }
    items
}

/// read integers or die
pub fn read_integers<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let vec: Vec<i64> = read_lines(filename)
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    vec
}