use aoc_runner_derive::{aoc, aoc_generator};

fn parse_digit(c: char) -> Option<u32> {
    c.to_digit(10)
}

#[aoc_generator(day1, part1)]
pub fn input_generator_1(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|l| {
            (
                l.chars().find_map(parse_digit).unwrap(),
                l.chars().rev().find_map(parse_digit).unwrap(),
            )
        })
        .collect()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[aoc_generator(day1, part2)]
pub fn input_generator_2(input: &str) -> Vec<(u32, u32)> {
    fn first_digit(s: &str) -> Option<u32> {
        let mut first_digit: Option<(usize, u32)> = None;
        for c in '0'..='9' {
            if let Some(pos) = s.find(c) {
                if first_digit.is_none() || first_digit.is_some_and(|(min_idx, _)| pos < min_idx) {
                    first_digit = Some((pos, parse_digit(c).unwrap()));
                }
            }
        }

        for (idx, number) in NUMBERS.iter().enumerate() {
            if let Some(pos) = s.find(number) {
                if first_digit.is_none() || first_digit.is_some_and(|(min_idx, _)| pos < min_idx) {
                    first_digit = Some((pos, (idx + 1) as u32));
                }
            }
        }

        first_digit.map(|(_, digit)| digit)
    }

    fn last_digit(s: &str) -> Option<u32> {
        let mut last_digit: Option<(usize, u32)> = None;
        for c in '0'..='9' {
            if let Some(pos) = s.rfind(c) {
                if last_digit.is_none() || last_digit.is_some_and(|(min_idx, _)| pos > min_idx) {
                    last_digit = Some((pos, parse_digit(c).unwrap()));
                }
            }
        }

        for (idx, number) in NUMBERS.iter().enumerate() {
            if let Some(pos) = s.rfind(number) {
                if last_digit.is_none() || last_digit.is_some_and(|(min_idx, _)| pos > min_idx) {
                    last_digit = Some((pos, (idx + 1) as u32));
                }
            }
        }

        last_digit.map(|(_, digit)| digit)
    }

    input
        .lines()
        .map(|l| (first_digit(l).unwrap(), last_digit(l).unwrap()))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|&(first, last)| first * 10 + last).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[(u32, u32)]) -> u32 {
    input.iter().map(|&(first, last)| first * 10 + last).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_part1() {
        const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(part1(&input_generator_1(INPUT)), 142);
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(part2(&input_generator_2(INPUT)), 281);
    }
}
