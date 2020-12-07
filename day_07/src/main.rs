use anyhow::Result;
use std::collections::HashMap;
use std::cell::RefCell;
use itertools::Itertools as _;

const DAY: &str = "07";

const GOAL: &str = "shiny gold";

type InnerBags = Vec<(usize, &'static str)>;
type Bags = HashMap<&'static str, InnerBags>;

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input = include_str!("../input.txt");

    let bags = parse_input(input);
    let solver = Solver::new(bags);

    println!("Part 1:  {}", part1(&solver)?);
    println!("Part 2:  {}", part2(&solver)?);

    Ok(())
}

fn part1(solver: &Solver) -> Result<u32> {
    let result = solver
        .bags
        .keys()
        .filter(|&bag| solver.contains_wanted(bag))
        .count() as u32;

    Ok(result)
}

fn part2(solver: &Solver) -> Result<u32> {
    let result = solver
        .count_inside(GOAL) as u32;

    Ok(result)
}

fn parse_input(input: &'static str) -> Bags {
    input
        .lines()
        .map(|line| {
            let (left, right) = line
                .splitn(2, " contain ")
                .collect_tuple::<(&str, &str)>()
                .unwrap();

            let holder = left
                .rsplitn(2, ' ')
                .nth(1)
                .unwrap();

            let inner_bags = if right.starts_with("no") {
                vec![]
            } else {
                right[..right.len() - 1]
                    .split(", ")
                    .map(|bag| {
                        let (count, color) = bag
                            .rsplitn(2, ' ')
                            .nth(1)
                            .unwrap()
                            .splitn(2, ' ')
                            .collect_tuple()
                            .unwrap();

                        (count.parse::<usize>().unwrap(), color)
                    })
                    .collect::<InnerBags>()
            };

            (holder, inner_bags)
        })
        .collect()
}

struct Solver {
    bags: Bags,
    cache: RefCell<HashMap<&'static str, bool>>,
}

impl Solver {
    fn new(bags: Bags) -> Self {
        Self {
            cache: RefCell::new(HashMap::with_capacity(bags.len())),
            bags,
        }
    }

    fn contains_wanted(&self, bag: &'static str) -> bool {
        if let Some(&value) = self.cache.borrow().get(bag) {
            return value;
        }

        let value = self
            .bags
            .get(bag)
            .unwrap()
            .iter()
            .any(|&(_count, bag)| { bag == GOAL || self.contains_wanted(&bag) });

        self.cache.borrow_mut().insert(bag, value);

        value
    }

    fn count_inside(&self, bag: &'static str) -> usize {
        self.bags
            .get(bag)
            .unwrap()
            .iter()
            .map(|&(count, bag)| count + count * self.count_inside(bag))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../test_input.txt");

        let bags = parse_input(input);
        let solver = Solver::new(bags);

        assert_eq!(part1(&solver).unwrap(), 4);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../test_input2.txt");

        let bags = parse_input(input);
        let solver = Solver::new(bags);

        assert_eq!(part2(&solver).unwrap(), 126);
    }
}
