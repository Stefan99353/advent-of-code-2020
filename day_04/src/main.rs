use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use anyhow::Result;
use regex::Regex;

const DAY: &str = "04";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let mut input_file = File::open("input.txt")?;
    let mut input: String = String::new();
    input_file.read_to_string(&mut input)?;

    let passports = input.split("\n\n").collect::<Vec<&str>>()
        .iter()
        .map(|s| { Passport::from_str(s).unwrap() })
        .collect::<Vec<Passport>>();

    println!("Part 1:  {}", part1(&passports)?);
    println!("Part 2:  {}", part2(&passports)?);

    Ok(())
}

fn part1(passports: &[Passport]) -> Result<usize>
{
    Ok(passports
        .iter()
        .filter(|p| { p.validate_part_1() })
        .count())
}

fn part2(passports: &[Passport]) -> Result<usize> {
    Ok(passports
        .iter()
        .filter(|p| { p.validate_part_2().unwrap_or(false) })
        .count())
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
    pub fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    pub fn validate_part_1(&self) -> bool {
        self.byr.is_some() && self.iyr.is_some() && self.eyr.is_some() && self.hgt.is_some() && self.hcl.is_some() && self.ecl.is_some() && self.pid.is_some()
    }

    pub fn validate_part_2(&self) -> Result<bool> {
        let regex_hgt = Regex::new(r"^((1[5-8]\dcm)|(19[0-3]cm)|(59in)|(6\din)|(7[0-6]in))$")?;
        let regex_hcl = Regex::new(r"^#[\da-f]{6}$")?;
        let regex_ecl = Regex::new(r"^((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth))$")?;
        let regex_pid = Regex::new(r"^\d{9}$")?;

        Ok(self.validate_part_1()
            && (1920..2003).contains(&self.byr.as_ref().unwrap().parse::<i32>()?)
            && (2010..2021).contains(&self.iyr.as_ref().unwrap().parse::<i32>()?)
            && (2020..2031).contains(&self.eyr.as_ref().unwrap().parse::<i32>()?)
            && regex_hgt.is_match(&self.hgt.as_ref().unwrap())
            && regex_hcl.is_match(&self.hcl.as_ref().unwrap())
            && regex_ecl.is_match(&self.ecl.as_ref().unwrap())
            && regex_pid.is_match(&self.pid.as_ref().unwrap())
        )
    }
}

impl std::str::FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result= Self::new();

        let properties: Vec<&str> = s.split(&[' ', '\n'][..]).collect();

        for property in properties {
            let prop_val: Vec<&str> = property.split(':').collect();

            match prop_val[0] {
                "byr" => { result.byr = Some(String::from(prop_val[1])) }
                "iyr" => { result.iyr = Some(String::from(prop_val[1])) }
                "eyr" => { result.eyr = Some(String::from(prop_val[1])) }
                "hgt" => { result.hgt = Some(String::from(prop_val[1])) }
                "hcl" => { result.hcl = Some(String::from(prop_val[1])) }
                "ecl" => { result.ecl = Some(String::from(prop_val[1])) }
                "pid" => { result.pid = Some(String::from(prop_val[1])) }
                "cid" => { result.cid = Some(String::from(prop_val[1])) }
                _ => { unreachable!() }
            }
        }

        Ok(
            result
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

        let passports = input.split("\n\n").collect::<Vec<&str>>()
            .iter()
            .map(|s| { Passport::from_str(s).unwrap() })
            .collect::<Vec<Passport>>();

        assert_eq!(part1(&passports).unwrap(), 2);
    }

    #[test]
    fn test_part_2() {
        let input: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let passports = input.split("\n\n").collect::<Vec<&str>>()
            .iter()
            .map(|s| { Passport::from_str(s).unwrap() })
            .collect::<Vec<Passport>>();

        assert_eq!(part2(&passports).unwrap(), 4);
    }
}
