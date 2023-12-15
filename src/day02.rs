use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Draw {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(", ")
            .try_fold(Draw::default(), |mut a, e| {
                let Some((n, color)) = e.splitn(2, ' ').collect_tuple() else {
                    return Err(());
                };

                let n: u32 = n.parse().map_err(|_| ())?;
                match color {
                    "red" => a.red += n,
                    "green" => a.green += n,
                    "blue" => a.blue += n,
                    _ => {
                        return Err(());
                    }
                }

                Ok(a)
            })
            .map_err(|_| ())
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("Game ") else {
            return Err(());
        };
        let Some((id, draws)) = s.splitn(2, ": ").collect_tuple() else {
            return Err(());
        };

        Ok(Game {
            id: id.parse().map_err(|_| ())?,
            draws: draws
                .split("; ")
                .map(|draw| draw.parse())
                .process_results(|it| it.collect())
                .map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> u32 {
    input
        .iter()
        .filter(|g| {
            g.draws
                .iter()
                .all(|d| d.red <= 12 && d.green <= 13 && d.blue <= 14)
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let red = g.draws.iter().map(|d| d.red).max().unwrap_or_default();
            let green = g.draws.iter().map(|d| d.green).max().unwrap_or_default();
            let blue = g.draws.iter().map(|d| d.blue).max().unwrap_or_default();
            red * green * blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 2286);
    }
}
