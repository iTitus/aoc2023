use aoc_runner_derive::{aoc, aoc_generator};

use crate::common::{Grid, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ground {
    Dot,
    Hash,
}

impl TryFrom<char> for Ground {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Ground::Dot,
            '#' => Ground::Hash,
            _ => {
                return Err(());
            }
        })
    }
}

fn find_reflection(grid: &Grid<Ground>, smudges: usize) -> usize {
    'outer: for mirror_x in 1..grid.size_x {
        let mut smudges_found = 0;
        for y in 0..grid.size_y {
            for (x, mirrored_x) in (0..mirror_x).rev().zip(mirror_x..grid.size_x) {
                if grid[Vec2i::new(x as _, y as _)] != grid[Vec2i::new(mirrored_x as _, y as _)] {
                    smudges_found += 1;
                    if smudges_found > smudges {
                        continue 'outer;
                    }
                }
            }
        }

        if smudges_found == smudges {
            return mirror_x;
        }
    }

    'outer: for mirror_y in 1..grid.size_y {
        let mut smudges_found = 0;
        for x in 0..grid.size_x {
            for (y, mirrored_y) in (0..mirror_y).rev().zip(mirror_y..grid.size_y) {
                if grid[Vec2i::new(x as _, y as _)] != grid[Vec2i::new(x as _, mirrored_y as _)] {
                    smudges_found += 1;
                    if smudges_found > smudges {
                        continue 'outer;
                    }
                }
            }
        }

        if smudges_found == smudges {
            return 100 * mirror_y;
        }
    }

    unreachable!();
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Grid<Ground>> {
    input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Grid<Ground>]) -> usize {
    input.iter().map(|p| find_reflection(p, 0)).sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Grid<Ground>]) -> usize {
    input.iter().map(|p| find_reflection(p, 1)).sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 400);
    }
}
