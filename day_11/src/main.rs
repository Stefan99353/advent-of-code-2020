use anyhow::Result;
use std::str::FromStr;

const DAY: &str = "11";

fn main() -> Result<()> {
    println!("-----  Advent of Code -- Day {}  -----", DAY);
    println!("--------------------------------------");
    println!();

    let input = include_str!("../input.txt");

    let tiles = Tiles::from_str(input)?;

    println!("Part 1:  {}", part1(&tiles)?);
    println!("Part 2:  {}", part2(&tiles)?);

    Ok(())
}

fn part1(tiles: &Tiles) -> Result<u32> {
    let mut tiles = tiles.clone();

    let count = loop {
        let new_tiles = tiles.tick();
        if new_tiles == tiles {
            break new_tiles
                .tiles
                .into_iter()
                .filter(|t| *t == Tile::Taken)
                .count();
        }
        tiles = new_tiles;
    };

    Ok(count as u32)
}

fn part2(tiles: &Tiles) -> Result<u32> {
    let mut tiles = tiles.clone();
    let line_of_sight_map = tiles.get_line_of_sight_map();

    let count = loop {
        let new_tiles = tiles.tick_with_line_of_sight(&line_of_sight_map);
        if new_tiles == tiles {
            break new_tiles
                .tiles
                .into_iter()
                .filter(|t| *t == Tile::Taken)
                .count();
        }
        tiles = new_tiles;
    };

    Ok(count as u32)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Tiles {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Tiles {
    fn tick(&self) -> Self {
        let mut new_tiles = Self {
            tiles: vec![Tile::Floor; self.tiles.len()],
            width: self.width,
            height: self.height,
        };

        for y in 0..new_tiles.height {
            for x in 0..new_tiles.width {
                let idx = y * self.width + x;
                new_tiles.tiles[idx] = match self.tiles[idx] {
                    Tile::Empty => {
                        if self.get_neighbours(x, y).any(|t| t == Tile::Taken) {
                            Tile::Empty
                        } else {
                            Tile::Taken
                        }
                    }
                    Tile::Taken => {
                        if self
                            .get_neighbours(x, y)
                            .filter(|t| *t == Tile::Taken)
                            .count()
                            >= 4
                        {
                            Tile::Empty
                        } else {
                            Tile::Taken
                        }
                    }
                    t => t,
                };
            }
        }

        new_tiles
    }

    fn tick_with_line_of_sight(&self, line_of_sight: &[Vec<usize>]) -> Tiles {
        let mut new_tiles = Self {
            tiles: vec![Tile::Floor; self.tiles.len()],
            width: self.width,
            height: self.height,
        };

        for y in 0..new_tiles.height {
            for x in 0..new_tiles.width {
                let idx = y * self.width + x;
                new_tiles.tiles[idx] = match self.tiles[idx] {
                    Tile::Empty => {
                        if line_of_sight[idx]
                            .iter()
                            .map(|idx| self.tiles[*idx])
                            .any(|t| t == Tile::Taken)
                        {
                            Tile::Empty
                        } else {
                            Tile::Taken
                        }
                    }
                    Tile::Taken => {
                        if line_of_sight[idx]
                            .iter()
                            .map(|idx| self.tiles[*idx])
                            .filter(|t| *t == Tile::Taken)
                            .count()
                            >= 5
                        {
                            Tile::Empty
                        } else {
                            Tile::Taken
                        }
                    }
                    t => t,
                }
            }
        }

        new_tiles
    }

    fn get_neighbours(&self, x: usize, y: usize) -> impl Iterator<Item=Tile> + '_ {
        (-1..=1)
            .flat_map(move |dx| (-1..=1).map(move |dy| (x as isize + dx, y as isize + dy)))
            .filter(move |(nx, ny)| *nx != x as isize || *ny != y as isize)
            .map(move |(x, y)| {
                if x >= self.width as isize || x < 0 || y >= self.height as isize || y < 0 {
                    Tile::OutOfBounds
                } else {
                    self.tiles[y as usize * self.width + x as usize]
                }
            })
    }

    fn get_line_of_sight_map(&self) -> Vec<Vec<usize>> {
        (0..self.height)
            .into_iter()
            .flat_map(|y| (0..self.width).into_iter().map(move |x| (x, y)))
            .map(|(x, y)| {
                (-1..=1)
                    .into_iter()
                    .flat_map(|dx| (-1..=1).into_iter().map(move |dy| (dx, dy)))
                    .filter(move |(dx, dy)| *dx != 0 || *dy != 0)
                    .filter_map(|(dx, dy)| self.get_first_seat_in_direction(x, y, dx, dy))
                    .map(|(x, y)| y * self.width + x)
                    .collect()
            })
            .collect()
    }

    fn get_first_seat_in_direction(
        &self,
        x: usize,
        y: usize,
        dx: isize,
        dy: isize,
    ) -> Option<(usize, usize)> {
        let mut x = x as isize + dx;
        let mut y = y as isize + dy;
        while x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            match self.tiles[y as usize * self.width + x as usize] {
                Tile::Taken | Tile::Empty => return Some((x as usize, y as usize)),
                _ => {}
            }
            x += dx;
            y += dy;
        }
        None
    }
}

impl std::str::FromStr for Tiles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Vec<Tile>> = s.lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Floor,
                        'L' => Tile::Empty,
                        '#' => Tile::Taken,
                        t => unreachable!("{}", t),
                    })
                    .collect()
            })
            .collect();

        let width = tiles[0].len();
        let height = tiles.len();

        let tiles = Self {
            tiles: tiles.into_iter().flat_map(|v| v.into_iter()).collect(),
            width,
            height,
        };

        Ok(tiles)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Taken,
    OutOfBounds,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../test_input.txt");

        let tiles = Tiles::from_str(input).unwrap();

        assert_eq!(part1(&tiles).unwrap(), 37);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../test_input.txt");

        let tiles = Tiles::from_str(input).unwrap();

        assert_eq!(part2(&tiles).unwrap(), 26);
    }
}
