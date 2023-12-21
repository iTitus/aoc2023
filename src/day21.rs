use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashSet;

use crate::common::{Direction, Grid, Vec2i};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Tile {
    Empty,
    Obstacle,
    Start,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            'S' => Self::Start,
            _ => {
                return Err(());
            }
        })
    }
}

fn replace_start(pipes: &mut Grid<Tile>) -> Vec2i {
    let start = pipes
        .pos_iter()
        .find(|(_, tile)| **tile == Tile::Start)
        .unwrap()
        .0;
    pipes[start] = Tile::Empty;
    start
}

fn get_reachable(grid: &Grid<Tile>, start: Vec2i, steps: usize) -> usize {
    let mut current: FxHashSet<Vec2i> = FxHashSet::from_iter([start]);
    let mut next: FxHashSet<Vec2i> = FxHashSet::default();
    for _ in 0..steps {
        next.clear();
        next.extend(
            current
                .iter()
                .flat_map(|v| Direction::VALUES.iter().map(|d| d.offset(v)))
                .filter(|v| *grid.mod_get(v) != Tile::Obstacle),
        );
        std::mem::swap(&mut current, &mut next);
    }
    current.len()
}

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> (Vec2i, Grid<Tile>) {
    let mut grid = input.parse().unwrap();
    let start = replace_start(&mut grid);
    (start, grid)
}

#[aoc(day21, part1)]
pub fn part1((start, grid): &(Vec2i, Grid<Tile>)) -> usize {
    get_reachable(grid, *start, 64)
}

#[aoc(day21, part2)]
#[allow(clippy::erasing_op, clippy::identity_op)]
pub fn part2((start, grid): &(Vec2i, Grid<Tile>)) -> usize {
    const N: usize = 26501365;

    debug_assert!(grid.size_x == grid.size_y);
    let n = grid.size_x;
    let rest = N % n;

    let a = get_reachable(grid, *start, rest + 0 * n);
    let b = get_reachable(grid, *start, rest + 1 * n);
    let c = get_reachable(grid, *start, rest + 2 * n);

    // newton interpolation (quadratic polynomial)
    let c0 = a;
    let c1 = b - a;
    let double_c2 = (c - b) - c1;
    let target = N / n;
    c0 + c1 * (target - 0) + (double_c2 * (target - 0) * (target - 1)) / 2
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;

    #[test]
    fn test_part1() {
        let (start, grid) = input_generator(INPUT);
        assert_eq!(get_reachable(&grid, start, 6), 16);
    }
}
