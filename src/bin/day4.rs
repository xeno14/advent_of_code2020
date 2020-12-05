use regex::Regex;
use std::fs;
use std::path::Path;

use aoc::day4::*;

fn main() {
    let items = read_file(Path::new("input/day4-valid.txt"));
    test(4, items);
    let items = read_file(Path::new("input/day4-invalid.txt"));
    test(0, items);
    let items = read_file(Path::new("input/day4-invalid.txt"));
    let items = read_file(Path::new("input/day4.txt"));
    let ans = items
        .into_iter()
        .map(|item| validate_item(&item.to_string()))
        .filter(|item| item.is_ok())
        .count();
    println!("{}", ans);
}

fn test(expected: usize, items: Vec<String>) {
    type Res = Result<(), String>;
    let (oks, errs): (Vec<Res>, Vec<Res>) = items
        .into_iter()
        .map(|item| validate_item(&item.to_string()))
        .partition(Result::is_ok);
    let ans = oks.into_iter().filter(Result::is_ok).count();
    for err in errs.into_iter() {
        println!("{}", err.unwrap_err());
    }
    if expected != ans {
        panic!("test fialed");
    }
}

fn validate_item(item: &str) -> Result<(), String> {
    let byr: String = extract_field(item, "byr")?;
    validate_byr(&byr)?;

    let iyr: String = extract_field(item, "iyr")?;
    validate_iyr(&iyr)?;

    let eyr: String = extract_field(item, "eyr")?;
    validate_eyr(&eyr)?;

    let hgt: String = extract_field(item, "hgt")?;
    validate_hgt(&hgt)?;

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl: String = extract_field(item, "hcl")?;
    validate_hcl(&hcl)?;

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let valid_ecl: std::collections::HashSet<String> =
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .iter()
            .cloned()
            .map(|s| s.to_string())
            .collect();
    let ecl: String = extract_field(item, "ecl")?;
    if !valid_ecl.contains(&ecl) {
        return Err("invalid ecl".to_owned());
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid: String = extract_field(item, "pid")?;
    match Regex::new("([0-9]{8})")
        .map_err(|e| e.to_string())?
        .captures(&pid)
    {
        Some(_) => (),
        None => return Err("invalid pid".to_owned()),
    };

    // cid (Country ID) - ignored, missing or not.

    Ok(())
}


fn read_file(path: &Path) -> Vec<String> {
    // std::fs::read_to_string
    // https://doc.rust-lang.org/std/fs/fn.read_to_string.html
    let input = match fs::read_to_string(path) {
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
            item.push(' ');
            item.push_str(line);
        }
        items.push(item);
    }
    items
}
