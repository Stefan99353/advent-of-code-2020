use std::fs::File;
use std::io::BufReader;

use anyhow::{Error, Result};

const DAY: &str = "01";

const SUM: u32 = 2020;

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<u32> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &Vec<u32>) -> Result<u32> {
    for l1 in 0..input.len() - 1 {
        for l2 in l1..input.len() - 1 {
            if input[l1] + input[l2] == SUM {
                return Ok(input[l1] * input[l2]);
            }
        }
    }

    return Err(Error::msg("No matching numbers found"));
}

fn part2(input: &Vec<u32>) -> Result<u32> {
    for l1 in 0..input.len() - 1 {
        for l2 in l1..input.len() - 1 {
            for l3 in l2..input.len() - 1 {
                if input[l1] + input[l2] + input[l3] == SUM {
                    return Ok(input[l1] * input[l2] * input[l3]);
                }
            }
        }
    }

    return Err(Error::msg("No matching numbers found"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input).unwrap(), 514579);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<u32> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input).unwrap(), 241861950);
    }

}