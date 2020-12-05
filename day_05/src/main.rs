use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use std::ops::Range;

const DAY: &str = "05";
const ROWS: u32 = 128;
const COLUMNS: u32 = 8;

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
    let mut ids: Vec<u32> = input.iter()
        .map(|s| { calc_seat_id(&s).unwrap_or(0) })
        .collect();
    ids.sort();

    Ok(*ids.last().unwrap())
}

fn part2(input: &[String]) -> Result<u32> {
    let valid_ids: Range<u32> = COLUMNS..(ROWS-1)*8+COLUMNS-1;

    let mut ids :Vec<u32> = input.iter()
        .map(|s| { calc_seat_id(&s).unwrap_or(0) })
        .collect();
    ids.sort();

    let mut id = 0;

    for valid_id in valid_ids {
        if !ids.contains(&valid_id) &&
            ids.contains(&(valid_id-1)) &&
            ids.contains( &(valid_id+1)) {
            id = valid_id;
        }
    }

    Ok(id)
}

fn calc_seat_id(input: &str) -> Result<u32> {
    let (rows, cols) = input.split_at(7);

    let rows = rows.replace("F", "0").replace("B", "1");
    let row = u32::from_str_radix(&rows, 2)?;
    let cols = cols.replace("L", "0").replace("R", "1");
    let col = u32::from_str_radix(&cols, 2)?;

    Ok((row * 8 + col) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: &str = "FBFBBFFRLR";
        assert_eq!(calc_seat_id(input).unwrap(), 357);
        let input: &str = "BFFFBBFRRR";
        assert_eq!(calc_seat_id(input).unwrap(), 567);
        let input: &str = "FFFBBBFRRR";
        assert_eq!(calc_seat_id(input).unwrap(), 119);
        let input: &str = "BBFFBBFRLL";
        assert_eq!(calc_seat_id(input).unwrap(), 820);

        let input: Vec<String> = vec!["FBFBBFFRLR".to_string(), "BFFFBBFRRR".to_string(), "FFFBBBFRRR".to_string(), "BBFFBBFRLL".to_string()];
        assert_eq!(part1(&input).unwrap(), 820);
    }
}
