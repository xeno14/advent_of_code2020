use aoc::read_lines;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Password {
    x: usize,
    y: usize,
    c: char,
    passwd: String,
}

impl FromStr for Password {
    type Err = String;

    // e.g. 1-3 a: abcde
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = match Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)") {
            Ok(re) => re,
            Err(e) => return Err(e.to_string()),
        };
        let caps = match re.captures(input) {
            Some(caps) => caps,
            None => return Err("regex error".to_owned()),
        };
        let x = match caps.get(1).map(|c| c.as_str().parse::<usize>()) {
            Some(x) => match x {
                Ok(x) => x,
                Err(e) => return Err(e.to_string()),
            }
            None => return Err("regex error".to_owned()),
        };

        let y = match caps.get(2).map(|c| c.as_str().parse::<usize>()) {
            Some(y) => match y {
                Ok(y) => y,
                Err(e) => return Err(e.to_string()),
            }
            None => return Err("regex error".to_owned()),
        };

        let c = match caps
            .get(3)
            .map(|c| c.as_str())
            .and_then(|c| c.chars().nth(0))
        {
            Some(c) => c,
            None => return Err("regex error".to_owned()),
        };

        let passwd = match caps.get(4).map(|c| c.as_str()) {
            Some(passwd) => passwd,
            None => return Err("regex error".to_owned()),
        };
        let passwd = String::from(passwd);

        Ok(Password { x, y, c, passwd })
    }
}

fn main() {
    // part1
    let ans = solve_part1("input/day2.txt").unwrap();
    println!("{:?}", ans);

    // part2
    let ans = solve_part2("input/day2.txt").unwrap();
    println!("{:?}", ans);
}

fn solve_part1(input: &str) -> Result<usize, String> {
    // TODO remove unwrap
    let passwords: Result<Vec<_>, _> = read_lines(input)
        .map(|line| Password::from_str(line.as_ref().unwrap()))
        .collect();
    let count = passwords?
        .into_iter()
        .filter(|p| {
            let count = p.passwd.chars().filter(|c| c == &p.c).count();
            p.x <= count && count <= p.y
        })
        .count();
    Ok(count)
}

fn solve_part2(input: &str) -> Result<usize, String> {
    // TODO remove unwrap
    let passwords: Result<Vec<_>, _> = read_lines(input)
        .map(|line| Password::from_str(line.as_ref().unwrap()))
        .collect();
    let count = passwords?
        .into_iter()
        .filter(|p| {
            let cond1 = match p.passwd.chars().nth(p.x - 1) {
                Some(c) => c == p.c,
                None => false,
            };
            let cond2 = match p.passwd.chars().nth(p.y - 1) {
                Some(c) => c == p.c,
                None => false,
            };
            return cond1 ^ cond2;
        })
        .count();
    Ok(count)
}
