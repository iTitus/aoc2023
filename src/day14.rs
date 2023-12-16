use std::collections::HashMap;

use crate::common::{Direction, Grid, Vec2i};
use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Tile {
    Empty,
    Obstacle,
    Rock,
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            'O' => Self::Rock,
            _ => {
                return Err(());
            }
        })
    }
}

trait Platform {
    fn tilt(&self, dir: Direction) -> Self;

    fn cycle(&self, n: usize) -> Self;

    fn total_load(&self) -> usize;
}

impl Platform for Grid<Tile> {
    fn tilt(&self, dir: Direction) -> Grid<Tile> {
        let mut grid = self.clone();
        let mut swap = |pos: Vec2i, dir: Direction| {
            if grid[pos] == Tile::Rock {
                let mut target = pos;
                loop {
                    let new_target = dir.offset(&target);
                    if !grid.in_bounds(&new_target) || grid[new_target] != Tile::Empty {
                        break;
                    }

                    target = new_target;
                }

                if target != pos {
                    grid[pos] = Tile::Empty;
                    grid[target] = Tile::Rock;
                }
            }
        };

        match dir {
            Direction::North => {
                for y in 1..self.size_y {
                    for x in 0..self.size_x {
                        swap(Vec2i::new(x as _, y as _), dir);
                    }
                }
            }
            Direction::East => {
                for x in (0..self.size_x - 1).rev() {
                    for y in 0..self.size_y {
                        swap(Vec2i::new(x as _, y as _), dir);
                    }
                }
            }
            Direction::South => {
                for y in (0..self.size_y - 1).rev() {
                    for x in 0..self.size_x {
                        swap(Vec2i::new(x as _, y as _), dir);
                    }
                }
            }
            Direction::West => {
                for x in 1..self.size_x {
                    for y in 0..self.size_y {
                        swap(Vec2i::new(x as _, y as _), dir);
                    }
                }
            }
        };

        grid
    }

    fn cycle(&self, n: usize) -> Grid<Tile> {
        let mut current = self.clone();

        let mut cache: FxHashMap<Grid<Tile>, usize> = HashMap::default();
        let mut i = 0;
        loop {
            if i == n {
                return current;
            }

            let next = current
                .tilt(Direction::North)
                .tilt(Direction::West)
                .tilt(Direction::South)
                .tilt(Direction::East);
            i += 1;

            if let Some(prev_i) = cache.get(&next) {
                let cycle_length = i - prev_i;
                let how_many_cycles = (n - i) / cycle_length;
                i += how_many_cycles * cycle_length;
            } else {
                cache.insert(next.clone(), i);
            }

            current = next;
        }
    }

    fn total_load(&self) -> usize {
        self.pos_iter()
            .map(|(pos, t)| {
                if *t == Tile::Rock {
                    self.size_y - pos.y as usize
                } else {
                    0
                }
            })
            .sum()
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Grid<Tile> {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &Grid<Tile>) -> usize {
    input.tilt(Direction::North).total_load()
}

#[aoc(day14, part2)]
pub fn part2(input: &Grid<Tile>) -> usize {
    input.cycle(1_000_000_000).total_load()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 64);
    }
}
