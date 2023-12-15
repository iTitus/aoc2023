use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Almanac {
    initial: Vec<u32>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split_it = s.split("\n\n");

        let initial = split_it
            .next()
            .ok_or(())?
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse())
            .process_results(|it| it.collect())
            .map_err(|_| ())?;
        let maps = split_it
            .map(|m| m.parse())
            .process_results(|it| it.collect())
            .map_err(|_| ())?;

        Ok(Almanac { initial, maps })
    }
}

impl Almanac {
    fn convert(&self, input: u32) -> u32 {
        let mut n = input;
        for m in &self.maps {
            n = m.convert(n);
        }

        n
    }

    fn convert_multi(&self, ranges: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        let mut cur_ranges = ranges;
        for m in &self.maps {
            cur_ranges = m.convert_multi(cur_ranges);
        }

        cur_ranges
    }
}

#[derive(Debug)]
pub struct Map {
    entries: Vec<MapEntry>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .trim()
            .lines()
            .skip(1)
            .map(|l| l.parse())
            .process_results(|it| it.collect())
            .map_err(|_| ())?;

        Ok(Map { entries })
    }
}

impl Map {
    fn convert(&self, input: u32) -> u32 {
        self.entries
            .iter()
            .filter_map(|e| e.convert(input))
            .next()
            .unwrap_or(input)
    }

    fn convert_multi(&self, mut input: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
        // this can be improved by merging interval results when they overlap/touch
        // => similar to that cuboid puzzle (AoC 2021 day 22)
        let mut results = Vec::new();
        'outer: while let Some((start, len)) = input.pop() {
            if len == 0 {
                continue;
            }

            let end = start + len;
            for e in &self.entries {
                let source_end = e.source_start + e.range_length;
                // check for intersection (like in AoC 2022 day 4)
                if start < source_end && end > e.source_start {
                    // cutoff at the beginning of our interval
                    if start < e.source_start {
                        let start_cutoff = e.source_start - start;
                        input.push((start, start_cutoff));
                    }

                    // cutoff at the end of our interval
                    if end > source_end {
                        let end_cutoff = end - source_end;
                        input.push((source_end, end_cutoff));
                    }

                    // this is just the overlapping part in the middle
                    let overlap_start = start.max(e.source_start);
                    let overlap_end = end.min(source_end);
                    let overlap_len = overlap_end - overlap_start;
                    let offset = overlap_start - e.source_start;
                    results.push((e.destination_start + offset, overlap_len));
                    continue 'outer;
                }
            }

            // if no entries match we use the identity mapping
            results.push((start, len));
        }

        results
    }
}

#[derive(Debug)]
pub struct MapEntry {
    destination_start: u32,
    source_start: u32,
    range_length: u32,
}

impl FromStr for MapEntry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (destination_start, source_start, range_length) = s
            .split_whitespace()
            .map(|n| n.parse())
            .process_results(|it| it.collect_tuple())
            .map_err(|_| ())?
            .ok_or(())?;

        Ok(MapEntry {
            destination_start,
            source_start,
            range_length,
        })
    }
}

impl MapEntry {
    fn convert(&self, input: u32) -> Option<u32> {
        if input >= self.source_start {
            let offset = input - self.source_start;
            if offset < self.range_length {
                return Some(self.destination_start + offset);
            }
        }

        None
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Almanac {
    input.parse().unwrap()
}

#[aoc(day5, part1)]
pub fn part1(input: &Almanac) -> u32 {
    input
        .initial
        .iter()
        .map(|n| input.convert(*n))
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Almanac) -> u32 {
    let ranges = input
        .initial
        .iter()
        .tuples()
        .map(|(start, len)| (*start, *len))
        .collect();
    input
        .convert_multi(ranges)
        .iter()
        .map(|(start, _len)| *start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 46);
    }
}
