use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

const DAY: &str = "02";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<Entry> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &Vec<Entry>) -> Result<usize> {
    Ok(
        input
            .iter()
            .filter(|entry| {
                let cnt = entry
                    .password
                    .chars()
                    .filter(|c| *c == entry.character)
                    .count();
                entry.range.contains(&cnt)
            })
            .count()
    )
}

fn part2(input: &Vec<Entry>) -> Result<usize> {
    Ok(
        input
            .iter()
            .filter(|entry| {
                let pos1 = entry.range.start() - 1;
                let pos2 = entry.range.end() - 1;

                let pos1: Option<bool> = entry.password
                    .get(pos1..)
                    .map(|c| c.starts_with(entry.character));

                let pos2: Option<bool> = entry.password
                    .get(pos2..)
                    .map(|c| c.starts_with(entry.character));

                match (pos1, pos2) {
                    (Some(true), Some(false)) => true,
                    (Some(false), Some(true)) => true,
                    (_, _) => false,
                }
            })
            .count()
    )
}

#[derive(Debug)]
struct Entry {
    range: std::ops::RangeInclusive<usize>,
    character: char,
    password: String,
}

impl std::str::FromStr for Entry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let min: usize;
        let max: usize;
        let char: char;
        let password: String;

        // Password
        let mut temp: Vec<&str> = s.split(": ").collect();
        password = String::from(temp[1]);

        // Char
        temp = temp[0].split(" ").collect();
        char = temp[1].chars().nth(0).unwrap();

        // Range
        temp = temp[0].split("-").collect();
        min = temp[0].parse()?;
        max = temp[1].parse()?;

        Ok(Self {
            range: (min..=max),
            character: char,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<&str> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let mut entries: Vec<Entry> = vec![];
        for line in input {
            entries.push(Entry::from_str(line).unwrap());
        }

        assert_eq!(part1(&entries).unwrap(), 2);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<&str> = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        let mut entries: Vec<Entry> = vec![];
        for line in input {
            entries.push(Entry::from_str(line).unwrap());
        }

        assert_eq!(part2(&entries).unwrap(), 1);
    }
}
