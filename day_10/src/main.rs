use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use std::collections::HashMap;

const DAY: &str = "xx";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<u64> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &[u64]) -> Result<u64> {
    let mut input = input.to_vec();
    input.push(0);

    input.sort_unstable();

    let last = input.last().unwrap() + 3;
    input.push(last);

    let (mut step1_count, mut step3_count) = (0, 0);

    for i in 0..input.len() - 1 {
        if input[i] + 1 == input[i + 1] { step1_count += 1; }
        if input[i] + 3 == input[i + 1] { step3_count += 1; }
    }

    Ok(step1_count * step3_count)
}

fn part2(input: &[u64]) -> Result<u64> {
    let mut input = input.to_vec();

    input.sort_unstable();
    let last = input.last().unwrap() + 3;

    let mut memory = HashMap::new();

    let result = count_connections(&input, 0, last, &mut memory);

    Ok(result)
}

fn count_connections(input: &[u64],
                     start: u64,
                     goal: u64,
                     memory: &mut HashMap<(usize, u64), u64>,
) -> u64 {
    let mut possible_ways = 0;

    let key = (input.len(), start);

    if let Some(value) = memory.get(&key) {
        return *value;
    }

    if goal - start <= 3 {
        possible_ways += 1;
    }

    if input.is_empty() {
        return possible_ways;
    }

    if input[0] - start <= 3 {
        possible_ways += count_connections(&input[1..], input[0], goal, memory);
    }

    possible_ways += count_connections(&input[1..], start, goal, memory);

    memory.insert(key, possible_ways);

    possible_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<u64> = common::input_vec(buf_reader).unwrap();

        assert_eq!(part1(&input).unwrap(), 35);
    }

    #[test]
    fn test_part_2() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<u64> = common::input_vec(buf_reader).unwrap();

        assert_eq!(part2(&input).unwrap(), 8);
    }
}
