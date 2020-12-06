use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

const DAY: &str = "xx";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<String> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &[String]) -> Result<u32> {
    Ok(0)
}

fn part2(input: &[String]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<String> = vec![];
        assert_eq!(part1(&input).unwrap(), 0);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<String> = vec![];
        assert_eq!(part2(&input).unwrap(), 0);
    }
}
