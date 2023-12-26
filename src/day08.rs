use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::ExtendedGcd;
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
        let (instructions, graph) = s.split_once("\n\n").ok_or(())?;
        let instructions = instructions
            .trim()
            .chars()
            .map(Instruction::try_from)
            .collect::<Result<_, _>>()?;
        let graph = graph
            .trim()
            .lines()
            .map(|l| {
                let (node, children) = l.split_once('=').ok_or(())?;
                let node = node.trim().to_string();
                let children = children
                    .trim_matches(|c: char| c == '(' || c == ')' || c.is_whitespace())
                    .split(',')
                    .map(str::trim)
                    .map(|s| s.trim().to_string())
                    .collect_tuple()
                    .ok_or(())?;
                Ok((node, children))
            })
            .collect::<Result<_, _>>()?;

        Ok(Map {
            instructions,
            graph,
        })
    }
}

#[derive(Debug)]
struct LoopInformationSystem {
    infos: Vec<LoopInformation>,
}

impl LoopInformationSystem {
    fn create(map: &Map, start: impl Fn(&str) -> bool, end: impl Fn(&str) -> bool) -> Self {
        Self {
            infos: map
                .graph
                .keys()
                .filter(|n| start(n.as_str()))
                .map(|start| {
                    // (node, instruction_offset) -> index
                    let mut all_nodes: FxHashMap<(&str, usize), usize> = FxHashMap::default();
                    let mut current = start.as_str();
                    for (n, (instruction_offset, instruction)) in
                        map.instructions.iter().enumerate().cycle().enumerate()
                    {
                        if let Some(&loop_start) = all_nodes.get(&(current, instruction_offset)) {
                            // possible optimization: use symmetries in instructions & graph to reduce the cycle length
                            // for that we need to find the shortest subcycle by just looking at the nodes, not the instruction offset
                            let statics = all_nodes
                                .iter()
                                .filter(|((node, _), index)| **index < loop_start && end(node))
                                .map(|(_, index)| *index as i64)
                                .sorted()
                                .collect();
                            let loop_length = (n - loop_start) as i64;
                            let dynamics = all_nodes
                                .iter()
                                .filter(|((node, _), index)| **index >= loop_start && end(node))
                                .map(|(_, index)| *index as i64)
                                .collect();
                            return LoopInformation {
                                statics,
                                loop_length,
                                dynamics,
                            };
                        } else {
                            all_nodes.insert((current, instruction_offset), n);
                        }

                        let children = &map.graph[current];
                        current = match instruction {
                            Instruction::L => children.0.as_str(),
                            Instruction::R => children.1.as_str(),
                        };
                    }

                    unreachable!();
                })
                .collect(),
        }
    }

    fn has_dynamic_solution(&self) -> bool {
        !self.infos.is_empty() && self.infos.iter().all(LoopInformation::has_dynamic_solution)
    }

    fn is_solution(&self, n: &i64) -> bool {
        !self.infos.is_empty() && self.infos.iter().all(|li| li.is_solution(n))
    }

    fn solve(&self) -> Option<i64> {
        // first try the statics
        if let Some(n) = self
            .infos
            .iter()
            .flat_map(|li| li.statics.iter())
            .find(|n| self.is_solution(n))
        {
            return Some(*n);
        }

        // then check the dynamics
        if !self.has_dynamic_solution() {
            return None;
        }

        fn solve_two_lc(a: &LinearCongruence, b: &LinearCongruence) -> Option<LinearCongruence> {
            // Chinese Remainder Theorem
            let (ExtendedGcd { gcd, x, y: _y }, lcm) = a.modulus.extended_gcd_lcm(&b.modulus);
            if a.value.rem_euclid(gcd) != b.value.rem_euclid(gcd) {
                None
            } else {
                // alternatively:
                // (a.value * _y * b.modulus + b.value * x * a.modulus) / gcd;
                Some(LinearCongruence::new_with_minimum(
                    a.value - x * a.modulus * (a.value - b.value) / gcd,
                    lcm,
                    a.minimum.max(b.minimum),
                ))
            }
        }

        fn solve_lcs(lcs: &[LinearCongruence]) -> Option<i64> {
            lcs.iter()
                .try_fold(LinearCongruence::default(), |acc, x| solve_two_lc(&acc, x))
                .map(|lc| lc.find_solution())
        }

        self.infos
            .iter()
            .map(|li| {
                li.dynamics
                    .iter()
                    .map(|n| LinearCongruence::new(*n, li.loop_length))
            })
            .multi_cartesian_product()
            .filter_map(|lcs| solve_lcs(&lcs))
            .min()
    }
}

#[derive(Debug)]
struct LoopInformation {
    statics: Vec<i64>,
    loop_length: i64,
    dynamics: Vec<i64>,
}

impl LoopInformation {
    fn has_dynamic_solution(&self) -> bool {
        !self.dynamics.is_empty()
    }

    fn is_solution(&self, n: &i64) -> bool {
        self.statics.binary_search(n).is_ok()
            || self
                .dynamics
                .iter()
                .map(|v| LinearCongruence::new(*v, self.loop_length))
                .any(|lc| lc.is_solution(n))
    }
}

#[derive(Debug, Copy, Clone)]
struct LinearCongruence {
    value: i64,
    modulus: i64,
    minimum: i64,
}

impl Default for LinearCongruence {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl LinearCongruence {
    fn new(value: i64, modulus: i64) -> Self {
        Self::new_with_minimum(value, modulus, value)
    }

    fn new_with_minimum(value: i64, modulus: i64, minimum: i64) -> Self {
        debug_assert!(modulus >= 1);
        debug_assert!(minimum >= 0);
        Self {
            value: value.rem_euclid(modulus),
            modulus,
            minimum,
        }
    }

    fn is_solution(&self, n: &i64) -> bool {
        *n >= self.minimum && n.rem_euclid(self.modulus) == self.value
    }

    fn find_solution(&self) -> i64 {
        let mut n = self.minimum / self.modulus;
        loop {
            let sol = self
                .modulus
                .checked_mul(n)
                .unwrap()
                .checked_add(self.value)
                .unwrap();
            if sol >= self.minimum {
                return sol;
            }

            n += 1;
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
pub fn part1(input: &Map) -> i64 {
    let lis = LoopInformationSystem::create(input, |n| n == "AAA", |n| n == "ZZZ");
    lis.solve().unwrap()
}

#[aoc(day8, part2)]
pub fn part2(input: &Map) -> i64 {
    let lis = LoopInformationSystem::create(input, |n| n.ends_with('A'), |n| n.ends_with('Z'));
    lis.solve().unwrap()
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

    /// https://www.reddit.com/r/adventofcode/comments/18gx9la/2023_day_8_part_2_pathological_inputs_spoilers/
    const INPUT_4: &str = r#"LLLLLR

11A = (11B, 11C)
11B = (11C, 11C)
11C = (11Z, 11Z)
11Z = (11E, 11E)
11E = (11F, 11F)
11F = (11B, 11B)
22A = (22B, 22Z)
22B = (22Z, 22D)
22Z = (22A, 22A)
22D = (22D, 22D)"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 2);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_3)), 6);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_4)), 8);
    }
}
