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

    let input: Vec<Instruction> = common::input_vec(buf_reader)?;

    println!("Part 1:  {}", part1(&input)?);
    println!("Part 2:  {}", part2(&input)?);

    Ok(())
}

fn part1(instructions: &[Instruction]) -> Result<u32> {
    let mut ship = Ship {
        facing: 90,
        y: 0,
        x: 0,
    };

    for instruction in instructions {
        match instruction.action {
            'N' => { ship.y += instruction.value }
            'S' => { ship.y -= instruction.value }
            'E' => { ship.x += instruction.value }
            'W' => { ship.x -= instruction.value }
            'L' => { ship.rotate(instruction.value * -1) }
            'R' => { ship.rotate(instruction.value) }
            'F' => { ship.forward(instruction.value) }
            i => { unreachable!("Unknown instruction {}", i); }
        }
    }

    Ok(ship.manhattan_distance())
}

fn part2(instructions: &[Instruction]) -> Result<u32> {
    let mut ship = Ship {
        facing: 0,
        y: 0,
        x: 0,
    };

    let mut waypoint = Waypoint {
        y: 1,
        x: 10,
    };

    for instruction in instructions {
        match instruction.action {
            'N' => { waypoint.y += instruction.value }
            'S' => { waypoint.y -= instruction.value }
            'E' => { waypoint.x += instruction.value }
            'W' => { waypoint.x -= instruction.value }
            'L' => { waypoint.rotate_around(ship.x, ship.y, instruction.value * -1) }
            'R' => { waypoint.rotate_around(ship.x, ship.y, instruction.value) }
            'F' => { ship.move_to_waypoint(&mut waypoint, instruction.value) }
            i => { unreachable!("Unknown instruction {}", i); }
        }
    }

    Ok(ship.manhattan_distance())
}

struct Waypoint {
    y: i32,
    x: i32,
}

impl Waypoint {
    fn rotate_around(&mut self, x: i32, y: i32, degrees: i32) {
        let new_waypoint_x: i32;
        let new_waypoint_y: i32;

        let degrees = match degrees {
            -90 => 270,
            -180 => 180,
            -270 => 90,
            d => d,
        };

        match degrees {
            90 => {
                new_waypoint_x = self.y - y;
                new_waypoint_y = (self.x - x) * -1
            }
            180 => {
                new_waypoint_x = (self.x - x) * -1;
                new_waypoint_y = (self.y - y) * -1;
            }
            270 => {
                new_waypoint_x = (self.y - y) * -1;
                new_waypoint_y = self.x - x;
            }
            _ => { unreachable!(); }
        }

        self.x = new_waypoint_x + x;
        self.y = new_waypoint_y + y;
    }
}

struct Ship {
    facing: u32,
    y: i32,
    x: i32,
}

impl Ship {
    fn move_to_waypoint(&mut self, waypoint: &mut Waypoint, times: i32) {
        let dx = waypoint.x - self.x;
        let dy = waypoint.y - self.y;

        self.x += dx * times;
        self.y += dy * times;
        waypoint.x += dx * times;
        waypoint.y += dy * times;
    }

    fn manhattan_distance(&self) -> u32 {
        self.y.abs() as u32 + self.x.abs() as u32
    }

    fn forward(&mut self, distance: i32) {
        match self.facing {
            0 => { self.y += distance }
            90 => { self.x += distance }
            180 => { self.y -= distance }
            270 => { self.x -= distance }
            _ => { unreachable!("Facing not valid!"); }
        }
    }

    fn rotate(&mut self, degrees: i32) {
        let mut new_facing = self.facing as i32 + degrees;

        if new_facing >= 360 {
            new_facing -= 360;
        }
        if new_facing < 0 {
            new_facing += 360;
        }

        self.facing = new_facing as u32;
    }
}

struct Instruction {
    action: char,
    value: i32,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (action, value) = s.split_at(1);
        let action = action.chars().next().unwrap();
        let value: i32 = value.parse()?;

        Ok(Self {
            action,
            value,
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

        assert_eq!(part1(&input).unwrap(), 25);
    }

    #[test]
    fn test_part_2() {
        let input_file = File::open("test_input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);

        let input: Vec<Instruction> = common::input_vec(buf_reader).unwrap();

        assert_eq!(part2(&input).unwrap(), 286);
    }
}
