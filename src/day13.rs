use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ground {
    Dot,
    Hash,
}

impl TryFrom<char> for Ground {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Ground::Dot,
            '#' => Ground::Hash,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug)]
pub struct Pattern {
    size_x: usize,
    size_y: usize,
    pattern: Vec<Vec<Ground>>,
}

impl Pattern {
    fn get(&self, pos: (usize, usize), smudge: Option<(usize, usize)>) -> Ground {
        let p = self.pattern[pos.1][pos.0];
        match smudge {
            None => p,
            Some(s) if pos != s => p,
            _ => match p {
                Ground::Dot => Ground::Hash,
                Ground::Hash => Ground::Dot,
            },
        }
    }

    fn is_x_mirror(&self, x: &usize, smudge: Option<(usize, usize)>) -> bool {
        (0..*x).rev().zip(*x..self.size_x).all(|(x1, x2)| {
            (0..self.size_y).all(|y| self.get((x1, y), smudge) == self.get((x2, y), smudge))
        })
    }

    fn is_y_mirror(&self, y: &usize, smudge: Option<(usize, usize)>) -> bool {
        (0..*y).rev().zip(*y..self.size_y).all(|(y1, y2)| {
            (0..self.size_x).all(|x| self.get((x, y1), smudge) == self.get((x, y2), smudge))
        })
    }

    fn find_x_reflections(
        &self,
        smudge: Option<(usize, usize)>,
    ) -> impl Iterator<Item = usize> + '_ {
        (1..self.size_x).filter(move |x| self.is_x_mirror(x, smudge))
    }

    fn find_y_reflections(
        &self,
        smudge: Option<(usize, usize)>,
    ) -> impl Iterator<Item = usize> + '_ {
        (1..self.size_y).filter(move |y| self.is_y_mirror(y, smudge))
    }

    fn find_reflection(&self) -> usize {
        if let Some(y) = self.find_y_reflections(None).next() {
            100 * y
        } else {
            self.find_x_reflections(None).next().unwrap()
        }
    }

    fn find_reflection_with_smudge(&self) -> usize {
        let existing_x = self.find_x_reflections(None).next();
        let existing_y = self.find_y_reflections(None).next();
        (0..self.size_x)
            .cartesian_product(0..self.size_y)
            .filter_map(|smudge| {
                if let Some(y) = self
                    .find_y_reflections(Some(smudge))
                    .find(|y| existing_y.is_none() || *y != existing_y.unwrap())
                {
                    Some(100 * y)
                } else {
                    self.find_x_reflections(Some(smudge))
                        .find(|x| existing_x.is_none() || *x != existing_x.unwrap())
                }
            })
            .next()
            .unwrap()
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(Ground::try_from)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, ()>>()?;
        Ok(Pattern {
            size_x: pattern[0].len(),
            size_y: pattern.len(),
            pattern,
        })
    }
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Pattern]) -> usize {
    input.iter().map(Pattern::find_reflection).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Pattern]) -> usize {
    input.iter().map(Pattern::find_reflection_with_smudge).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 405)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 400)
    }
}
