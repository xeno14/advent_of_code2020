use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // part1
    let ans = read_lines("input/day2.txt")
        .filter(|line| {
            is_valid(line.as_ref().unwrap())
        })
        .count();
    println!("{:?}", ans);

    // part2
    let ans = read_lines("input/day2.txt")
        .filter(|line| {
            is_valid2(line.as_ref().unwrap())
        })
        .count();
    println!("{:?}", ans);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

fn is_valid(input: &str) -> bool {
    // extract password
    let re: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(input).unwrap();

    let lbound: i32 = caps.get(1).map(|c| c.as_str().parse().unwrap()).unwrap();
    let ubound: i32 = caps.get(2).map(|c| c.as_str().parse().unwrap()).unwrap();
    let letter = caps.get(3).map(|c| c.as_str()).unwrap();
    let password = caps.get(4).map(|c| c.as_str()).unwrap();

    // let count_letter = password.to_string().
    let count_letter = password.chars().fold(0, |mut count, c| {
        if c.to_string() == letter {
            count += 1;
        }
        count
    });

    if lbound <= count_letter && count_letter <= ubound {
        true
    } else {
        false
    }
}

fn is_valid2(input: &str) -> bool {
    // extract password
    let re: Regex = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let caps = re.captures(input).unwrap();

    let a: usize = caps.get(1).map(|c| c.as_str().parse().unwrap()).unwrap();
    let b: usize = caps.get(2).map(|c| c.as_str().parse().unwrap()).unwrap();
    let letter = caps
        .get(3)
        .map(|c| c.as_str())
        .unwrap()
        .chars()
        .nth(0)
        .unwrap();
    let password = caps.get(4).map(|c| c.as_str()).unwrap();

    let cond1 = password.chars().nth(a - 1).unwrap() == letter;
    let cond2 = password.chars().nth(b - 1).unwrap() == letter;
    return cond1 ^ cond2;
}
