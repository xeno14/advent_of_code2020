use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let input = "input/day1-1.txt";
    let vec = read_integers(input);
    let ans = solve_part1(vec);
    println!("part1={}", ans);

    // Part 2
    // O(N^3) solution
    let input = "input/day1-2.txt";
    let vec = read_integers(input);
    let ans = solve_part2(vec);
    println!("part2={}", ans);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// read integers or die
fn read_integers<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let mut vec: Vec<i64> = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            vec.push(line.unwrap().parse().unwrap());
        }
    }
    vec
}

fn solve_part1(vec: Vec<i64>) -> i64 {
    // Part 1
    // O(N^2) solution
    for x in &vec {
        for y in &vec {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    panic!("not found")
}

fn solve_part2(vec: Vec<i64>) -> i64 {
    // Part 1
    // O(N^2) solution
    for x in &vec {
        for y in &vec {
            for z in &vec {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!("not found")
}
