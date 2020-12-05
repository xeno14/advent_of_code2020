use std::fs;
use std::path::Path;

use aoc::day4::*;

fn main() {
    let items = read_items(Path::new("input/day4-valid.txt"));
    assert_eq!(4, validate_items(items.into_iter()));

    let items = read_items(Path::new("input/day4-invalid.txt"));
    assert_eq!(0, validate_items(items.into_iter()));

    let items = read_items(Path::new("input/day4.txt"));
    let ans = validate_items(items.into_iter());
    println!("{}", ans);
}

fn validate_items<I>(items: I) -> usize
where
    I: Iterator<Item = String>,
{
    items
        .into_iter()
        .map(|item| validate_item(&item.to_string()))
        .filter(|item| item.is_ok())
        .count()
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

    let hcl: String = extract_field(item, "hcl")?;
    validate_hcl(&hcl)?;

    let ecl: String = extract_field(item, "ecl")?;
    validate_ecl(&ecl)?;

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let pid: String = extract_field(item, "pid")?;
    validate_pid(&pid)?;

    // cid (Country ID) - ignored, missing or not.

    Ok(())
}

fn read_items(path: &Path) -> Vec<String> {
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
