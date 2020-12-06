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