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
                    // (node, pattern_offset) -> (index, looped, end)
                    let mut all_nodes: FxHashMap<(&str, usize), (usize, bool, bool)> =
                        FxHashMap::default();
                    let mut dynamics = vec![];

                    let mut current = start.as_str();
                    for (n, (pattern_offset, instruction)) in
                        map.instructions.iter().enumerate().cycle().enumerate()
                    {
                        all_nodes
                            .entry((current, pattern_offset))
                            .and_modify(|(index, looped, end)| {
                                if !*looped {
                                    if *end {
                                        let first_occurrence = *index as i64;
                                        let latest_occurrence = n as i64;
                                        dynamics.push(LinearCongruence::new(
                                            first_occurrence,
                                            latest_occurrence - first_occurrence,
                                            first_occurrence,
                                        ));
                                    }

                                    *looped = true;
                                }
                            })
                            .or_insert((n, false, end(current)));

                        if all_nodes
                            .values()
                            .max_by_key(|(index, _, _)| *index)
                            .is_some_and(|(_, looped, _)| *looped)
                        {
                            break;
                        }

                        let children = &map.graph[current];
                        current = match instruction {
                            Instruction::L => children.0.as_str(),
                            Instruction::R => children.1.as_str(),
                        };
                    }

                    let statics = all_nodes
                        .values()
                        .filter(|(_, looped, end)| *end && !*looped)
                        .map(|(index, _, _)| *index as i64)
                        .sorted()
                        .collect();
                    LoopInformation { statics, dynamics }
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
            let (ExtendedGcd { gcd, x, .. }, lcm) = a.modulus.extended_gcd_lcm(&b.modulus);
            if a.value.rem_euclid(gcd) != b.value.rem_euclid(gcd) {
                None
            } else {
                Some(LinearCongruence::new(
                    a.value - x * a.modulus * (a.value - b.value) / gcd,
                    lcm,
                    a.minimum.max(b.minimum),
                ))
            }
        }

        fn solve_lcs(lcs: &[&LinearCongruence]) -> Option<i64> {
            let mut sol: LinearCongruence = *lcs[0];
            for lc in &lcs[1..] {
                if let Some(partial_sol) = solve_two_lc(&sol, lc) {
                    sol = partial_sol;
                } else {
                    return None;
                }
            }

            Some(sol.find_solution())
        }

        self.infos
            .iter()
            .map(|li| li.dynamics.iter())
            .multi_cartesian_product()
            .filter_map(|lcs| solve_lcs(&lcs))
            .min()
    }
}

#[derive(Debug)]
struct LoopInformation {
    statics: Vec<i64>,
    dynamics: Vec<LinearCongruence>,
}

impl LoopInformation {
    fn has_dynamic_solution(&self) -> bool {
        !self.dynamics.is_empty()
    }

    fn is_solution(&self, n: &i64) -> bool {
        self.statics.binary_search(n).is_ok() || self.dynamics.iter().any(|lc| lc.is_solution(n))
    }
}

#[derive(Debug, Copy, Clone)]
struct LinearCongruence {
    value: i64,
    modulus: i64,
    minimum: i64,
}

impl LinearCongruence {
    fn new(value: i64, modulus: i64, minimum: i64) -> Self {
        assert!(minimum >= 0);
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
        // TODO: optimize and safeguard against overflow
        let mut n = self.minimum / self.modulus;
        loop {
            let sol = n * self.modulus + self.value;
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
        };
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

#[aoc(day8, part2, general)]
pub fn part2_general(input: &Map) -> i64 {
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
