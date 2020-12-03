use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

const DAY: &str = "03";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<Row> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(input: &[Row]) -> Result<u64> {
    let (mut cx, mut cy): (usize, usize) = (0, 0);
    let mut total_trees: u64 = 0;

    let width = input[0].trees.len();

    while cy < input.len() - 1 {
        cx += 3; // Go right
        cy += 1; // Go down

        if cx >= width {
            cx -= width;
        }

        total_trees += input[cy].trees[cx];
    }

    Ok(total_trees)
}

fn part2(input: &[Row]) -> Result<u64> {
    let (mut cx, mut cy): (usize, usize) = (0, 0);
    let mut result: u64 = 1;

    let offsets: Vec<(usize, usize)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];

    let width = input[0].trees.len();

    for (ox, oy) in offsets {
        let mut total_trees: u64 = 0;

        while cy < input.len() - 1 {
            cx += ox; // Go right
            cy += oy; // Go down

            if cx >= width {
                cx -= width;
            }

            total_trees += input[cy].trees[cx];
        }

        result *= total_trees;
        cx = 0;
        cy = 0;
    }

    Ok(result)
}

#[derive(Debug)]
struct Row {
    trees: Vec<u64>
}

impl std::str::FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<u64> = s
            .chars()
            .map(|c| {
                match c {
                    '.' => 0,
                    '#' => 1,
                    _ => { unreachable!() }
                }
            })
            .collect();

        Ok(Row {
            trees,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_part_1() {
        let input: Vec<&str> = vec!["..##.......", "#...#...#..", ".#....#..#.", "..#.#...#.#", ".#...##..#.", "..#.##.....", ".#.#.#....#", ".#........#", "#.##...#...", "#...##....#", ".#..#...#.#"];
        let mut entries: Vec<Row> = vec![];
        for line in input {
            entries.push(Row::from_str(line).unwrap());
        }

        assert_eq!(part1(&entries).unwrap(), 7);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<&str> = vec!["..##.......", "#...#...#..", ".#....#..#.", "..#.#...#.#", ".#...##..#.", "..#.##.....", ".#.#.#....#", ".#........#", "#.##...#...", "#...##....#", ".#..#...#.#"];
        let mut entries: Vec<Row> = vec![];
        for line in input {
            entries.push(Row::from_str(line).unwrap());
        }

        assert_eq!(part2(&entries).unwrap(), 336);
    }
}
