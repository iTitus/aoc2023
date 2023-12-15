use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse())
                .process_results(|it| it.collect())
        })
        .process_results(|it| it.collect())
        .unwrap()
}

fn find_next(mut v: Vec<i32>, mut acc: i32) -> i32 {
    let mut v2 = Vec::with_capacity(v.len());
    loop {
        if v.iter().all(|&n| n == 0) {
            return acc;
        }

        acc += v.last().unwrap();
        v2.clear();
        v2.extend(v.iter().tuple_windows().map(|(&a, &b)| b - a));
        std::mem::swap(&mut v, &mut v2);
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &[Vec<i32>]) -> i32 {
    input.iter().map(|s| find_next(s.to_vec(), 0)).sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|s| find_next(s.iter().copied().rev().collect(), 0))
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 2);
    }
}
