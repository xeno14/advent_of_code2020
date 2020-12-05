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
    }
}
