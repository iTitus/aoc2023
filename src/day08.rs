use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::Integer;
use rustc_hash::FxHashMap;

#[derive(Debug)]
pub enum Instruction {
    L,
    R,
}

impl TryFrom<char> for Instruction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            'L' => Self::L,
            'R' => Self::R,
            _ => return Err(()),
        })
    }
}

#[derive(Debug)]
pub struct Map {
    instructions: Vec<Instruction>,
    graph: FxHashMap<String, (String, String)>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, graph) = s.split("\n\n").collect_tuple().ok_or(())?;
        let instructions = instructions
            .trim()
            .chars()
            .map(Instruction::try_from)
            .process_results(|it| it.collect())
            .map_err(|_| ())?;
        let graph = graph
            .trim()
            .lines()
            .map(|l| {
                let (node, children) = l.split_once('=').ok_or(())?;
                let (l, r) = children
                    .trim()
                    .strip_prefix('(')
                    .ok_or(())?
                    .strip_suffix(')')
                    .ok_or(())?
                    .split_once(',')
                    .ok_or(())?;
                return Ok((
                    node.trim().to_string(),
                    (l.trim().to_string(), r.trim().to_string()),
                ));
            })
            .process_results(|it| it.collect())
            .map_err(|_: ()| ())?;

        Ok(Map {
            instructions,
            graph,
        })
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    input.parse().unwrap()
}

fn get_path_length(map: &Map, start: &str, end: impl Fn(&str) -> bool) -> usize {
    let mut current = start;
    for (n, instruction) in map.instructions.iter().cycle().enumerate() {
        if end(current) {
            return n;
        }

        let children = &map.graph[current];
        current = match instruction {
            Instruction::L => children.0.as_str(),
            Instruction::R => children.1.as_str(),
        }
    }

    unreachable!();
}

#[aoc(day8, part1)]
pub fn part1(input: &Map) -> usize {
    get_path_length(input, "AAA", |n| n == "ZZZ")
}

#[aoc(day8, part2)]
pub fn part2(input: &Map) -> usize {
    // assume all start positions end up in a loop of constant length N around a "__Z" node after exactly N steps
    // assume that all the loop lengths work out with the instruction count
    // this is undocumented but should hold for all inputs
    input
        .graph
        .keys()
        .filter(|n| n.ends_with('A'))
        .map(|n| get_path_length(input, n, |n| n.ends_with('Z')))
        .fold(1, |a, e| a.lcm(&e))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

    const INPUT_2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

    const INPUT_3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 2)
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 6)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_3)), 6)
    }
}
