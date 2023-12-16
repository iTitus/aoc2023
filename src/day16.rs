use std::collections::HashMap;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    ForwardMirror,
    BackwardMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '/' => Self::ForwardMirror,
            '\\' => Self::BackwardMirror,
            '|' => Self::VerticalSplitter,
            '-' => Self::HorizontalSplitter,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Platform {
    grid: Vec<Vec<Tile>>,
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Platform {
            grid: s
                .lines()
                .map(str::trim)
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(Tile::try_from).collect::<Result<_, _>>())
                .collect::<Result<_, _>>()?,
        })
    }
}

impl Platform {
    fn size_x(&self) -> usize {
        self.grid[0].len()
    }

    fn size_y(&self) -> usize {
        self.grid.len()
    }
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Platform {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
pub fn part1(_input: &Platform) -> usize {
    0
}

#[aoc(day16, part2)]
pub fn part2(_input: &Platform) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 0);
    }
}
