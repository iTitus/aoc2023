use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nalgebra::DMatrix;

use crate::common::{parse_lines, parse_split};

#[derive(Debug, Copy, Clone)]
pub enum Spring {
    Dot,
    Hash,
    Question,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Spring::Dot,
            '#' => Spring::Hash,
            '?' => Spring::Question,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug)]
pub struct Springs {
    springs: Vec<Spring>,
    amounts: Vec<u32>,
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (springs, amounts) = s.split_whitespace().collect_tuple().ok_or(())?;
        Ok(Springs {
            springs: springs
                .chars()
                .map(Spring::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            amounts: parse_split(amounts, ',').map_err(|_| ())?,
        })
    }
}

impl Springs {
    fn unfold(&self, n: usize) -> Springs {
        let mut springs = Vec::with_capacity(if n == 0 {
            0
        } else {
            n * self.springs.len() + (n - 1)
        });
        for i in 0..n {
            if i > 0 {
                springs.push(Spring::Question);
            }
            springs.extend_from_slice(&self.springs);
        }

        Springs {
            springs,
            amounts: self.amounts.repeat(n),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Springs> {
    parse_lines(input).unwrap()
}

fn count_alignments(springs: &Springs) -> usize {
    // dynamic programming solution
    // taking inspiration from DNA alignment matching

    #[derive(Copy, Clone)]
    enum PatternChar {
        DotStar,
        Dot,
        Hash,
    }

    let mut pattern = vec![PatternChar::DotStar];
    let mut first = true;
    for n in &springs.amounts {
        if first {
            first = false;
        } else {
            pattern.extend([PatternChar::Dot, PatternChar::DotStar]);
        }
        for _ in 0..*n {
            pattern.push(PatternChar::Hash);
        }
    }
    pattern.push(PatternChar::DotStar);

    let pl = pattern.len();
    let hl = springs.springs.len();
    let mut m = DMatrix::from_element(pl + 1, hl + 1, 0);

    for r in (0..=pl).rev() {
        for c in (0..=hl).rev() {
            if r == pl && c == hl {
                m[(r, c)] = 1;
            } else if c == hl {
                m[(r, c)] = match pattern[r] {
                    PatternChar::DotStar => m[(r + 1, c)],
                    _ => 0,
                }
            } else if r == pl {
                m[(r, c)] = 0
            } else {
                m[(r, c)] = match (pattern[r], springs.springs[c]) {
                    (PatternChar::DotStar, Spring::Dot) => m[(r, c + 1)],
                    (PatternChar::DotStar, Spring::Hash) => m[(r + 1, c)],
                    (PatternChar::DotStar, Spring::Question) => m[(r, c + 1)] + m[(r + 1, c)],
                    (PatternChar::Dot, Spring::Dot | Spring::Question)
                    | (PatternChar::Hash, Spring::Hash | Spring::Question) => m[(r + 1, c + 1)],
                    _ => 0,
                }
            }
        }
    }

    m[(0, 0)]
}

#[aoc(day12, part1)]
pub fn part1(input: &[Springs]) -> usize {
    input.iter().map(count_alignments).sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Springs]) -> usize {
    input
        .iter()
        .map(|s| s.unfold(5))
        .map(|s| count_alignments(&s))
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT_1: &str = r#"#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1"#;

    const INPUT_2: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_part1_1() {
        assert_eq!(part1(&input_generator(INPUT_1)), 6);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 525152);
    }
}
