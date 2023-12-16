use std::str::FromStr;

use crate::common::Vec2i;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Universe {
    galaxies: Vec<Vec2i>,
}

impl FromStr for Universe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Vec2i::new(x as _, y as _)),
                    _ => None,
                })
            })
            .collect();
        Ok(Universe { galaxies })
    }
}

fn distances(galaxies: &[Vec2i], expansion: i64) -> i64 {
    // the shortest distance between two points on a grid is the manhattan distance (L1 norm)
    // there is no need for bfs/dijkstra

    // when calculating the manhattan distance, the x- and y-distances can be derived separately
    // because they are independent

    // the naive idea to incorporate the space expansion is to find all the empty rows
    // (for the y distance) and the empty columns (for the x distance) between the start and
    // end galaxy and multiply them by the expansion factor
    // we can do better though - assuming the coordinates are sorted:
    // so we have a <= b as coordinates, the distance between them is d = b - a
    // because the coordinates are sorted there will be no other galaxy between a & b,
    // thus we can multiply d directly to get the expanded distance

    // finally we want to sum up those modified distances
    // we have to use those directly and not re-calculate them for non-neighboring galaxies
    // because they contain the space expansion already
    // additionally we want to avoid a quadratic algorithm for that:
    // the crux lies in the fact that the distance between x_i and x_j is equal to
    // d_ij = x_j - x_i = sum_{k=i}^{j-1} (x_{k+1} - x_k) (telescoping sum)
    // then we want to sum up those d_ij for all {i, j} <: 0..(n-1)
    // sum_{i=0}^{n-1} sum_{j=i+1}^{n-1} d_ij = sum_{i=0}^{n-1} sum_{j=i+1}^{n-1} sum_{k=i}^{j-1} (x_{k+1} - x_k)
    // but our goal is a single sum: sum_{i=0}^{n-1} X(n, i) * (x_{i+1} - x_i)
    // working out a few examples by hand and looking up the sequence in the OEIS gave the formula:
    // X(n, i) = (n - (i+1)) * (i+1)

    fn component_distances(it: impl IntoIterator<Item = i64>, n: i64, exp: i64) -> i64 {
        it.into_iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .map(|d| if d > 1 { d + (d - 1) * (exp - 1) } else { d })
            .enumerate()
            .map(|(i, d)| {
                // formula based on https://oeis.org/A003991
                let m = i as i64 + 1;
                (n - m) * m * d
            })
            .sum()
    }

    // the coordinates are already sorted by y, because we parse line by line
    let n = galaxies.len() as i64;
    let x_distances = component_distances(galaxies.iter().map(|g| g.x).sorted(), n, expansion);
    let y_distances = component_distances(galaxies.iter().map(|g| g.y), n, expansion);
    x_distances + y_distances
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Universe {
    input.parse().unwrap()
}

#[aoc(day11, part1)]
pub fn part1(input: &Universe) -> i64 {
    distances(&input.galaxies, 2)
}

#[aoc(day11, part2)]
pub fn part2(input: &Universe) -> i64 {
    distances(&input.galaxies, 1_000_000)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

    const INPUT_2: &str = include_str!("../alternative_inputs/day11.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 374);
    }

    #[test]
    fn test_part2_partial() {
        let input = input_generator(INPUT);
        assert_eq!(distances(&input.galaxies, 100), 8410);
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 2466269413);
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 155354715564293);
    }
}
