use std::cmp::Reverse;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Num = u32;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<Num>> {
    let all_calories = input
        .lines()
        .map(|l| {
            if l.is_empty() {
                0
            } else {
                let result = Num::from_str(l).unwrap();
                if result == 0 {
                    panic!();
                }

                result
            }
        }).collect_vec();

    let mut calories_grouped = vec![];
    let mut current_group = vec![];
    for calorie in all_calories {
        if calorie != 0 {
            current_group.push(calorie);
        } else {
            calories_grouped.push(current_group);
            current_group = vec![];
        }
    }

    if !current_group.is_empty() {
        calories_grouped.push(current_group);
    }

    calories_grouped
}

#[aoc(day1, part1)]
pub fn part1(input: &[Vec<Num>]) -> Num {
    input.iter()
        .map(|calories| calories.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &[Vec<Num>]) -> Num {
    input.iter()
        .map(|calories| calories.iter().sum())
        .sorted_unstable_by_key(|c: &Num| Reverse(*c))
        .take(3)
        .sum()
}
