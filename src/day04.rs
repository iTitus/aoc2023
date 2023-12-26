use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rustc_hash::FxHashSet;

use crate::common::{parse_lines, parse_split_whitespace};

#[derive(Debug)]
pub struct Card {
    _id: u32,
    winning_numbers: FxHashSet<u32>,
    my_numbers: FxHashSet<u32>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("Card") else {
            return Err(());
        };
        let Some((id, numbers)) = s.splitn(2, ':').collect_tuple() else {
            return Err(());
        };

        let Some((winning_numbers, my_numbers)) = numbers.splitn(2, '|').collect_tuple() else {
            return Err(());
        };

        Ok(Card {
            _id: id.trim().parse().map_err(|_| ())?,
            winning_numbers: parse_split_whitespace(winning_numbers).map_err(|_| ())?,
            my_numbers: parse_split_whitespace(my_numbers).map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Card> {
    parse_lines(input).unwrap()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Card]) -> u32 {
    input
        .iter()
        .map(|c| {
            let n = c.winning_numbers.intersection(&c.my_numbers).count() as u32;
            if n == 0 {
                0
            } else {
                1 << (n - 1)
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Card]) -> u32 {
    let l = input.len();
    let mut counters = vec![1; l];
    for i in 0..l {
        let amount = counters[i];
        let c = &input[i];
        let win_amount = c.winning_numbers.intersection(&c.my_numbers).count();

        // assume there won't be an out of bounds error
        for c in &mut counters[(i + 1)..=(i + win_amount)] {
            *c += amount;
        }
    }

    counters.iter().sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 30);
    }
}
