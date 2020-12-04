use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

use anyhow::Result;
use regex::Regex;

const DAY: &str = "04";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(input_file);

    let mut input: String = String::new();

    buf_reader.read_to_string(&mut input)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &String) -> Result<u64>
{
    let mut total_valid: u64 = 0;
    let passports: Vec<&str> = input.split("\n\n").collect();

    for passport in passports {
        let passport = Passport::from_str(passport)?;

        if passport.validate_part_1()
        {
            total_valid += 1;
        }
    }

    Ok(total_valid)
}

fn part2(input: &String) -> Result<u64> {
    let mut total_valid: u64 = 0;
    let passports: Vec<&str> = input.split("\n\n").collect();

    for passport in passports {
        let passport = Passport::from_str(passport)?;

        if passport.validate_part_2().unwrap() {
            total_valid += 1;
        }
    }

    Ok(total_valid)
}

#[allow(dead_code)]
#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn validate_part_1(&self) -> bool {
        if self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
        {
            true
        } else { false }
    }

    pub fn validate_part_2(&self) -> Result<bool> {
        let regex_byr = Regex::new(r"(19[2-9][\d])|(200[012])")?;
        let regex_iyr = Regex::new(r"20((1\d)|(20))")?;
        let regex_eyr = Regex::new(r"20((2[\d])|(30))")?;
        let regex_hcl = Regex::new(r"^#[\da-f]{6}$")?;
        let regex_pid = Regex::new(r"^[\d]{9}$")?;

        match &self.byr {
            None => { return Ok(false); }
            Some(byr) => {
                if !regex_byr.is_match(byr) { return Ok(false); }
            }
        }

        match &self.iyr {
            None => { return Ok(false); }
            Some(iyr) => {
                if !regex_iyr.is_match(iyr) { return Ok(false); }
            }
        }

        match &self.eyr {
            None => { return Ok(false); }
            Some(eyr) => {
                if !regex_eyr.is_match(eyr) { return Ok(false); }
            }
        }

        match &self.hgt {
            None => {
                return Ok(false);
            }
            Some(hgt) => {
                if !hgt.ends_with("cm") && !hgt.ends_with("in") {
                    return Ok(false);
                }

                let (val, unit) = hgt.split_at(hgt.len() - 2);
                let val: u32 = val.parse()?;

                match unit {
                    "cm" => {
                        if val < 150 || val > 193 { return Ok(false); }
                    }
                    "in" => {
                        if val < 59 || val > 76 { return Ok(false); }
                    }
                    _ => { return Ok(false); }
                }
            }
        }

        match &self.hcl {
            None => { return Ok(false); }
            Some(hcl) => {
                if !regex_hcl.is_match(hcl) { return Ok(false); }
            }
        }

        match &self.ecl {
            None => {
                return Ok(false); }
            Some(ecl) => {
                let colors = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

                if !colors.contains(&ecl.as_str()) { return Ok(false); }
            }
        }

        match &self.pid {
            None => { return Ok(false); }
            Some(pid) => {
                if !regex_pid.is_match(pid) { return Ok(false); }
            }
        }

        Ok(true)
    }
}

impl std::str::FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byr = None;
        let mut iyr = None;
        let mut eyr = None;
        let mut hgt = None;
        let mut hcl = None;
        let mut ecl = None;
        let mut pid = None;
        let mut cid = None;

        let properties: Vec<&str> = s.split(&[' ', '\n'][..]).collect();

        for property in properties {
            let prop_val: Vec<&str> = property.split(':').collect();

            match prop_val[0] {
                "byr" => { byr = Some(String::from(prop_val[1])) }
                "iyr" => { iyr = Some(String::from(prop_val[1])) }
                "eyr" => { eyr = Some(String::from(prop_val[1])) }
                "hgt" => { hgt = Some(String::from(prop_val[1])) }
                "hcl" => { hcl = Some(String::from(prop_val[1])) }
                "ecl" => { ecl = Some(String::from(prop_val[1])) }
                "pid" => { pid = Some(String::from(prop_val[1])) }
                "cid" => { cid = Some(String::from(prop_val[1])) }
                _ => { unreachable!() }
            }
        }

        Ok(
            Self {
                byr,
                iyr,
                eyr,
                hgt,
                hcl,
                ecl,
                pid,
                cid,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(&String::from(input)).unwrap(), 2);
    }

    #[test]
    fn test_part_2() {
        let input: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(&String::from(input)).unwrap(), 2);
    }
}
