use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use crate::common::{Direction, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DigEntry {
    dir: Direction,
    amount: i64,
    color: u32,
}

impl FromStr for DigEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amount, color) = s.split_whitespace().collect_tuple().ok_or(())?;
        Ok(DigEntry {
            dir: dir.chars().exactly_one().map_err(|_| ())?.try_into()?,
            amount: amount.parse().map_err(|_| ())?,
            color: u32::from_str_radix(
                color.trim_matches(|c| c == '(' || c == '#' || c == ')'),
                16,
            )
            .map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<DigEntry> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}

fn find_area(entries: &[DigEntry], f: impl Fn(&DigEntry) -> (Direction, i64)) -> i64 {
    let mut current = Vec2i::new(0, 0);
    let mut circumference = 0;
    let vertices: Vec<_> = entries
        .iter()
        .map(|d| {
            let (dir, amount) = f(d);
            circumference += amount;
            current = dir.offset_with_amount(&current, amount);
            current
        })
        .collect();

    // assumptions: loop and no crossings
    debug_assert!(vertices.last().is_some_and(|v| *v == Vec2i::new(0, 0)));
    debug_assert!(vertices.iter().all_unique());

    // shoelace formula again
    let double_area = vertices
        .iter()
        .circular_tuple_windows()
        .map(|(vp, v, vn)| v.x * (vn.y - vp.y))
        .sum::<i64>()
        .abs();

    // picks theorem again, but add circumference again to include the trench
    (double_area + circumference + 2) / 2
}

#[aoc(day18, part1)]
pub fn part1(input: &[DigEntry]) -> i64 {
    find_area(input, |d| (d.dir, d.amount))
}

#[aoc(day18, part2)]
pub fn part2(input: &[DigEntry]) -> i64 {
    find_area(input, |d| {
        (
            match d.color & 0xf {
                0 => Direction::East,
                1 => Direction::South,
                2 => Direction::West,
                3 => Direction::North,
                _ => unreachable!(),
            },
            (d.color >> 4) as i64,
        )
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 952408144115);
    }
}
