use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use std::collections::HashMap;
use std::cell::RefCell;
use itertools::Itertools as _;

const DAY: &str = "08";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input_file = File::open("input.txt")?;
    let buf_reader = BufReader::new(input_file);

    let input: Vec<Instruction> = common::input_vec(buf_reader)?;
    let mut runner = Runner::new(input);

    println!("Part 1:  {}", part1(&mut runner)?);
    println!("Part 2:  {}", part2(&mut runner)?);

    Ok(())
}

fn part1(runner: &mut Runner) -> Result<i64> {
    let (_, acc) = runner.check_bootloop();
    Ok(acc)
}

fn part2(runner: &mut Runner) -> Result<i64> {
    for i in 0..runner.instructions.len() {
        runner.reset();

        runner.instructions.get_mut(i).unwrap().switch();

        let (looped, _) = runner.check_bootloop();

        if looped {
            runner.instructions.get_mut(i).unwrap().switch();
        } else {
            return Ok(runner.accumulator);
        }
    }

    Err(anyhow::Error::msg("No change fixed it!"))
}

struct Runner {
    instructions: Vec<Instruction>,
    accumulator: i64,
    position: u32,
    cache: RefCell<HashMap<u32, bool>>,
}

impl Runner {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            cache: RefCell::new(HashMap::with_capacity(instructions.len())),
            instructions,
            accumulator: 0,
            position: 0,
        }
    }

    fn reset(&mut self) {
        self.cache = RefCell::new(HashMap::with_capacity(self.instructions.len()));
        self.accumulator = 0;
        self.position = 0;
    }

    fn check_bootloop(&mut self) -> (bool, i64) {
        while self.position < self.instructions.len() as u32 {

            // Check cache
            if let Some(_executed) = self.cache.borrow().get(&self.position) {
                return (true, self.accumulator);
            }

            let instruction = self.instructions.get(self.position as usize).unwrap();

            self.cache.borrow_mut().insert(self.position, true);

            match instruction.operation.as_str() {
                "acc" => {
                    self.accumulator += instruction.argument as i64;
                    self.position += 1;
                }
                "jmp" => {
                    self.position = (self.position as i32 + instruction.argument) as u32;
                }
                _ => {
                    self.position += 1;
                }
            }
        }

        (false, self.accumulator)
    }
}

struct Instruction {
    operation: String,
    argument: i32,
}

impl Instruction {
    fn switch(&mut self) {
        match self.operation.as_str() {
            "jmp" => { self.operation = String::from("nop"); }
            "nop" => { self.operation = String::from("jmp"); }
            _ => {}
        }
    }
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s
            .splitn(2, ' ')
            .collect_tuple::<(&str, &str)>()
            .unwrap();

        Ok(Self {
            operation: String::from(op),
            argument: arg.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<Instruction> = common::input_vec(buf_reader).unwrap();
        let mut runner = Runner::new(input);

        assert_eq!(part1(&mut runner).unwrap(), 5);
    }

    #[test]
    fn test_part_2() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<Instruction> = common::input_vec(buf_reader).unwrap();
        let mut runner = Runner::new(input);

        assert_eq!(part2(&mut runner).unwrap(), 8);
    }
}
