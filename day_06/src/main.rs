use std::fs::File;
use std::io::Read;

use anyhow::Result;

const DAY: &str = "06";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let mut input_file = File::open("input.txt")?;
    let mut input: String = String::new();
    input_file.read_to_string(&mut input)?;

    let input = input.split("\n\n").collect::<Vec<&str>>();


    println!("Part 1:  {}", part1(&input));
    println!("Part 2:  {}", part2(&input));

    Ok(())
}

fn part1(input: &[&str]) -> u32 {
    input.iter()
        .map(|g| { individual_questions(g) })
        .sum()
}

fn part2(input: &[&str]) -> u32 {
    input.iter()
        .map(|g| { individual_questions_part2(g) })
        .sum()
}

fn individual_questions(input: &str) -> u32 {
    let input = input.replace(' ', "").replace('\n', "");

    let mut chars = input.chars().collect::<Vec<char>>();
    chars.sort_unstable();
    chars.dedup();
    chars.len() as u32
}

fn individual_questions_part2(input: &str) -> u32 {
    let all_questions = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let input = input.replace(' ', "");
    let input = input.split('\n').collect::<Vec<&str>>();

    let mut total = 0;

    for question in all_questions {
        let mut do_count_question = 1;

        for person in &input {
            if !person.contains(question) {
                do_count_question = 0;
            }
        }

        total += do_count_question;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let input = input.split("\n\n").collect::<Vec<&str>>();

        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn test_part_2() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        let input = input.split("\n\n").collect::<Vec<&str>>();

        assert_eq!(part2(&input), 6);
    }
}
