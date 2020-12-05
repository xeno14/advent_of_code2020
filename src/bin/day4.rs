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
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let byr: String = extract_field(item, "byr")?;
    validate_byr(&byr)?;

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let iyr: String = extract_field(item, "iyr")?;
    validate_iyr(&iyr)?;

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let eyr: String = extract_field(item, "eyr")?;
    validate_eyr(&eyr)?;

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    let hgt: String = extract_field(item, "hgt")?;
    match validate_height(&hgt) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let hcl: String = extract_field(item, "hcl")?;
    match validate_haircolor(&hcl) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

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

fn validate_height(hgt: &str) -> Result<(), String> {
    let expr = r"(\d+)([a-z]+)";

    let caps = Regex::new(expr.as_ref())
        .map_err(|e| e.to_string())?
        .captures(hgt)
        .ok_or(format!("unable to capture {}", expr))?;

    let height = caps
        .get(1)
        .ok_or(format!("digit not found"))?
        .as_str()
        .parse::<u32>()
        .map_err(|e| e.to_string())?;
    let unit = caps
        .get(2)
        .ok_or(format!("unit not found"))?
        .as_str()
        .to_string();

    // hgt (Height) - a number followed by either cm or in:
    if unit != "cm" && unit != "in" {
        return Err("wrong unit".to_owned());
    }
    // If cm, the number must be at least 150 and at most 193.
    if unit == "cm" && !(150 <= height && height <= 193) {
        return Err("invalid height for cm".to_owned());
    }
    // If in, the number must be at least 59 and at most 76.
    else if unit == "in" && !(59 <= height && height <= 76) {
        return Err("invalid height for in".to_owned());
    }
    Ok(())
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
fn validate_haircolor(hcl: &str) -> Result<(), String> {
    let expr = r"#([0-9a-z]{6})";

    let caps = Regex::new(expr.as_ref())
        .map_err(|e| e.to_string())?
        .captures(hcl)
        .ok_or(format!("unable to capture {}", expr))?;

    let color = caps
        .get(1)
        .ok_or(format!("unable to capture {}", expr))?
        .as_str();
    if color.len() != 6 {
        return Err("color is not 6digit".to_owned());
    }
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
