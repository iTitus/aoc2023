use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use rustc_hash::FxHashMap;

use crate::common::parse_lines;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    parse_lines(input).unwrap()
}

#[aoc(day3, part1)]
pub fn part1(input: &[String]) -> u32 {
    fn contains_symbol(s: &str) -> bool {
        s.chars().any(|c| c != '.' && !c.is_ascii_digit())
    }

    let re = Regex::new(r#"\d+"#).unwrap();

    let mut sum = 0;
    for (i, l) in input.iter().enumerate() {
        for m in re.find_iter(l) {
            let start = m.start();
            let end = m.end();

            // assume rectangular grid and ascii
            let startm1 = if start > 0 { start - 1 } else { start };
            let endp1 = if end < l.len() - 1 { end + 1 } else { end };

            let has_symbol_fn = || {
                if i > 0 {
                    let prev = &input[i - 1][startm1..endp1];
                    if contains_symbol(prev) {
                        return true;
                    }
                }

                if i < input.len() - 1 {
                    let next = &input[i + 1][startm1..endp1];
                    if contains_symbol(next) {
                        return true;
                    }
                }

                contains_symbol(&l[startm1..(startm1 + 1)])
                    || contains_symbol(&l[(endp1 - 1)..endp1])
            };

            if has_symbol_fn() {
                let n: u32 = m.as_str().parse().unwrap();
                sum += n;
            }
        }
    }

    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &[String]) -> u32 {
    fn find_star(s: &str) -> Option<usize> {
        s.chars().position(|c| c == '*')
    }

    let re = Regex::new(r#"\d+"#).unwrap();

    let mut gears: FxHashMap<(usize, usize), Vec<u32>> = FxHashMap::default();
    for (i, l) in input.iter().enumerate() {
        for m in re.find_iter(l) {
            let start = m.start();
            let end = m.end();

            // assume rectangular grid and ascii
            let startm1 = if start > 0 { start - 1 } else { start };
            let endp1 = if end < l.len() - 1 { end + 1 } else { end };

            let has_symbol_fn = || {
                if i > 0 {
                    let prev = &input[i - 1][startm1..endp1];
                    if let Some(pos) = find_star(prev) {
                        return Some((i - 1, startm1 + pos));
                    }
                }

                if i < input.len() - 1 {
                    let next = &input[i + 1][startm1..endp1];
                    if let Some(pos) = find_star(next) {
                        return Some((i + 1, startm1 + pos));
                    }
                }

                if let Some(pos) = find_star(&l[startm1..(startm1 + 1)]) {
                    return Some((i, startm1 + pos));
                }

                if let Some(pos) = find_star(&l[(endp1 - 1)..endp1]) {
                    return Some((i, endp1 - 1 + pos));
                }

                None
            };

            if let Some(gear_pos) = has_symbol_fn() {
                let n: u32 = m.as_str().parse().unwrap();
                gears.entry(gear_pos).or_default().push(n);
            }
        }
    }

    gears
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 467835);
    }
}
