use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Lines<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}

pub mod day4 {
    pub type Err = String;

    /// extract the value of a field
    pub fn extract_field(item: &str, field: &str) -> Result<String, Err> {
        let exp: String = format!(r"{}:([^\s]+)", field);

        let caps = regex::Regex::new(exp.as_ref())
            .map_err(|e| e.to_string())?
            .captures(item)
            .ok_or(format!("unable to capture {}", exp))?;

        let value = caps
            .get(1)
            .ok_or(format!("unable to capture {}", field))?
            .as_str()
            .to_string();
        Ok(value)
    }

    /// validate year
    fn validate_year(year: &str, min: u32, max: u32) -> Result<(), Err> {
        if year.len() != 4 {
            return Err("not 4 digits".to_owned());
        }
        let y: u32 = year.parse::<u32>().map_err(|e| e.to_string())?;
        if y < min || max < y {
            return Err("out of range".to_owned());
        }
        Ok(())
    }

    /// byr (Birth Year) - four digits; at least 1920 and at most 2002.
    pub fn validate_byr(byr: &str) -> Result<(), Err> {
        match validate_year(byr, 1920, 2002) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{} {}", byr, e)),
        }
    }

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    pub fn validate_iyr(iyr: &str) -> Result<(), Err> {
        match validate_year(iyr, 2010, 2020) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{} {}", iyr, e)),
        }
    }

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    pub fn validate_eyr(eyr: &str) -> Result<(), Err> {
        match validate_year(eyr, 2020, 2030) {
            Ok(ok) => Ok(ok),
            Err(e) => Err(format!("{} {}", eyr, e)),
        }
    }

    /// hgt (Height) - a number followed by either cm or in:
    /// If cm, the number must be at least 150 and at most 193.
    /// If in, the number must be at least 59 and at most 76.
    pub fn validate_hgt(hgt: &str) -> Result<(), String> {
        let expr = r"^(\d+)(cm|in)$";

        let caps = regex::Regex::new(expr.as_ref())
            .map_err(|e| e.to_string())?
            .captures(hgt)
            .ok_or(format!("unable to capture {}", expr))?;

        let height: u32 = caps
            .get(1)
            .ok_or(format!("digit not found"))?
            .as_str()
            .parse::<u32>()
            .map_err(|e| e.to_string())?;
        let unit: String = caps
            .get(2)
            .ok_or(format!("unit not found"))?
            .as_str()
            .to_string();

        // If cm, the number must be at least 150 and at most 193.
        if unit == "cm" && !(150 <= height && height <= 193) {
            return Err("invalid height cm".to_owned());
        }
        // If in, the number must be at least 59 and at most 76.
        else if unit == "in" && !(59 <= height && height <= 76) {
            return Err("invalid height in".to_owned());
        }
        Ok(())
    }

    /// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    pub fn validate_hcl(hcl: &str) -> Result<(), String> {
        let expr = r"^#([0-9a-z]{6})$";

        // let caps =
        regex::Regex::new(expr.as_ref())
            .map_err(|e| e.to_string())?
            .captures(hcl)
            .ok_or(format!("unable to capture {}", expr))?;
        Ok(())
    }

    /// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    pub fn validate_ecl(ecl: &str) -> Result<(), String> {
        let valid_ecl: std::collections::HashSet<String> =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
                .iter()
                .cloned()
                .map(|s| s.to_string())
                .collect();
        if !valid_ecl.contains(ecl) {
            return Err("invalid ecl".to_owned());
        }
        Ok(())
    }

    /// pid (Passport ID) - a nine-digit number, including leading zeroes.
    pub fn validate_pid(pid: &str) -> Result<(), String> {
        let expr = "^([0-9]{9})$";
        regex::Regex::new(expr)
            .map_err(|e| e.to_string())?
            .captures(&pid)
            .ok_or(format!("unable to capture {}", expr))?;
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_extract_field() {
            let item = "iyr:2010 hgt:158cm hcl:#b6652a";
            assert_eq!(Ok("2010".to_owned()), extract_field(item, "iyr"));
            assert_eq!(Ok("158cm".to_owned()), extract_field(item, "hgt"));

            assert!(extract_field(item, "byr").is_err())
        }

        #[test]
        fn test_validate_byr() {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            assert_eq!(Ok(()), validate_byr("2001"));
            assert_eq!(Ok(()), validate_byr("2002"));
            assert_eq!(Ok(()), validate_byr("1920"));

            assert!(validate_byr("02001").is_err());
            assert!(validate_byr("1919").is_err());
            assert!(validate_byr("2003").is_err());
        }

        #[test]
        fn test_validate_iyr() {
            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            assert_eq!(Ok(()), validate_iyr("2010"));
            assert_eq!(Ok(()), validate_iyr("2020"));

            assert!(validate_iyr("02015").is_err());
            assert!(validate_iyr("1919").is_err());
            assert!(validate_iyr("2021").is_err());
        }

        #[test]
        fn test_validate_eyr() {
            assert_eq!(Ok(()), validate_eyr("2020"));
            assert_eq!(Ok(()), validate_eyr("2030"));

            assert!(validate_eyr("02025").is_err());
            assert!(validate_eyr("1919").is_err());
        }

        #[test]
        fn test_validate_hgt() {
            assert_eq!(Ok(()), validate_hgt("150cm"));
            assert_eq!(Ok(()), validate_hgt("193cm"));
            assert_eq!(Ok(()), validate_hgt("59in"));
            assert_eq!(Ok(()), validate_hgt("76in"));

            assert!(validate_hgt("190in").is_err());
            assert!(validate_hgt("190").is_err());
        }

        #[test]
        fn test_validate_hcl() {
            // hcl valid:   #123abc
            // hcl invalid: #123abz
            // hcl invalid: 123abc
            assert_eq!(Ok(()), validate_hcl("#123abc"));
            assert_eq!(Ok(()), validate_hcl("#123abz"));
            assert!(validate_hcl("#123abzaaaaaa").is_err());
            assert!(validate_hcl("123abz").is_err());
            assert!(validate_hcl("a#23abz").is_err());
        }

        #[test]
        fn test_validate_ecl() {
            // ecl valid:   brn
            // ecl invalid: wat
            assert!(validate_ecl("brn").is_ok());
            assert!(validate_ecl("wat").is_err());
        }

        #[test]
        fn test_validate_pid() {
            // pid valid:   000000001
            // pid invalid: 0123456789
            assert_eq!(Ok(()), validate_pid("000000001"));
            assert!(validate_pid("0123456789").is_err());
            assert!(validate_pid("01234567").is_err());
        }
    }
}
