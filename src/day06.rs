use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Race> {
    fn parse_numbers(s: &str) -> Vec<u64> {
        s.split_whitespace()
            .skip(1)
            .map(|n| n.parse())
            .process_results(|it| it.collect())
            .unwrap()
    }

    let (times, distances) = input.trim().lines().collect_tuple().unwrap();
    let times = parse_numbers(times);
    let distances = parse_numbers(distances);

    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn count_better_button_times(total_time: u64, distance_to_beat: u64) -> u64 {
    // we want to solve the inequality distance_travelled(total_time, button_time) > distance_to_beat
    // distance_travelled(total_time, button_time) = -button_time^2 + total_time*button_time
    // solve the equality distance_travelled(total_time, button_time) = distance_to_beat, which has 2 solutions
    // the inequality holds between those
    // then use smart rounding to count the integer values between those roots
    let minus_p_half = total_time as f64 / 2.0;
    let p_half_sq = minus_p_half.powi(2);
    if p_half_sq <= distance_to_beat as f64 {
        return 0;
    }

    let disc_sqrt = (p_half_sq - distance_to_beat as f64).sqrt();
    let t1 = minus_p_half + disc_sqrt;
    let t2 = minus_p_half - disc_sqrt;
    // we always have 0 <= t2 < t1

    let t1_i = (t1 - 1.0).ceil() as u64;
    let t2_i = (t2 + 1.0).floor() as u64;
    if t1_i < t2_i {
        // this can happen when t1 and t2 are really close and the rounding moves them past each other
        0
    } else {
        t1_i - t2_i + 1
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &[Race]) -> u64 {
    input
        .iter()
        .map(|r| count_better_button_times(r.time, r.distance))
        .product()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Race]) -> u64 {
    fn merge_numbers(mut it: impl Iterator<Item = u64>) -> u64 {
        it.join("").parse().unwrap()
    }

    let time = merge_numbers(input.iter().map(|r| r.time));
    let distance = merge_numbers(input.iter().map(|r| r.distance));
    count_better_button_times(time, distance)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 288)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 71503)
    }
}
