use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

const DAY: &str = "09";
const PREAMBLE: usize = 25;

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<u64> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input, PREAMBLE)?);
    println!("Part 2:  {}", part2(&input, PREAMBLE)?);

    Ok(())
}

fn part1(input: &[u64], preamble: usize) -> Result<u64> {
    for pos in preamble..input.len() {
        let mut matches_pattern = false;
        let val = input.get(pos).unwrap();

        for i in pos - preamble..pos {
            for j in pos - preamble..pos {
                let (i, j) = (input.get(i).unwrap(), input.get(j).unwrap());
                if i != j && i + j == *val {
                    matches_pattern = true;
                }
            }
        }

        if !matches_pattern {
            return Ok(*input.get(pos).unwrap());
        }
    }

    Err(anyhow::Error::msg("No result found"))
}

fn part2(input: &[u64], preamble: usize) -> Result<u64> {
    let result_part1 = part1(input, preamble).unwrap();

    for pos in 0..input.len() {
        let mut numbers: Vec<u64> = vec![];

        let mut offset: usize = 0;
        while pos + offset < input.len() && numbers.iter().sum::<u64>() < result_part1 {
            numbers.push(*input.get(pos + offset).unwrap());
            offset += 1;
        }

        if numbers.iter().sum::<u64>() == result_part1 {
            numbers.sort_unstable();

            return Ok(numbers.first().unwrap() + numbers.last().unwrap());
        }
    }

    Err(anyhow::Error::msg("No result found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<u64> = common::input_vec(buf_reader).unwrap();
        assert_eq!(part1(&input, 5).unwrap(), 127);
    }

    #[test]
    fn test_part_2() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<u64> = common::input_vec(buf_reader).unwrap();
        assert_eq!(part2(&input, 5).unwrap(), 62);
    }
}
